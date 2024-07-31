//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use alloc::boxed::Box;
use core::{
    alloc::{AllocError, Allocator, Layout},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use oak_core::sync::OnceCell;
use oak_sev_guest::{
    crypto::GuestMessageEncryptor,
    ghcb::GhcbProtocol,
    guest::{GuestMessage, Message},
    instructions::{pvalidate, InstructionError, PageSize as SevPageSize, Validation},
    msr::{change_snp_page_state, PageAssignment, SevStatus, SnpPageStateChangeRequest},
};
use spinning_top::{lock_api::MutexGuard, RawSpinlock, Spinlock};
use x86_64::{
    instructions::tlb,
    structures::paging::{Page, PageSize, PageTableFlags, Size2MiB, Size4KiB},
    PhysAddr, VirtAddr,
};
use zerocopy::{AsBytes, FromBytes};
use zeroize::Zeroize;

use crate::{sev_status, BootAllocator};

pub static GHCB_WRAPPER: Ghcb = Ghcb::new();

pub struct Ghcb {
    ghcb: OnceCell<Spinlock<GhcbProtocol<'static, oak_sev_guest::ghcb::Ghcb>>>,
}

impl Ghcb {
    const fn new() -> Self {
        Self { ghcb: OnceCell::new() }
    }

    pub fn init(&self, alloc: &'static BootAllocator) {
        let ghcb = Box::leak(Box::new_in(oak_sev_guest::ghcb::Ghcb::default(), alloc));
        let ghcb_addr = VirtAddr::from_ptr(ghcb);

        share_page(Page::containing_address(ghcb_addr));

        ghcb.reset();

        // We can't use `.expect()` here as Spinlock doesn't implement `fmt::Debug`.
        if self
            .ghcb
            .set(Spinlock::new(GhcbProtocol::new(ghcb, |vaddr: VirtAddr| {
                Some(PhysAddr::new(vaddr.as_u64()))
            })))
            .is_err()
        {
            panic!("couldn't initialize GHCB wrapper");
        }

        // SNP requires that the GHCB is registered with the hypervisor.
        if sev_status().contains(SevStatus::SNP_ACTIVE) {
            self.get()
                .unwrap()
                .register_with_hypervisor()
                .expect("couldn't register the GHCB address with the hypervisor");
        }
    }

    pub fn deinit(&self) {
        let ghcb_addr = VirtAddr::new(self.ghcb.get().unwrap().lock().get_gpa().as_u64());
        unshare_page(Page::containing_address(ghcb_addr));
    }

    pub fn get(
        &self,
    ) -> Option<MutexGuard<'_, RawSpinlock, GhcbProtocol<'static, oak_sev_guest::ghcb::Ghcb>>> {
        self.ghcb.get().map(|mutex| mutex.lock())
    }
}

/// Cryptographic helper to encrypt and decrypt messages for the GHCB guest
/// message protocol.
static GUEST_MESSAGE_ENCRYPTOR: Spinlock<Option<GuestMessageEncryptor>> = Spinlock::new(None);

/// Allocator that forces allocations to be 4K-aligned (and sized) and marks the
/// pages as shared.
///
/// This allocator is inefficient as it will only allocate 4K chunks,
/// potentially wasting memory. For example, if you allocate two u32-s, although
/// they could well fit on one page, currently that'd use 8K of memory.
/// That, however, is an implementation detail, and may change in the future.
#[repr(transparent)]
struct SharedAllocator<A: Allocator> {
    inner: A,
}

impl<A: Allocator> SharedAllocator<A> {
    fn new(allocator: A) -> Self {
        Self { inner: allocator }
    }
}

unsafe impl<A: Allocator> Allocator for SharedAllocator<A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let layout =
            layout.align_to(Size4KiB::SIZE as usize).map_err(|_| AllocError)?.pad_to_align();
        let allocation = self.inner.allocate(layout)?;
        if sev_status().contains(SevStatus::SEV_ENABLED) {
            for offset in (0..allocation.len()).step_by(Size4KiB::SIZE as usize) {
                // Safety: the allocation has succeeded and the offset won't exceed the size of
                // the allocation.
                share_page(Page::containing_address(VirtAddr::from_ptr(unsafe {
                    allocation.as_non_null_ptr().as_ptr().add(offset)
                })))
            }
        }
        Ok(allocation)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let layout = layout
            .align_to(Size4KiB::SIZE as usize)
            .map_err(|_| AllocError)
            .unwrap()
            .pad_to_align();
        if sev_status().contains(SevStatus::SEV_ENABLED) {
            for offset in (0..layout.size()).step_by(Size4KiB::SIZE as usize) {
                // Safety: the allocation has succeeded and the offset won't exceed the size of
                // the allocation.
                unshare_page(Page::containing_address(VirtAddr::from_ptr(unsafe {
                    ptr.as_ptr().add(offset)
                })))
            }
        }
        self.inner.deallocate(ptr, layout)
    }
}

