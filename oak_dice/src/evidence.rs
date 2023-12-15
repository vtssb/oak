//
// Copyright 2023 The Project Oak Authors
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

//! C-like structs for representing DICE data in environments where we don't have protocol buffer
//! support.

use crate::utils::PaddedCopyFromSlice;
use alloc::{format, string::String, vec::Vec};
use strum::{Display, FromRepr};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

/// The maximum size of the signed attestation report.
pub const REPORT_SIZE: usize = 2048;

/// The size of the attestation report generated by AMD SEV-SNP.
///
/// See Table 21 in <https://www.amd.com/system/files/TechDocs/56860.pdf>.
const AMD_SEV_SNP_ATTESTATION_REPORT_SIZE: usize = 1184;

static_assertions::const_assert!(REPORT_SIZE >= AMD_SEV_SNP_ATTESTATION_REPORT_SIZE);

/// The maximum size of an ECDSA private key.
pub const PRIVATE_KEY_SIZE: usize = 64;

/// The actual size used when encoding a Nist P256 private key.
pub const P256_PRIVATE_KEY_SIZE: usize = 32;

/// The actual size used when encoding a X25519 private key.
pub const X25519_PRIVATE_KEY_SIZE: usize = 32;

/// The maximum size of a serialized COSE Key object representing an ECDSA public key.
pub const PUBLIC_KEY_SIZE: usize = 256;

/// The maximum size of a serialized CWT certificate.
pub const CERTIFICATE_SIZE: usize = 1024;

/// The name of the kernel command-line parameter that is used to send the physical address of the
/// Stage 0 DICE data struct.
pub const DICE_DATA_CMDLINE_PARAM: &str = "oak-dice";

/// The magic number used to identify the Stage 0 DICE data in memory.
pub const STAGE0_MAGIC: u64 = u64::from_le_bytes(*b"oak.dice");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Display, FromRepr)]
#[repr(u64)]
/// The hardware platform providing the Trusted Execution Environment.
pub enum TeePlatform {
    /// Platform is not specified.
    Unspecified = 0,
    /// AMD SEV-SNP.
    AmdSevSnp = 1,
    /// Intel TDX.
    IntelTdx = 2,
}

/// Attestation evidence generated by Stage 0.
#[derive(AsBytes, FromZeroes, FromBytes, Clone)]
#[repr(C)]
pub struct RootLayerEvidence {
    /// The hardware platform that generated the remote attestation report.
    pub tee_platform: u64,
    _padding: u64,
    /// Byte representation of the signed attestation report provided by the TEE platform.
    pub remote_attestation_report: [u8; REPORT_SIZE],
    /// Serialized COSE Key representation of the ECDSA public key that can be used to verify the
    /// signature of the next layer's certificate.
    ///
    /// The SHA2-256 digest of this field must included as additional data in the signed remote
    /// attestation report to bind the key to the report. This means that the attestation
    /// report effectively acts as a non-standard certificate for this key.
    pub eca_public_key: [u8; PUBLIC_KEY_SIZE],
}

impl RootLayerEvidence {
    pub fn get_tee_platform(&self) -> Result<TeePlatform, &'static str> {
        TeePlatform::from_repr(self.tee_platform).ok_or("invalid TEE Platform value")
    }

    pub fn get_remote_attestation_report(&self) -> Result<&[u8], &'static str> {
        match self.get_tee_platform()? {
            TeePlatform::AmdSevSnp => {
                Ok(&self.remote_attestation_report[..AMD_SEV_SNP_ATTESTATION_REPORT_SIZE])
            }
            _ => Ok(&self.remote_attestation_report),
        }
    }

    pub fn set_remote_attestation_report(&mut self, src: &[u8]) -> Result<(), &'static str> {
        self.remote_attestation_report.padded_copy_from_slice(src)
    }

    pub fn get_eca_public_key(&self) -> Result<Vec<u8>, String> {
        crate::utils::cbor_encoded_bytes_to_vec(&self.eca_public_key)
    }

    pub fn set_eca_public_key(&mut self, src: &[u8]) -> Result<(), &'static str> {
        self.eca_public_key.padded_copy_from_slice(src)
    }
}

static_assertions::assert_eq_size!([u8; 2320], RootLayerEvidence);

/// Attestation evidence for non-root layers.
#[derive(AsBytes, FromZeroes, FromBytes, Clone)]
#[repr(C)]
pub struct LayerEvidence {
    /// Serialized CWT certificate for the ECA private key owned by the corresponding layer. The
    /// certificate must include measurements of the layer that owns the private key.
    pub eca_certificate: [u8; CERTIFICATE_SIZE],
}

