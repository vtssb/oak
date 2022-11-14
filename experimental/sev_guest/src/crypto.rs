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

//! This module provides an optional implementation for encrypting and decrypting guest requests
//! using the RustCrypto `aes-gcm` crate.

use crate::guest::{GuestMessage, Message};
use aes_gcm::{
    aes::{
        cipher::{BlockSizeUser, Unsigned},
        Aes256,
    },
    AeadInPlace, Aes256Gcm, KeyInit, Nonce, Tag,
};
use core::mem::size_of;
use zerocopy::{AsBytes, FromBytes};

/// Wrapper for encrypting and decrypting guest messages.
///
/// The sequence number is used as an initialization vector/nonce for AES-GCM. The sequence number
/// is internally incremented on every encryption to avoid reuse. It is also incremented on every
/// decryption to synchronize it with the sequence number used by the Platform Secure Processor
/// (PSP).
///
/// To make sure that the same sequence number is never reused with the same key, the same key must
/// never be used to instantiate more than one instance of this struct.
///
/// If a request fails the PSP will not increment the sequence number. This means that we will be
/// out of sync with its sequence number. There is no safe way to recover from that, so the best
/// course of action is to delete the key or shut down the VM.
///
/// See Chapter 7 and Section 8.26.2 in <https://www.amd.com/system/files/TechDocs/56860.pdf>.
pub struct GuestMessageEncryptor {
    cipher: Aes256Gcm,
    sequence_number: u64,
}

impl GuestMessageEncryptor {
    /// Creates a new instance.
    ///
    /// For now we always assume that VMPCK_0 will be used as the key.
    pub fn new(key: &[u8]) -> Result<Self, &'static str> {
        Self::new_with_sequence_number(key, 0)
    }

    /// Creates a new instance with a specific sequence number.
    ///
    /// This function should only be used if the sequence number was incremented previously to a
    /// known value but can not be incremented further outside of this struct, e.g. use this in the
    /// kernel if the firmware used the same key to encrypt and decrypt guest messages.
    pub fn new_with_sequence_number(
        key: &[u8],
        initial_sequence_number: u64,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            cipher: Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid key length")?,
            sequence_number: initial_sequence_number,
        })
    }

    /// Creates an encrypted `GuestMessage` from the provided message.
    ///
    /// The sequence number is incremented automatically if the operation is successful.
    pub fn encrypt_message<M: AsBytes + Message>(
        &mut self,
        message: &M,
    ) -> Result<GuestMessage, &'static str> {
        let mut result = GuestMessage::new();
        let bytes = message.as_bytes();
        result.header.auth_header.message_type = M::get_message_type() as u8;
        let message_size = bytes.len();
        result.header.auth_header.message_size = message_size as u16;
        result.header.sequence_number = self.sequence_number + 1;
        let mut iv_bytes = [0u8; <Aes256 as BlockSizeUser>::BlockSize::USIZE];
        iv_bytes[0..size_of::<u64>()].copy_from_slice(result.header.sequence_number.as_bytes());
        let nonce = Nonce::from_slice(&iv_bytes[..]);
        let associated_data = result.header.auth_header.as_bytes();
        result.payload[0..message_size].copy_from_slice(bytes);
        let buffer = &mut result.payload[0..message_size];
        let auth_tag = self
            .cipher
            .encrypt_in_place_detached(nonce, associated_data, buffer)
            .map_err(|_| "Message encryption failed")?;
        result.header.auth_tag[0..auth_tag.len()].copy_from_slice(auth_tag.as_slice());

        self.sequence_number += 1;
        Ok(result)
    }

    /// Extracts a decrypted message from an encrypted `GuestMessage`.
    ///
    /// The sequence number is incremented automatically if the operation is successful.
    pub fn decrypt_message<M: AsBytes + FromBytes + Message>(
        &mut self,
        message: &GuestMessage,
    ) -> Result<M, &'static str> {
        let mut result = M::new_zeroed();
        message.validate()?;
        if M::get_message_type() as u8 != message.header.auth_header.message_type {
            return Err("Invalid message type");
        }
        let sequence_number = message.header.sequence_number;
        if sequence_number != self.sequence_number + 1 {
            return Err("Unexpected sequence numer");
        }
        let mut iv_bytes = [0u8; <Aes256 as BlockSizeUser>::BlockSize::USIZE];
        iv_bytes[0..size_of::<u64>()].copy_from_slice(sequence_number.as_bytes());
        let nonce = Nonce::from_slice(&iv_bytes[..]);
        let associated_data = message.header.auth_header.as_bytes();
        let buffer = result.as_bytes_mut();
        if buffer.len() != message.header.auth_header.message_size as usize {
            return Err("Invalid message length");
        }
        buffer.copy_from_slice(&message.payload[0..buffer.len()]);
        let tag = Tag::from_slice(&message.header.auth_tag[0..size_of::<Tag>()]);
        self.cipher
            .decrypt_in_place_detached(nonce, associated_data, buffer, tag)
            .map_err(|_| "Couldn't decrypt message")?;
        self.sequence_number += 1;
        Ok(result)
    }
}