/// Stores a data structure on a shared page.
pub struct Shared<T: 'static, A: Allocator> {
    inner: Box<T, SharedAllocator<A>>,
}

impl<T, A: Allocator> Shared<T, A> {
    pub fn new_in(t: T, alloc: A) -> Self
    where
        A: 'static,
    {
        Self { inner: Box::new_in(t, SharedAllocator::new(alloc)) }
    }
}

impl<T, A: Allocator> Deref for Shared<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, A: Allocator> DerefMut for Shared<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, A: Allocator> AsRef<T> for Shared<T, A> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T, A: Allocator> AsMut<T> for Shared<T, A> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// Shares a single 4KiB page with the hypervisor.
fn share_page(page: Page<Size4KiB>) {
    let page_start = page.start_address().as_u64();
    // Only the first 2MiB is mapped as 4KiB pages, so make sure we fall in that
    // range.
    assert!(page_start < Size2MiB::SIZE);
    // Remove the ENCRYPTED bit from the entry that maps the page.
    {
        let mut page_tables = crate::paging::PAGE_TABLE_REFS.get().unwrap().lock();
        let pt = &mut page_tables.pt_0;
        pt[page.p1_index()].set_addr(
            PhysAddr::new(page_start),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
    }
    tlb::flush_all();

    // SNP requires extra handling beyond just removing the encrypted bit.
    if sev_status().contains(SevStatus::SNP_ACTIVE) {
        let request = SnpPageStateChangeRequest::new(page_start as usize, PageAssignment::Shared)
            .expect("invalid address for page location");
        change_snp_page_state(request).expect("couldn't change SNP state for page");
    }
}

/// Stops sharing a single 4KiB page with the hypervisor when running with AMD
/// SEV-SNP enabled.
fn unshare_page(page: Page<Size4KiB>) {
    let page_start = page.start_address().as_u64();
    // Only the first 2MiB is mapped as 4KiB pages, so make sure we fall in that
    // range.
    assert!(page_start < Size2MiB::SIZE);
    if sev_status().contains(SevStatus::SNP_ACTIVE) {
        let request = SnpPageStateChangeRequest::new(page_start as usize, PageAssignment::Private)
            .expect("invalid address for page location");
        change_snp_page_state(request).expect("couldn't change SNP state for page");
    }
    // Mark the page as encrypted.
    {
        let mut page_tables = crate::paging::PAGE_TABLE_REFS.get().unwrap().lock();
        let pt = &mut page_tables.pt_0;
        pt[page.p1_index()].set_addr(
            PhysAddr::new(page_start | crate::encrypted()),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
    }
    tlb::flush_all();
    // We have to revalidate the page again after un-sharing it.
    if let Err(err) = pvalidate(page_start as usize, SevPageSize::Page4KiB, Validation::Validated) {
        if err != InstructionError::ValidationStatusNotUpdated {
            panic!("shared page revalidation failed");
        }
    }
}

/// Initializes the Guest Message encryptor using VMPCK0.
pub fn init_guest_message_encryptor() -> Result<(), &'static str> {
    // Safety: `SecretsPage` implements `FromBytes` which ensures that it has no
    // requirements on the underlying bytes.
    let key = &mut unsafe { crate::SEV_SECRETS.assume_init_mut() }.vmpck_0[..];
    GUEST_MESSAGE_ENCRYPTOR.lock().replace(GuestMessageEncryptor::new(key)?);
    // Once the we have read VMPCK0 we wipe it so that later boot stages cannot
    // request attestation reports or derived sealing keys for VMPL0. This stops
    // later boot stages from creating counterfeit DICE chains.
    key.zeroize();
    // The sev-guest driver in the upstream kernel does not initialize with such
    // an empty vmpck. So we fill it up with 0xFF.
    key.fill(0xFF);
    Ok(())
}

/// Sends a request to the Secure Processor using the Guest Message Protocol.
pub fn send_guest_message_request<
    Request: AsBytes + FromBytes + Message,
    Response: AsBytes + FromBytes + Message,
>(
    request: Request,
) -> Result<Response, &'static str> {
    let mut guard = GUEST_MESSAGE_ENCRYPTOR.lock();
    let encryptor = guard.as_mut().ok_or("guest message encryptor is not initialized")?;
    let alloc = &crate::SHORT_TERM_ALLOC;
    let mut request_message = Shared::new_in(GuestMessage::new(), alloc);
    encryptor.encrypt_message(request, request_message.as_mut())?;
    let response_message = Shared::new_in(GuestMessage::new(), alloc);

    let request_address =
        PhysAddr::new(VirtAddr::from_ptr(request_message.as_ref() as *const GuestMessage).as_u64());
    let response_address = PhysAddr::new(
        VirtAddr::from_ptr(response_message.as_ref() as *const GuestMessage).as_u64(),
    );

    GHCB_WRAPPER
        .get()
        .ok_or("GHCB not initialized")?
        .do_guest_message_request(request_address, response_address)?;
    response_message.validate()?;
    encryptor.decrypt_message::<Response>(response_message.as_ref())
}