impl LayerEvidence {
    pub fn claims(&self) -> Result<coset::cwt::ClaimsSet, String> {
        let decoded = crate::utils::cbor_encoded_bytes_to_vec(&self.eca_certificate[..])?;
        crate::cert::get_claims_set_from_certificate_bytes(&decoded)
            .map_err(|err| format!("failed get claims: {:?}", err))
    }
}

static_assertions::assert_eq_size!([u8; CERTIFICATE_SIZE], LayerEvidence);

/// Private key that can be used by a layer to sign a certificate for the next layer.
#[derive(AsBytes, FromZeroes, FromBytes)]
#[repr(C)]
pub struct CertificateAuthority {
    /// The RAW bytes representing an ECDSA private key.
    pub eca_private_key: [u8; PRIVATE_KEY_SIZE],
}

static_assertions::assert_eq_size!([u8; PRIVATE_KEY_SIZE], CertificateAuthority);

impl Drop for CertificateAuthority {
    fn drop(&mut self) {
        // Zero out the ECA private key.
        self.eca_private_key.fill(0);
    }
}

/// Wrapper for passing DICE info from Stage0 to the next layer (Stage 1 or the Restricted Kernel).
#[derive(AsBytes, FromZeroes, FromBytes)]
#[repr(C, align(4096))]
pub struct Stage0DiceData {
    /// Magic number that is expected to always be set to the value of `STAGE0_MAGIC`.
    pub magic: u64,
    _padding_0: u64,
    /// The evidence about Stage 0 and the initial state of the VM.
    pub root_layer_evidence: RootLayerEvidence,
    /// The evidence about the next layer.
    pub layer_1_evidence: LayerEvidence,
    pub layer_1_certificate_authority: CertificateAuthority,
    _padding_1: [u8; 672],
}

static_assertions::assert_eq_size!([u8; 4096], Stage0DiceData);

/// Certificates for the ECDSA keys that can be used for an application for signing or encryption.
#[derive(AsBytes, FromZeroes, FromBytes, Clone)]
#[repr(C)]
pub struct ApplicationKeys {
    /// Serialized CWT certificate for the signing private key. The certificate must include
    /// measurements of the application.
    pub signing_public_key_certificate: [u8; CERTIFICATE_SIZE],
    /// Serialized CWT certificate for the encryption private key. The certificate must include
    /// measurements of the application.
    pub encryption_public_key_certificate: [u8; CERTIFICATE_SIZE],
}

impl ApplicationKeys {
    pub fn claims(&self) -> Result<coset::cwt::ClaimsSet, String> {
        let decoded =
            crate::utils::cbor_encoded_bytes_to_vec(&self.encryption_public_key_certificate[..])?;
        crate::cert::get_claims_set_from_certificate_bytes(&decoded)
            .map_err(|err| format!("failed get claims: {:?}", err))
    }
}

static_assertions::assert_eq_size!([u8; 2048], ApplicationKeys);

/// ECDSA private keys that can be used for an application for signing or encryption.
#[derive(AsBytes, FromZeroes, FromBytes)]
#[repr(C)]
pub struct ApplicationPrivateKeys {
    /// The RAW bytes representing an ECDSA private key that can be used to sign arbitrary data.
    pub signing_private_key: [u8; PRIVATE_KEY_SIZE],
    /// The RAW bytes representing an ECDSA private key that can be used for hybrid encryption.
    pub encryption_private_key: [u8; PRIVATE_KEY_SIZE],
}

static_assertions::assert_eq_size!([u8; 128], ApplicationPrivateKeys);

impl Drop for ApplicationPrivateKeys {
    fn drop(&mut self) {
        // Zero out the private keys.
        self.signing_private_key.fill(0);
        self.encryption_private_key.fill(0);
    }
}

/// Wrapper for passing the attestation evidence from the Restricted Kernel to the application.
#[derive(AsBytes, FromZeroes, FromBytes, Clone)]
#[repr(C)]
pub struct Evidence {
    /// Evidence about Stage 0 and the initial state of the VM.
    pub root_layer_evidence: RootLayerEvidence,
    /// The evidence about the Restricted Kernel.
    pub restricted_kernel_evidence: LayerEvidence,
    /// Keys (and associated certificates) that can be used by the application for encryption or
    /// signing.
    pub application_keys: ApplicationKeys,
}

static_assertions::assert_eq_size!([u8; 5392], Evidence);

/// Wrapper for passing the attestation evidence and private keys from the Restricted Kernel to the
/// application.
#[derive(AsBytes, FromZeroes, FromBytes)]
#[repr(C)]
pub struct RestrictedKernelDiceData {
    pub evidence: Evidence,
    pub application_private_keys: ApplicationPrivateKeys,
}

static_assertions::assert_eq_size!([u8; 5520], RestrictedKernelDiceData);
