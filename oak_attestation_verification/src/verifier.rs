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

//! Provides verification based on evidence, endorsements and reference values.

use alloc::{format, string::String, vec::Vec};

use anyhow::Context;
use coset::{cbor::Value, cwt::ClaimsSet, CborSerializable, CoseKey, RegisteredLabelWithPrivate};
use ecdsa::{signature::Verifier, Signature};
use oak_dice::cert::{
    cose_key_to_hpke_public_key, cose_key_to_verifying_key, get_public_key_from_claims_set,
    ACPI_MEASUREMENT_ID, APPLICATION_KEY_ID, CONTAINER_IMAGE_LAYER_ID,
    ENCLAVE_APPLICATION_LAYER_ID, EVENT_ID, FINAL_LAYER_CONFIG_MEASUREMENT_ID,
    INITRD_MEASUREMENT_ID, KERNEL_COMMANDLINE_ID, KERNEL_COMMANDLINE_MEASUREMENT_ID,
    KERNEL_LAYER_ID, KERNEL_MEASUREMENT_ID, LAYER_2_CODE_MEASUREMENT_ID,
    LAYER_3_CODE_MEASUREMENT_ID, MEMORY_MAP_MEASUREMENT_ID, SETUP_DATA_MEASUREMENT_ID, SHA2_256_ID,
    SYSTEM_IMAGE_LAYER_ID,
};
use oak_proto_rust::oak::{
    attestation::v1::{
        attestation_results::Status, binary_reference_value, endorsements, expected_digests,
        expected_values, extracted_evidence::EvidenceValues, kernel_binary_reference_value,
        reference_values, root_layer_data::Report, text_expected_value, text_reference_value,
        AmdAttestationReport, AmdSevExpectedValues, ApplicationKeys, ApplicationLayerData,
        ApplicationLayerEndorsements, ApplicationLayerExpectedValues,
        ApplicationLayerReferenceValues, AttestationResults, BinaryReferenceValue, CbData,
        CbEndorsements, CbExpectedValues, CbReferenceValues, ContainerLayerData,
        ContainerLayerEndorsements, ContainerLayerExpectedValues, ContainerLayerReferenceValues,
        EndorsementReferenceValue, Endorsements, Event, EventData, EventExpectedValues, EventLog,
        EventReferenceValues, Evidence, ExpectedDigests, ExpectedRegex, ExpectedStringLiterals,
        ExpectedValues, ExtractedEvidence, FakeAttestationReport, FirmwareAttachment,
        InsecureExpectedValues, IntelTdxAttestationReport, IntelTdxExpectedValues,
        KernelAttachment, KernelBinaryReferenceValue, KernelExpectedValues, KernelLayerData,
        KernelLayerEndorsements, KernelLayerExpectedValues, KernelLayerReferenceValues,
        LayerEvidence, OakContainersData, OakContainersEndorsements, OakContainersExpectedValues,
        OakContainersReferenceValues, OakRestrictedKernelData, OakRestrictedKernelEndorsements,
        OakRestrictedKernelExpectedValues, OakRestrictedKernelReferenceValues,
        OrchestratorMeasurements, RawDigests, ReferenceValues, RootLayerData,
        RootLayerEndorsements, RootLayerEvidence, RootLayerExpectedValues,
        RootLayerReferenceValues, Stage0Measurements, Stage1Measurements, SystemLayerData,
        SystemLayerEndorsements, SystemLayerExpectedValues, SystemLayerReferenceValues, TcbVersion,
        TeePlatform, TextExpectedValue, TextReferenceValue, TransparentReleaseEndorsement,
        VerificationSkipped,
    },
    RawDigest,
};
use oak_sev_snp_attestation_report::AttestationReport;
use prost::Message;
#[cfg(feature = "regex")]
use regex_lite::Regex;
use sha2::Digest;
use x509_cert::{
    der::{Decode, DecodePem},
    Certificate,
};
use zerocopy::FromBytes;

use crate::{
    amd::{product_name, verify_attestation_report_signature, verify_cert_signature},
    claims::{get_digest, parse_endorsement_statement},
    endorsement::verify_binary_endorsement,
    util::{
        hash_sha2_256, hex_to_raw_digest, is_hex_digest_match, is_raw_digest_match,
        raw_digest_from_contents, raw_to_hex_digest,
    },
};

// We don't use additional authenticated data.
const ADDITIONAL_DATA: &[u8] = b"";

pub fn to_attestation_results(
    verify_result: &anyhow::Result<ExtractedEvidence>,
) -> AttestationResults {
    match verify_result {
        #[allow(deprecated)]
        Ok(extracted_evidence) => AttestationResults {
            status: Status::Success.into(),
            encryption_public_key: extracted_evidence.encryption_public_key.clone(),
            signing_public_key: extracted_evidence.signing_public_key.clone(),
            extracted_evidence: Some(extracted_evidence.clone()),
            ..Default::default()
        },
        Err(err) => AttestationResults {
            status: Status::GenericFailure.into(),
            reason: format!("{:#?}", err),
            ..Default::default()
        },
    }
}

/// Verifies entire setup by forwarding to individual setup types.
/// The `now_utc_millis` argument will be changed to a time type as work
/// progresses.
///
/// This just fetches expected values using [get_expected_values], and then
/// calls [verify_with_expected_values] with those.
///
/// If you'd like to cache and reuse the values, call those two methods
/// indepedently, and cache the results of the first.
pub fn verify(
    now_utc_millis: i64,
    evidence: &Evidence,
    endorsements: &Endorsements,
    reference_values: &ReferenceValues,
) -> anyhow::Result<ExtractedEvidence> {
    let expected_values = get_expected_values(now_utc_millis, endorsements, reference_values)
        .context("getting expected values")?;

    verify_with_expected_values(now_utc_millis, evidence, endorsements, &expected_values)
}

/// Verifies entire setup by forwarding to individual setup types.
/// The `now_utc_millis` argument will be changed to a time type as work
/// progresses.
/// This variant returns expected values along with the extracted evidence.
pub fn verify_with_expected_values(
    now_utc_millis: i64,
    evidence: &Evidence,
    endorsements: &Endorsements,
    expected_values: &ExpectedValues,
) -> anyhow::Result<ExtractedEvidence> {
    // Ensure the Attestation report is properly signed by the platform and that it
    // includes the root public key used in the DICE chain.
    {
        let tee_certificate: &[u8] =
            match endorsements.r#type.as_ref().context("no endorsements")? {
                endorsements::Type::OakRestrictedKernel(endorsements) => endorsements
                    .root_layer
                    .as_ref()
                    .context("no root layer endorsements")?
                    .tee_certificate
                    .as_ref(),
                endorsements::Type::OakContainers(endorsements) => endorsements
                    .root_layer
                    .as_ref()
                    .context("no root layer endorsements")?
                    .tee_certificate
                    .as_ref(),
                endorsements::Type::Cb(endorsements) => endorsements
                    .root_layer
                    .as_ref()
                    .context("no root layer endorsements")?
                    .tee_certificate
                    .as_ref(),
                endorsements::Type::Standalone(_) => &[],
            };
        let root_layer = evidence.root_layer.as_ref().context("no root layer evidence")?;
        verify_root_attestation_signature(now_utc_millis, root_layer, tee_certificate)
            .context("verifying root signature")?;
    };

    // Ensure the DICE chain signatures are valid and extract the measurements,
    // public keys and other attestation-related data from the DICE chain.
    let extracted_evidence = verify_dice_chain(evidence).context("invalid DICE chain")?;

    compare_expected_values(&extracted_evidence, expected_values)
        .context("comparing expected values to evidence")?;

    Ok(extracted_evidence)
}

// Create the set of [ExpectedValues] for the provided [endorsements] and
// [reference_values]. These can be cached by the client for as long as the
// validity time provided.
pub fn get_expected_values(
    now_utc_millis: i64,
    endorsements: &Endorsements,
    reference_values: &ReferenceValues,
) -> anyhow::Result<ExpectedValues> {
    match (endorsements.r#type.as_ref(), reference_values.r#type.as_ref()) {
        (
            Some(endorsements::Type::OakRestrictedKernel(ends)),
            Some(reference_values::Type::OakRestrictedKernel(rvs)),
        ) => {
            let expected = get_oak_restricted_kernel_expected_values(now_utc_millis, ends, rvs)
                .context("getting RK expected values")?;
            Ok(ExpectedValues {
                r#type: Some(expected_values::Type::OakRestrictedKernel(expected)),
            })
        }
        (
            Some(endorsements::Type::OakContainers(ends)),
            Some(reference_values::Type::OakContainers(rvs)),
        ) => {
            let expected = get_oak_containers_expected_values(now_utc_millis, ends, rvs)
                .context("getting containers expected values")?;
            Ok(ExpectedValues { r#type: Some(expected_values::Type::OakContainers(expected)) })
        }
        (Some(endorsements::Type::Cb(ends)), Some(reference_values::Type::Cb(rvs))) => {
            let expected = get_cb_expected_values(now_utc_millis, ends, rvs)
                .context("getting CB expected values")?;
            Ok(ExpectedValues { r#type: Some(expected_values::Type::Cb(expected)) })
        }
        // Evidence, endorsements and reference values must exist and reflect the same chain type.
        (None, _) => anyhow::bail!("Endorsements are empty"),
        (_, None) => anyhow::bail!("Reference values are empty"),
        (Some(_), Some(_)) => {
            anyhow::bail!("Mismatch between evidence, endorsements and reference values")
        }
    }
}

fn compare_expected_values(
    extracted_evidence: &ExtractedEvidence,
    expected_values: &ExpectedValues,
) -> anyhow::Result<()> {
    match (extracted_evidence.evidence_values.as_ref(), expected_values.r#type.as_ref()) {
        (
            Some(EvidenceValues::OakRestrictedKernel(values)),
            Some(expected_values::Type::OakRestrictedKernel(expected)),
        ) => compare_oak_restricted_kernel_measurement_digests(values, expected),
        (
            Some(EvidenceValues::OakContainers(values)),
            Some(expected_values::Type::OakContainers(expected)),
        ) => compare_oak_containers_measurement_digests(values, expected),
        (Some(EvidenceValues::Cb(values)), Some(expected_values::Type::Cb(expected))) => {
            compare_cb_measurement_digests(values, expected)
        }
        // Evidence, endorsements and reference values must exist and reflect the same chain type.
        (None, _) => anyhow::bail!("Reference values are empty"),
        (_, None) => anyhow::bail!("Expected values are empty"),
        (Some(_), Some(_)) => {
            anyhow::bail!("Mismatch between evidence, endorsements and reference values")
        }
    }
}

/// Verifies signatures of the certificates in the DICE chain and extracts the
/// evidence values from the certificates if the verification is successful.
pub fn verify_dice_chain(evidence: &Evidence) -> anyhow::Result<ExtractedEvidence> {
    let root_layer_verifying_key = {
        let cose_key = {
            let root_layer = evidence
                .root_layer
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("no root layer evidence"))?;
            CoseKey::from_slice(&root_layer.eca_public_key).map_err(|_cose_err| {
                anyhow::anyhow!("couldn't deserialize root layer public key")
            })?
        };
        cose_key_to_verifying_key(&cose_key).map_err(|msg| anyhow::anyhow!(msg))?
    };

    // Sequentially verify the layers, eventually retrieving the verifying key of
    // the last layer.
    let last_layer_verifying_key = evidence
        .layers
        .iter()
        .try_fold(root_layer_verifying_key, |previous_layer_verifying_key, current_layer| {
            let cert = coset::CoseSign1::from_slice(&current_layer.eca_certificate)
                .map_err(|_cose_err| anyhow::anyhow!("could not parse certificate"))?;
            cert.verify_signature(ADDITIONAL_DATA, |signature, contents| {
                let sig = Signature::from_slice(signature)?;
                previous_layer_verifying_key.verify(contents, &sig)
            })
            .map_err(|error| anyhow::anyhow!(error))?;
            let payload = cert.payload.ok_or_else(|| anyhow::anyhow!("no cert payload"))?;
            let claims = ClaimsSet::from_slice(&payload)
                .map_err(|_cose_err| anyhow::anyhow!("could not parse claims set"))?;
            let cose_key = get_public_key_from_claims_set(&claims)
                .map_err(|msg| anyhow::anyhow!(msg))
                .context("getting pk from claims")?;
            cose_key_to_verifying_key(&cose_key)
                .map_err(|msg| anyhow::anyhow!(msg))
                .context("converting cose key")
        })
        .context("getting last layer key")?;

    // Finally, use the last layer's verification key to verify the application
    // keys.
    {
        let appl_keys = evidence
            .application_keys
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no application keys in evidence"))?;

        // Verify encryption certificate.
        let encryption_cert =
            coset::CoseSign1::from_slice(&appl_keys.encryption_public_key_certificate)
                .map_err(|_cose_err| anyhow::anyhow!("could not parse encryption certificate"))?;
        encryption_cert
            .verify_signature(ADDITIONAL_DATA, |signature, contents| {
                let sig = Signature::from_slice(signature)?;
                last_layer_verifying_key.verify(contents, &sig)
            })
            .map_err(|error| anyhow::anyhow!(error))
            .context("failed to verify CWT signature")?;

        // Verify signing certificate.
        let signing_cert = coset::CoseSign1::from_slice(&appl_keys.signing_public_key_certificate)
            .map_err(|_cose_err| anyhow::anyhow!("could not parse encryption certificate"))?;
        signing_cert
            .verify_signature(ADDITIONAL_DATA, |signature, contents| {
                let sig = Signature::from_slice(signature)?;
                last_layer_verifying_key.verify(contents, &sig)
            })
            .map_err(|error| anyhow::anyhow!(error))?;
    }

    // Verify the event log claim for this layer if it exists. This is done for all
    // layers here, since the event log is tied uniquely closely to the DICE chain.
    if let Some(event_log) = &evidence.event_log {
        validate_that_event_log_is_captured_in_dice_layers(event_log, &evidence.layers)
            .context("events in log do not match the digests in the dice chain")?
    }
    extract_evidence(evidence)
}

/// Validates that the digest of the events captured in the event log are
/// correctly described in the claims of the associated dice layers.
fn validate_that_event_log_is_captured_in_dice_layers(
    event_log: &EventLog,
    dice_layers: &Vec<LayerEvidence>,
) -> anyhow::Result<()> {
    dice_layers.iter().zip(event_log.encoded_events.iter()).try_for_each(
        |(current_layer, encoded_event)| {
            let event_digest = {
                let claims = claims_set_from_serialized_cert(&current_layer.eca_certificate)
                    .map_err(|_cose_err| anyhow::anyhow!("could not parse claims set"))?;
                extract_event_data(&claims)
                    .context("couldn't extract event claim")?
                    .event
                    .context("Missing event")?
            };
            let actual_event_hash = &<sha2::Sha256 as sha2::Digest>::digest(encoded_event).to_vec();
            return if actual_event_hash != &event_digest.sha2_256 {
                Err(anyhow::anyhow!("event log hash mismatch"))
            } else {
                Ok(())
            };
        },
    )
}

fn get_oak_restricted_kernel_expected_values(
    now_utc_millis: i64,
    endorsements: &OakRestrictedKernelEndorsements,
    reference_values: &OakRestrictedKernelReferenceValues,
) -> anyhow::Result<OakRestrictedKernelExpectedValues> {
    Ok(OakRestrictedKernelExpectedValues {
        root_layer: Some(get_root_layer_expected_values(
            now_utc_millis,
            endorsements.root_layer.as_ref(),
            reference_values.root_layer.as_ref().context("no root layer reference values")?,
        )?),
        kernel_layer: Some(get_kernel_layer_expected_values(
            now_utc_millis,
            endorsements.kernel_layer.as_ref(),
            reference_values.kernel_layer.as_ref().context("no kernel layer reference values")?,
        )?),
        application_layer: Some(get_application_layer_expected_values(
            now_utc_millis,
            endorsements.application_layer.as_ref(),
            reference_values
                .application_layer
                .as_ref()
                .context("no application layer reference values")?,
        )?),
    })
}

/// Validates the values extracted from the evidence against the reference
/// values and endorsements for Oak Restricted Kernel applications.
fn compare_oak_restricted_kernel_measurement_digests(
    values: &OakRestrictedKernelData,
    expected: &OakRestrictedKernelExpectedValues,
) -> anyhow::Result<()> {
    compare_root_layer_measurement_digests(
        values.root_layer.as_ref().context("no root layer evidence values")?,
        expected.root_layer.as_ref().context("no root layer expected values")?,
    )
    .context("comparing root layer values")?;

    compare_kernel_layer_measurement_digests(
        values.kernel_layer.as_ref().context("no kernel layer evidence values")?,
        expected.kernel_layer.as_ref().context("no kernel layer expected values")?,
    )
    .context("comparing kernel layer values")?;

    compare_application_layer_measurement_digests(
        values.application_layer.as_ref().context("no applications layer evidence values")?,
        expected.application_layer.as_ref().context("no application layer expected values")?,
    )
    .context("comparing application layer values")
}

fn get_oak_containers_expected_values(
    now_utc_millis: i64,
    endorsements: &OakContainersEndorsements,
    reference_values: &OakContainersReferenceValues,
) -> anyhow::Result<OakContainersExpectedValues> {
    Ok(OakContainersExpectedValues {
        root_layer: Some(
            get_root_layer_expected_values(
                now_utc_millis,
                endorsements.root_layer.as_ref(),
                reference_values.root_layer.as_ref().context("no root layer reference values")?,
            )
            .context("getting root layer values")?,
        ),
        kernel_layer: Some(
            get_kernel_layer_expected_values(
                now_utc_millis,
                endorsements.kernel_layer.as_ref(),
                reference_values
                    .kernel_layer
                    .as_ref()
                    .context("no kernel layer reference values")?,
            )
            .context("getting kernel layer values")?,
        ),
        system_layer: Some(
            get_system_layer_expected_values(
                now_utc_millis,
                endorsements.system_layer.as_ref(),
                reference_values
                    .system_layer
                    .as_ref()
                    .context("no application layer reference values")?,
            )
            .context("getting system layer values")?,
        ),
        container_layer: Some(
            get_container_layer_expected_values(
                now_utc_millis,
                endorsements.container_layer.as_ref(),
                reference_values
                    .container_layer
                    .as_ref()
                    .context("no kernel layer reference values")?,
            )
            .context("getting container layer values")?,
        ),
    })
}

/// Validates the values extracted from the evidence against the reference
/// values and endorsements for Oak Restricted Containers applications.
fn compare_oak_containers_measurement_digests(
    values: &OakContainersData,
    expected: &OakContainersExpectedValues,
) -> anyhow::Result<()> {
    compare_root_layer_measurement_digests(
        values.root_layer.as_ref().context("no root layer evidence values")?,
        expected.root_layer.as_ref().context("no root layer expected values")?,
    )
    .context("comparing root layer digests")?;

    compare_kernel_layer_measurement_digests(
        values.kernel_layer.as_ref().context("no kernel layer evidence values")?,
        expected.kernel_layer.as_ref().context("no kernel layer expected_values")?,
    )
    .context("comparing kernel layer digests")?;

    compare_system_layer_measurement_digests(
        values.system_layer.as_ref().context("no system layer evidence values")?,
        expected.system_layer.as_ref().context("no system layer expected_values")?,
    )
    .context("comparing system layer digests")?;

    compare_container_layer_measurement_digests(
        values.container_layer.as_ref().context("no container layer evidence values")?,
        expected.container_layer.as_ref().context("no system layer expected_values")?,
    )
    .context("comparing container layer digests")
}

fn get_cb_expected_values(
    now_utc_millis: i64,
    endorsements: &CbEndorsements,
    reference_values: &CbReferenceValues,
) -> anyhow::Result<CbExpectedValues> {
    Ok(CbExpectedValues {
        root_layer: Some(
            get_root_layer_expected_values(
                now_utc_millis,
                endorsements.root_layer.as_ref(),
                reference_values.root_layer.as_ref().context("no root layer reference values")?,
            )
            .context("getting root layer values")?,
        ),
        kernel_layer: Some(
            get_event_expected_values(
                now_utc_millis,
                reference_values
                    .kernel_layer
                    .as_ref()
                    .context("no kernel layer reference values")?,
            )
            .context("getting kernel layer values")?,
        ),
        system_layer: Some(
            get_event_expected_values(
                now_utc_millis,
                reference_values
                    .system_layer
                    .as_ref()
                    .context("no system layer reference values")?,
            )
            .context("getting system layer values")?,
        ),
        application_layer: Some(
            get_event_expected_values(
                now_utc_millis,
                reference_values
                    .application_layer
                    .as_ref()
                    .context("no application layer reference values")?,
            )
            .context("getting application layer values")?,
        ),
    })
}

/// Validates the values extracted from the evidence against the reference
/// values and endorsements for CB workloads.
fn compare_cb_measurement_digests(
    values: &CbData,
    expected: &CbExpectedValues,
) -> anyhow::Result<()> {
    compare_root_layer_measurement_digests(
        values.root_layer.as_ref().context("no root layer evidence values")?,
        expected.root_layer.as_ref().context("no root layer expected values")?,
    )
    .context("root layer verification failed")?;

    compare_event_measurement_digests(
        values.kernel_layer.as_ref().context("no kernel layer evidence values")?,
        expected.kernel_layer.as_ref().context("no kernel layer expected_values")?,
    )
    .context("kernel layer verification failed")?;

    compare_event_measurement_digests(
        values.system_layer.as_ref().context("no system layer evidence values")?,
        expected.system_layer.as_ref().context("no system layer expected_values")?,
    )
    .context("system layer verification failed")?;

    compare_event_measurement_digests(
        values.application_layer.as_ref().context("no application layer evidence values")?,
        expected.application_layer.as_ref().context("no application layer expected_values")?,
    )
    .context("application layer verification failed")
}

/// Verifies the AMD SEV attestation report.
fn verify_amd_sev_attestation_report(
    attestation_report_values: &AmdAttestationReport,
    expected_values: &AmdSevExpectedValues,
) -> anyhow::Result<()> {
    // Stage 0 only destroys VMPCK0, so we only trust attestation reports that were
    // generated in VMPL0.
    anyhow::ensure!(
        attestation_report_values.vmpl == 0,
        "attestation report was not generated from VMPL {}, not VMPL 0",
        attestation_report_values.vmpl
    );

    if !expected_values.allow_debug && attestation_report_values.debug {
        anyhow::bail!("debug mode not allowed");
    }

    match (
        expected_values.min_tcb_version.as_ref(),
        attestation_report_values.reported_tcb.as_ref(),
    ) {
        (Some(min_tcb_version), Some(reported_tcb_version)) => {
            anyhow::ensure!(
                reported_tcb_version.boot_loader >= min_tcb_version.boot_loader,
                format!(
                    "unsupported boot loader version in the reported TCB: {}",
                    reported_tcb_version.boot_loader
                )
            );
            anyhow::ensure!(
                reported_tcb_version.tee >= min_tcb_version.tee,
                format!(
                    "unsupported tee version in the reported TCB: {}",
                    reported_tcb_version.tee
                )
            );
            anyhow::ensure!(
                reported_tcb_version.snp >= min_tcb_version.snp,
                format!(
                    "unsupported snp version in the reported TCB: {}",
                    reported_tcb_version.snp
                )
            );
            anyhow::ensure!(
                reported_tcb_version.microcode >= min_tcb_version.microcode,
                format!(
                    "unsupported microcode version in the reported TCB: {}",
                    reported_tcb_version.microcode
                )
            );
        }
        (Some(_), None) => anyhow::bail!("no reported TCB version in the attestation report"),
        (None, _) => anyhow::bail!("no min TCB version reference value"),
    }

    Ok(())
}

/// Verifies the Intel TDX attestation report.
fn verify_intel_tdx_attestation_report(
    _attestation_report_values: &IntelTdxAttestationReport,
    _expected_values: &IntelTdxExpectedValues,
) -> anyhow::Result<()> {
    anyhow::bail!("needs implementation")
}

/// Verifies insecure attestation.
fn verify_insecure(_expected_values: &InsecureExpectedValues) -> anyhow::Result<()> {
    Ok(())
}

const ASK_MILAN_CERT_PEM: &str = include_str!("../data/ask_milan.pem");
const ASK_GENOA_CERT_PEM: &str = include_str!("../data/ask_genoa.pem");

/// Verifies the signature chain for the attestation report included in the
/// root.
fn verify_root_attestation_signature(
    _now_utc_millis: i64,
    root_layer: &RootLayerEvidence,
    serialized_certificate: &[u8],
) -> anyhow::Result<()> {
    match root_layer.platform() {
        TeePlatform::Unspecified => anyhow::bail!("unspecified TEE platform"),
        TeePlatform::AmdSevSnp => {
            // We demand that product-specific ASK signs the VCEK.
            let vcek = Certificate::from_der(serialized_certificate)
                .map_err(|_err| anyhow::anyhow!("could not parse VCEK cert"))?;

            let ask = if product_name(&vcek)?.contains("Milan") {
                Certificate::from_pem(ASK_MILAN_CERT_PEM)
                    .map_err(|_err| anyhow::anyhow!("could not parse Milan ASK cert"))?
            } else if product_name(&vcek)?.contains("Genoa") {
                Certificate::from_pem(ASK_GENOA_CERT_PEM)
                    .map_err(|_err| anyhow::anyhow!("could not parse Genoa ASK cert"))?
            } else {
                anyhow::bail!("unsupported AMD product");
            };

            // TODO(#4747): user current date as part of VCEK verification.
            verify_cert_signature(&ask, &vcek).context("verifying vcek cert")?;

            let report = AttestationReport::ref_from(&root_layer.remote_attestation_report)
                .context("invalid AMD SEV-SNP attestation report")?;
            report.validate().map_err(|msg| anyhow::anyhow!(msg))?;

            // Ensure that the attestation report is signed by the VCEK public key.
            verify_attestation_report_signature(&vcek, report)
                .context("verifying attestation report signature")?;

            // Check that the root ECA public key for the DICE chain is bound to the
            // attestation report to ensure that the entire chain is valid.
            let expected = &hash_sha2_256(&root_layer.eca_public_key[..])[..];
            let actual = report.data.report_data;

            anyhow::ensure!(
                // The report data contains 64 bytes by default, but we only use the first 32 bytes
                // at the moment.
                expected.len() < actual.len() && expected == &actual[..expected.len()],
                "The root layer's ECA public key is not bound to the attestation report"
            );

            Ok(())
        }
        TeePlatform::IntelTdx => anyhow::bail!("not supported"),
        TeePlatform::None => Ok(()),
    }
}

fn get_root_layer_expected_values(
    now_utc_millis: i64,
    endorsements: Option<&RootLayerEndorsements>,
    reference_values: &RootLayerReferenceValues,
) -> anyhow::Result<RootLayerExpectedValues> {
    // Propagate each of the existing reference value for a TEE platform to the
    // corresponding expected value.

    let amd_sev = if let Some(amd_sev_values) = reference_values.amd_sev.as_ref() {
        let stage0_expected = get_stage0_expected_values(
            now_utc_millis,
            endorsements.and_then(|value| value.stage0.as_ref()),
            amd_sev_values.stage0.as_ref().context("stage0 binary reference values not found")?,
        )
        .context("getting stage0 expected values")?;
        Some(AmdSevExpectedValues {
            stage0_expected: Some(stage0_expected),
            min_tcb_version: amd_sev_values.min_tcb_version.clone(),
            allow_debug: amd_sev_values.allow_debug,
        })
    } else {
        None
    };

    let intel_tdx = reference_values.intel_tdx.as_ref().map(|_| IntelTdxExpectedValues {});
    let insecure = reference_values.insecure.as_ref().map(|_| InsecureExpectedValues {});

    Ok(RootLayerExpectedValues { amd_sev, intel_tdx, insecure })
}

fn compare_root_layer_measurement_digests(
    values: &RootLayerData,
    expected_values: &RootLayerExpectedValues,
) -> anyhow::Result<()> {
    match (
        values.report.as_ref(),
        expected_values.amd_sev.as_ref(),
        expected_values.intel_tdx.as_ref(),
        expected_values.insecure.as_ref(),
    ) {
        (Some(Report::SevSnp(report_values)), Some(amd_sev_values), _, _) => {
            let measurement = RawDigest {
                sha2_384: report_values.initial_measurement.to_vec(),
                ..Default::default()
            };
            compare_measurement_digest(
                &measurement,
                amd_sev_values
                    .stage0_expected
                    .as_ref()
                    .context("no stage0 expected value provided")?,
            )
            .context("stage0 measurement values failed verification")?;
            verify_amd_sev_attestation_report(report_values, amd_sev_values)
        }
        (Some(Report::Tdx(report_values)), _, Some(intel_tdx_values), _) => {
            verify_intel_tdx_attestation_report(report_values, intel_tdx_values)
        }
        (_, _, _, Some(insecure_values)) => {
            verify_insecure(insecure_values).context("insecure root layer verification failed")
        }
        (Some(Report::Fake(_)), _, _, None) => {
            Err(anyhow::anyhow!("unexpected insecure attestation report"))
        }
        (None, _, _, _) => Err(anyhow::anyhow!("no attestation report")),
        (_, _, _, _) => Err(anyhow::anyhow!(
            "invalid combination of root layer reference values and endorsed evidence"
        )),
    }
}

fn get_kernel_layer_expected_values(
    now_utc_millis: i64,
    endorsements: Option<&KernelLayerEndorsements>,
    reference_values: &KernelLayerReferenceValues,
) -> anyhow::Result<KernelLayerExpectedValues> {
    Ok(KernelLayerExpectedValues {
        kernel: Some(
            get_kernel_expected_values(
                now_utc_millis,
                endorsements.and_then(|value| value.kernel.as_ref()),
                reference_values.kernel.as_ref().context("no kernel reference value")?,
            )
            .context("failed to get kernel expected values")?,
        ),

        // TODO: b/331252282 - Remove temporary workaround for cmd line.
        kernel_cmd_line_text: Some(
            get_text_expected_values(
                now_utc_millis,
                reference_values
                    .kernel_cmd_line_text
                    .as_ref()
                    .context("no kernel command line text reference values")?,
                endorsements.and_then(|value| value.kernel_cmd_line.as_ref()),
            )
            .context("failed to get kernel cmd line expected value")?,
        ),

        init_ram_fs: Some(
            get_expected_measurement_digest(
                now_utc_millis,
                endorsements.and_then(|value| value.init_ram_fs.as_ref()),
                reference_values
                    .init_ram_fs
                    .as_ref()
                    .context("no initial RAM disk reference value")?,
            )
            .context("failed to get initramfs expected value")?,
        ),

        memory_map: Some(
            get_expected_measurement_digest(
                now_utc_millis,
                endorsements.and_then(|value| value.memory_map.as_ref()),
                reference_values.memory_map.as_ref().context("no memory map reference value")?,
            )
            .context("failed to get memory map expected value")?,
        ),

        acpi: Some(
            get_expected_measurement_digest(
                now_utc_millis,
                endorsements.and_then(|value| value.acpi.as_ref()),
                reference_values.acpi.as_ref().context("no ACPI reference value")?,
            )
            .context("failed to get ACPI table expected value")?,
        ),
    })
}

/// Verifies the measurement values of the kernel layer, which is common to both
/// the Oak Restricted Kernel and Oak Containers setups.
fn compare_kernel_layer_measurement_digests(
    values: &KernelLayerData,
    expected: &KernelLayerExpectedValues,
) -> anyhow::Result<()> {
    let expected_kernel_values = expected.kernel.as_ref().context("no kernel expected values")?;

    compare_measurement_digest(
        values.kernel_image.as_ref().context("no kernel evidence value")?,
        expected_kernel_values
            .image
            .as_ref()
            .context("expected values contained no image digests")?,
    )
    .context("kernel image failed verification")?;

    compare_measurement_digest(
        values.kernel_setup_data.as_ref().context("no kernel setup data evidence value")?,
        expected_kernel_values
            .setup_data
            .as_ref()
            .context("expected values contained no setup_data digests")?,
    )
    .context("kernel setup data failed verification")?;

    // TODO: b/331252282 - Remove temporary workaround for cmd line.
    match (&values.kernel_raw_cmd_line, &expected.kernel_cmd_line_text) {
        // We support the skipped value, whether the kernel cmd line text is valid or not.
        // Skipping is a way to work around the cmd line length limit.
        (_, Some(TextExpectedValue { r#type: Some(text_expected_value::Type::Skipped(_)) })) => {
            Ok(())
        }

        // If a kernel cmd line is provided, it must be shorter than 256 bytes.
        (Some(kernel_raw_cmd_line), Some(expected)) if kernel_raw_cmd_line.len() < 256 => {
            compare_text_value(kernel_raw_cmd_line.as_str(), expected)
        }
        _ => Err(anyhow::anyhow!("No valid kernel_raw_cmd_line provided")),
    }?;

    compare_measurement_digest(
        values.init_ram_fs.as_ref().context("no initramfs value provided")?,
        expected.init_ram_fs.as_ref().context("no initramfs expected value provided")?,
    )
    .context("initramfs value failed verification")?;
    compare_measurement_digest(
        values.memory_map.as_ref().context("no memory_map value provided")?,
        expected.memory_map.as_ref().context("no memory_map expected value provided")?,
    )
    .context("memory_map value failed verification")?;
    compare_measurement_digest(
        values.acpi.as_ref().context("no ACPI table value provided")?,
        expected.acpi.as_ref().context("no ACPI table expected value provided")?,
    )
    .context("ACPI table value failed verification")
}

fn get_event_expected_values(
    now_utc_millis: i64,
    reference_values: &EventReferenceValues,
) -> anyhow::Result<EventExpectedValues> {
    let event = Some(get_expected_measurement_digest(
        now_utc_millis,
        None,
        reference_values.event.as_ref().context("event reference value")?,
    )?);
    Ok(EventExpectedValues { event })
}

fn compare_event_measurement_digests(
    values: &EventData,
    expected: &EventExpectedValues,
) -> anyhow::Result<()> {
    compare_measurement_digest(
        values.event.as_ref().context("no event evidence value")?,
        expected.event.as_ref().context("no event image value")?,
    )
    .context("event failed verification")
}

fn get_system_layer_expected_values(
    now_utc_millis: i64,
    endorsements: Option<&SystemLayerEndorsements>,
    reference_values: &SystemLayerReferenceValues,
) -> anyhow::Result<SystemLayerExpectedValues> {
    let system_image = Some(get_expected_measurement_digest(
        now_utc_millis,
        endorsements.and_then(|value| value.system_image.as_ref()),
        reference_values.system_image.as_ref().context("container bundle reference value")?,
    )?);
    Ok(SystemLayerExpectedValues { system_image })
}

fn compare_system_layer_measurement_digests(
    values: &SystemLayerData,
    expected: &SystemLayerExpectedValues,
) -> anyhow::Result<()> {
    compare_measurement_digest(
        values.system_image.as_ref().context("no system image evidence value")?,
        expected.system_image.as_ref().context("no expected system image value")?,
    )
    .context("system layer system image failed verification")
}

fn get_application_layer_expected_values(
    now_utc_millis: i64,
    endorsements: Option<&ApplicationLayerEndorsements>,
    reference_values: &ApplicationLayerReferenceValues,
) -> anyhow::Result<ApplicationLayerExpectedValues> {
    let binary = Some(get_expected_measurement_digest(
        now_utc_millis,
        endorsements.and_then(|value| value.binary.as_ref()),
        reference_values.binary.as_ref().context("container bundle reference value")?,
    )?);
    let configuration = Some(get_expected_measurement_digest(
        now_utc_millis,
        endorsements.and_then(|value| value.configuration.as_ref()),
        reference_values.configuration.as_ref().context("container bundle reference value")?,
    )?);
    Ok(ApplicationLayerExpectedValues { binary, configuration })
}

fn compare_application_layer_measurement_digests(
    values: &ApplicationLayerData,
    expected: &ApplicationLayerExpectedValues,
) -> anyhow::Result<()> {
    compare_measurement_digest(
        values.binary.as_ref().context("no binary evidence value")?,
        expected.binary.as_ref().context("no expected binary value")?,
    )
    .context("application layer binary failed verification")?;
    compare_measurement_digest(
        values.config.as_ref().context("no config evidence value")?,
        expected.configuration.as_ref().context("no expected config value")?,
    )
    .context("application layer config failed verification")
}

fn get_container_layer_expected_values(
    now_utc_millis: i64,
    endorsements: Option<&ContainerLayerEndorsements>,
    reference_values: &ContainerLayerReferenceValues,
) -> anyhow::Result<ContainerLayerExpectedValues> {
    let bundle = Some(get_expected_measurement_digest(
        now_utc_millis,
        endorsements.and_then(|value| value.binary.as_ref()),
        reference_values.binary.as_ref().context("container bundle reference value")?,
    )?);
    let config = Some(get_expected_measurement_digest(
        now_utc_millis,
        endorsements.and_then(|value| value.binary.as_ref()),
        reference_values.configuration.as_ref().context("container config reference value")?,
    )?);
    Ok(ContainerLayerExpectedValues { bundle, config })
}

fn compare_container_layer_measurement_digests(
    values: &ContainerLayerData,
    expected: &ContainerLayerExpectedValues,
) -> anyhow::Result<()> {
    compare_measurement_digest(
        values.bundle.as_ref().context("no bundle evidence value")?,
        expected.bundle.as_ref().context("no expected bundle value")?,
    )
    .context("container bundle failed verification")?;
    compare_measurement_digest(
        values.config.as_ref().context("no config evidence value")?,
        expected.config.as_ref().context("no expected config value")?,
    )
    .context("container config failed verification")
}

// Generate the expected measurement digest values for the provided endorsement
// and reference_value. The resulting values can be cached by the client to
// avoid re-computation later.
fn get_expected_measurement_digest(
    now_utc_millis: i64,
    endorsement: Option<&TransparentReleaseEndorsement>,
    reference_value: &BinaryReferenceValue,
) -> anyhow::Result<ExpectedDigests> {
    match reference_value.r#type.as_ref() {
        Some(binary_reference_value::Type::Skip(_)) => Ok(ExpectedDigests {
            r#type: Some(expected_digests::Type::Skipped(VerificationSkipped {})),
        }),
        Some(binary_reference_value::Type::Endorsement(public_keys)) => {
            let endorsement =
                endorsement.context("matching endorsement not found for reference value")?;
            verify_binary_endorsement(
                now_utc_millis,
                &endorsement.endorsement,
                &endorsement.endorsement_signature,
                &endorsement.rekor_log_entry,
                &public_keys.endorser_public_key,
                &public_keys.rekor_public_key,
            )
            .context("verifying binary endorsement")?;
            Ok(into_expected_digests(&[hex_to_raw_digest(&get_digest(
                &parse_endorsement_statement(&endorsement.endorsement)?,
            )?)?]))
        }
        Some(binary_reference_value::Type::Digests(expected_digests)) => {
            Ok(into_expected_digests(&expected_digests.digests))
        }
        None => Err(anyhow::anyhow!("empty binary reference value")),
    }
}

fn into_expected_digests(source: &[RawDigest]) -> ExpectedDigests {
    ExpectedDigests {
        r#type: Some(expected_digests::Type::Digests(RawDigests { digests: source.to_vec() })),
    }
}
/// Verifies the measurement digest value against a reference value and
/// the expected digests calculated from endorsements and reference values.
fn compare_measurement_digest(
    measurement: &RawDigest,
    expected: &ExpectedDigests,
) -> anyhow::Result<()> {
    match expected.r#type.as_ref() {
        Some(expected_digests::Type::Skipped(_)) => Ok(()),
        Some(expected_digests::Type::Digests(digests)) => digests
            .digests
            .iter()
            .find(|expected| is_raw_digest_match(measurement, expected).is_ok())
            .map(|_| ())
            .ok_or(anyhow::anyhow!(
                "measurement digest {:?} does not match any reference values",
                measurement
            )),
        None => Err(anyhow::anyhow!("empty expected value")),
    }
}

// Extract the stage0 data from the provided Endorsement
// It will only be returned if the endorsement was verified.
fn get_verified_stage0_attachment(
    now_utc_millis: i64,
    endorsement: &TransparentReleaseEndorsement,
    public_keys: &EndorsementReferenceValue,
) -> anyhow::Result<FirmwareAttachment> {
    verify_binary_endorsement(
        now_utc_millis,
        &endorsement.endorsement,
        &endorsement.endorsement_signature,
        &endorsement.rekor_log_entry,
        &public_keys.endorser_public_key,
        &public_keys.rekor_public_key,
    )
    .context("verifying binary endorsement")?;
    // Parse endorsement statement and verify attachment digest.
    let parsed_statement = parse_endorsement_statement(&endorsement.endorsement)
        .context("parsing endorsement statement")?;
    if parsed_statement.predicate.usage != "firmware" {
        anyhow::bail!("unexpected endorsement usage: {}", parsed_statement.predicate.usage);
    }
    let expected_digest = get_digest(&parsed_statement).context("getting expected digest")?;
    let actual_digest = raw_to_hex_digest(&raw_digest_from_contents(&endorsement.subject));
    is_hex_digest_match(&actual_digest, &expected_digest).context("comparing digests")?;
    FirmwareAttachment::decode(&*endorsement.subject)
        .map_err(|_| anyhow::anyhow!("couldn't parse stage0 attachment"))
}

// Get the expected values from the provided TransparentReleaseEndorsement.
// The endorsement is expected to contain a subject that can be deserialized as
// a FirmwareAttachment.
// The subject itself will be verified, and then the expected digests (each
// corresponding to a number of vCPU, any of them a valid match for the digest
// in the evidence).
fn get_stage0_expected_values(
    now_utc_millis: i64,
    endorsement: Option<&TransparentReleaseEndorsement>,
    reference_value: &BinaryReferenceValue,
) -> anyhow::Result<ExpectedDigests> {
    match reference_value.r#type.as_ref() {
        Some(binary_reference_value::Type::Skip(_)) => Ok(ExpectedDigests {
            r#type: Some(expected_digests::Type::Skipped(VerificationSkipped {})),
        }),
        Some(binary_reference_value::Type::Endorsement(public_keys)) => {
            let firmware_attachment = get_verified_stage0_attachment(
                now_utc_millis,
                endorsement.context("matching endorsement not found for reference value")?,
                public_keys,
            )
            .context("getting verified stage0 attachment")?;

            Ok(into_expected_digests(
                firmware_attachment
                    .configs
                    .values()
                    .map(|digest| hex_to_raw_digest(digest).unwrap())
                    .collect::<Vec<RawDigest>>()
                    .as_slice(),
            ))
        }
        Some(binary_reference_value::Type::Digests(expected_digests)) => {
            Ok(into_expected_digests(expected_digests.digests.as_slice()))
        }

        None => Err(anyhow::anyhow!("empty stage0 reference value")),
    }
}

// Extract the KernelAttachment data from the provided Endorsement
// It will only be returned if the endorsement was verified.
fn get_verified_kernel_attachment(
    now_utc_millis: i64,
    endorsement: &TransparentReleaseEndorsement,
    public_keys: &EndorsementReferenceValue,
) -> anyhow::Result<KernelAttachment> {
    verify_binary_endorsement(
        now_utc_millis,
        &endorsement.endorsement,
        &endorsement.endorsement_signature,
        &endorsement.rekor_log_entry,
        &public_keys.endorser_public_key,
        &public_keys.rekor_public_key,
    )
    .context("verifying binary endorsement")?;
    // Parse endorsement statement and verify attachment digest.
    let parsed_statement = parse_endorsement_statement(&endorsement.endorsement)
        .context("parsing endorsement statement")?;
    if parsed_statement.predicate.usage != "kernel" {
        anyhow::bail!("unexpected endorsement usage: {}", parsed_statement.predicate.usage);
    }
    let expected_digest = get_digest(&parsed_statement).context("getting expected digest")?;
    let actual_digest = raw_to_hex_digest(&raw_digest_from_contents(&endorsement.subject));
    is_hex_digest_match(&actual_digest, &expected_digest).context("comparing expected digest")?;
    KernelAttachment::decode(&*endorsement.subject)
        .map_err(|_| anyhow::anyhow!("couldn't parse kernel attachment"))
}

// Get the expected values from the provided TransportReleaseEndorsement.
// The endorsement is expected to contain a subject that can be deserialized as
// a KernelAttachment.
// The subject itself will be verified, and then the image and setup_data
// expected values will be returned.
// Subsequent callers can provide just the cached image and setup_data digests.
fn get_kernel_expected_values(
    now_utc_millis: i64,
    endorsement: Option<&TransparentReleaseEndorsement>,
    reference_value: &KernelBinaryReferenceValue,
) -> anyhow::Result<KernelExpectedValues> {
    match reference_value.r#type.as_ref() {
        Some(kernel_binary_reference_value::Type::Skip(_)) => Ok(KernelExpectedValues {
            image: Some(ExpectedDigests {
                r#type: Some(expected_digests::Type::Skipped(VerificationSkipped {})),
            }),
            setup_data: Some(ExpectedDigests {
                r#type: Some(expected_digests::Type::Skipped(VerificationSkipped {})),
            }),
        }),
        Some(kernel_binary_reference_value::Type::Endorsement(public_keys)) => {
            let kernel_attachment = get_verified_kernel_attachment(
                now_utc_millis,
                endorsement.context("matching endorsement not found for reference value")?,
                public_keys,
            )
            .context("getting verified kernel attachment")?;
            let expected_image = kernel_attachment
                .image
                .ok_or_else(|| anyhow::anyhow!("no image digest in kernel attachment"))?;
            let expected_setup_data = kernel_attachment
                .setup_data
                .ok_or_else(|| anyhow::anyhow!("no setup data digest in kernel attachment"))?;

            Ok(KernelExpectedValues {
                image: Some(into_expected_digests(&[hex_to_raw_digest(&expected_image)?])),
                setup_data: Some(into_expected_digests(&[hex_to_raw_digest(
                    &expected_setup_data,
                )?])),
            })
        }
        Some(kernel_binary_reference_value::Type::Digests(expected_digests)) => {
            Ok(KernelExpectedValues {
                image: Some(into_expected_digests(
                    &expected_digests
                        .image
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("no image digests provided"))?
                        .digests,
                )),
                setup_data: Some(into_expected_digests(
                    &expected_digests
                        .setup_data
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("no setup_data digests provided"))?
                        .digests,
                )),
            })
        }
        None => Err(anyhow::anyhow!("empty binary reference value")),
    }
}

fn compare_text_value(actual: &str, expected: &TextExpectedValue) -> anyhow::Result<()> {
    match expected.r#type.as_ref() {
        Some(text_expected_value::Type::Skipped(_)) => Ok(()),
        Some(text_expected_value::Type::Regex(regex)) => {
            verify_regex(actual, &regex.value).context("regex from endorsement does not match")
        }
        Some(text_expected_value::Type::StringLiterals(string_literals)) => {
            if string_literals.value.iter().any(|sl| sl == actual) {
                Ok(())
            } else {
                Err(anyhow::anyhow!(format!(
                    "value doesn't match the reference value string literal: {actual}"
                )))
            }
        }
        None => Err(anyhow::anyhow!("missing skip or value in the text expected value")),
    }
}

fn get_text_expected_values(
    now_utc_millis: i64,
    value: &TextReferenceValue,
    endorsement: Option<&TransparentReleaseEndorsement>,
) -> anyhow::Result<TextExpectedValue> {
    match value.r#type.as_ref() {
        Some(text_reference_value::Type::Skip(_)) => Ok(TextExpectedValue {
            r#type: Some(text_expected_value::Type::Skipped(VerificationSkipped {})),
        }),
        Some(text_reference_value::Type::Endorsement(public_keys)) => {
            let endorsement =
                endorsement.context("matching endorsement not found for text reference value")?;
            verify_binary_endorsement(
                now_utc_millis,
                &endorsement.endorsement,
                &endorsement.endorsement_signature,
                &endorsement.rekor_log_entry,
                &public_keys.endorser_public_key,
                &public_keys.rekor_public_key,
            )
            .context("verifying binary endorsement")?;
            // Compare the actual command line against the one inlined in the endorsement.
            let regex = String::from_utf8(endorsement.subject.clone())
                .expect("endorsement subject is not utf8");
            Ok(TextExpectedValue {
                r#type: Some(text_expected_value::Type::Regex(ExpectedRegex { value: regex })),
            })
        }
        Some(text_reference_value::Type::Regex(regex)) => Ok(TextExpectedValue {
            r#type: Some(text_expected_value::Type::Regex(ExpectedRegex {
                value: regex.value.clone(),
            })),
        }),
        Some(text_reference_value::Type::StringLiterals(string_literals)) => {
            Ok(TextExpectedValue {
                r#type: Some(text_expected_value::Type::StringLiterals(ExpectedStringLiterals {
                    value: string_literals.value.clone(),
                })),
            })
        }
        None => Err(anyhow::anyhow!("missing skip or value in the text reference value")),
    }
}

#[cfg(feature = "regex")]
fn verify_regex(actual: &str, regex: &str) -> anyhow::Result<()> {
    let re = Regex::new(regex)
        .map_err(|msg| anyhow::anyhow!("couldn't parse regex in the reference value: {msg}"))?;
    Ok(anyhow::ensure!(
        re.is_match(actual),
        format!("value doesn't match the reference value regex: {actual}")
    ))
}

#[cfg(not(feature = "regex"))]
fn verify_regex(_actual: &str, _regex: &str) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("verification of regex values not supported"))
}

struct ApplicationKeyValues {
    encryption_public_key: Vec<u8>,
    signing_public_key: Vec<u8>,
}

/// Translates [`Stage0Measurements`] to [`KernelLayerData`]. Both hold the same
/// data, just in slightly different proto messages.
fn stage0_measurements_to_kernel_layer_data(measurements: Stage0Measurements) -> KernelLayerData {
    // We need to set fields of [`KernelLayerData`] to create it, some are
    // deprecated.
    #[allow(deprecated)]
    KernelLayerData {
        kernel_image: Some(RawDigest {
            sha2_256: measurements.kernel_measurement,
            ..Default::default()
        }),
        kernel_setup_data: Some(RawDigest {
            sha2_256: measurements.setup_data_digest,
            ..Default::default()
        }),
        kernel_cmd_line: None,
        kernel_raw_cmd_line: Some(measurements.kernel_cmdline),
        init_ram_fs: Some(RawDigest {
            sha2_256: measurements.ram_disk_digest,
            ..Default::default()
        }),
        memory_map: Some(RawDigest {
            sha2_256: measurements.memory_map_digest,
            ..Default::default()
        }),
        acpi: Some(RawDigest { sha2_256: measurements.acpi_digest, ..Default::default() }),
    }
}

/// Translates [`Stage1Measurements`] to [`SystemLayerData`]. Both hold the same
/// data, just in slightly different proto messages.
///
/// [`Stage1Measurements`] is deprecated and only present for compatibility
/// with older binaries that have been previously imported into Google3 and may
/// still use them.
#[allow(deprecated)]
fn stage1_measurements_to_system_layer_data(measurements: Stage1Measurements) -> SystemLayerData {
    SystemLayerData { system_image: measurements.system_image }
}

/// Translates [`OrchestratorMeasurements`] to
/// [`ContainerLayerData`]. Both hold the same data, just in slightly
/// different proto messages.
///
/// [`OrchestratorMeasurements`] is deprecated and only present for
/// compatibility  with older binaries that have been previously imported into
/// Google3 and may still use them.
#[allow(deprecated)]
fn oak_containers_orchestrator_measurements_to_container_layer_data(
    measurements: OrchestratorMeasurements,
) -> ContainerLayerData {
    ContainerLayerData { bundle: measurements.container_image, config: measurements.config }
}

/// Extracts measurements, public keys and other attestation-related values from
/// the evidence without verifying it. For most usecases, this function should
/// not be used. Instead use the [`verify`] function, which verifies the
/// attestation and only returns evidence upon successful verification. Hence
/// marked as dangerous.
pub fn extract_evidence(evidence: &Evidence) -> anyhow::Result<ExtractedEvidence> {
    let evidence_values =
        Some(extract_evidence_values(evidence).context("couldn't extract evidence values")?);
    let ApplicationKeyValues { encryption_public_key, signing_public_key } =
        extract_application_key_values(
            evidence.application_keys.as_ref().context("no application keys")?,
        )
        .context("couldn't extract application key values")?;

    Ok(ExtractedEvidence { evidence_values, encryption_public_key, signing_public_key })
}

/// Extracts the measurements and other attestation-related values from the
/// evidence.
fn extract_evidence_values(evidence: &Evidence) -> anyhow::Result<EvidenceValues> {
    let root_layer =
        Some(extract_root_values(evidence.root_layer.as_ref().context("no root layer evidence")?)?);

    if let Some(event_log) = &evidence.event_log
        && !event_log.encoded_events.is_empty()
    {
        let decoded_events: Vec<Event> = event_log
            .encoded_events
            .iter()
            .enumerate()
            .map(|(index, encoded_event)| {
                Event::decode(encoded_event.as_slice()).map_err(|e| {
                    anyhow::anyhow!("Failed to decode event with index {}: {}", index, e)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        #[derive(Debug, PartialEq)]
        enum EvidenceType {
            OakContainers,
            // Evidence from an older build of Oak Containers that uses deprecated event protos
            // in stage1 and the orchestrator.
            LegacyOakContainers,
            OakRestrictedKernel,
            CB,
        }

        let evidence_type = {
            if decoded_events.len() == 3
                && decoded_events[0].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.Stage0Measurements")
                && decoded_events[1].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.SystemLayerData")
                && decoded_events[2].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.ContainerLayerData")
            {
                EvidenceType::OakContainers
            } else if decoded_events.len() == 3
                && decoded_events[0].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.Stage0Measurements")
                && decoded_events[1].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.Stage1Measurements")
                && decoded_events[2].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.OrchestratorMeasurements")
            {
                EvidenceType::LegacyOakContainers
            } else if decoded_events.len() == 2
                && decoded_events[0].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.Stage0Measurements")
                && decoded_events[1].event.as_ref().map(|e| e.type_url.as_str())
                    == Some("type.googleapis.com/oak.attestation.v1.ApplicationLayerData")
            {
                EvidenceType::OakRestrictedKernel
            // CB evidence has three layers; if it does not extracting the
            // evidence will fail.
            } else if decoded_events.len() == 3 {
                EvidenceType::CB
            } else {
                anyhow::bail!("events indicate an unexpected evidence type");
            }
        };

        match evidence_type {
            EvidenceType::OakContainers => {
                let kernel_layer = decoded_events[0]
                    .event
                    .as_ref()
                    .and_then(|e| Stage0Measurements::decode(e.value.as_slice()).ok())
                    .map(stage0_measurements_to_kernel_layer_data)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak containers kernel layer data")
                    })?;
                let system_layer = decoded_events[1]
                    .event
                    .as_ref()
                    .and_then(|e| SystemLayerData::decode(e.value.as_slice()).ok())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak containers system layer data")
                    })?;
                let container_layer = decoded_events[2]
                    .event
                    .as_ref()
                    .and_then(|e| ContainerLayerData::decode(e.value.as_slice()).ok())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak containers container layer data")
                    })?;

                return Ok(EvidenceValues::OakContainers(OakContainersData {
                    root_layer,
                    kernel_layer: Some(kernel_layer),
                    system_layer: Some(system_layer),
                    container_layer: Some(container_layer),
                }));
            }
            EvidenceType::LegacyOakContainers => {
                let kernel_layer = decoded_events[0]
                    .event
                    .as_ref()
                    .and_then(|e| Stage0Measurements::decode(e.value.as_slice()).ok())
                    .map(stage0_measurements_to_kernel_layer_data)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak containers kernel layer data")
                    })?;
                let system_layer = decoded_events[1]
                    .event
                    .as_ref()
                    .and_then(|e| Stage1Measurements::decode(e.value.as_slice()).ok())
                    .map(stage1_measurements_to_system_layer_data)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode legacy oak containers system layer data")
                    })?;
                let container_layer = decoded_events[2]
                    .event
                    .as_ref()
                    .and_then(|e| OrchestratorMeasurements::decode(e.value.as_slice()).ok())
                    .map(oak_containers_orchestrator_measurements_to_container_layer_data)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Failed to decode legacy oak containers container layer data"
                        )
                    })?;
                return Ok(EvidenceValues::OakContainers(OakContainersData {
                    root_layer,
                    kernel_layer: Some(kernel_layer),
                    system_layer: Some(system_layer),
                    container_layer: Some(container_layer),
                }));
            }
            EvidenceType::OakRestrictedKernel => {
                let kernel_layer = decoded_events[0]
                    .event
                    .as_ref()
                    .and_then(|e| Stage0Measurements::decode(e.value.as_slice()).ok())
                    .map(stage0_measurements_to_kernel_layer_data)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak restricted kernel layer data")
                    })?;
                let application_layer = decoded_events[1]
                    .event
                    .as_ref()
                    .and_then(|e| ApplicationLayerData::decode(e.value.as_slice()).ok())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to decode oak restricted application layer data")
                    })?;

                return Ok(EvidenceValues::OakRestrictedKernel(OakRestrictedKernelData {
                    root_layer,
                    kernel_layer: Some(kernel_layer),
                    application_layer: Some(application_layer),
                }));
            }
            EvidenceType::CB => Ok(EvidenceValues::Cb(CbData {
                root_layer,
                kernel_layer: Some(EventData {
                    event: Some(RawDigest {
                        sha2_256: sha2::Sha256::digest(&event_log.encoded_events[0]).to_vec(),
                        ..Default::default()
                    }),
                }),
                system_layer: Some(EventData {
                    event: Some(RawDigest {
                        sha2_256: sha2::Sha256::digest(&event_log.encoded_events[1]).to_vec(),
                        ..Default::default()
                    }),
                }),
                application_layer: Some(EventData {
                    event: Some(RawDigest {
                        sha2_256: sha2::Sha256::digest(&event_log.encoded_events[2]).to_vec(),
                        ..Default::default()
                    }),
                }),
            })),
        }
    }
    // There's no eventlog, proceed to extract evidence using the existing logic.
    else {
        let final_layer_claims = &claims_set_from_serialized_cert(
            &evidence
                .application_keys
                .as_ref()
                .context("no application keys")?
                .encryption_public_key_certificate,
        )
        .context("couldn't parse final DICE layer certificate")?;

        // Determine the type of evidence from the claims in the certificate for the
        // final.
        if let Ok(container_layer_data) = extract_container_layer_data(final_layer_claims) {
            match &evidence.layers[..] {
                [kernel_layer, system_layer] => {
                    let kernel_layer = Some(
                        extract_kernel_values(
                            &claims_set_from_serialized_cert(&kernel_layer.eca_certificate)
                                .context("couldn't parse kernel DICE layer certificate")?,
                        )
                        .context("couldn't extract kernel values")?,
                    );
                    let system_layer = Some(
                        extract_system_layer_data(
                            &claims_set_from_serialized_cert(&system_layer.eca_certificate)
                                .context("couldn't parse system DICE layer certificate")?,
                        )
                        .context("couldn't extract system layer values")?,
                    );

                    let container_layer = Some(container_layer_data);
                    Ok(EvidenceValues::OakContainers(OakContainersData {
                        root_layer,
                        kernel_layer,
                        system_layer,
                        container_layer,
                    }))
                }
                _ => Err(anyhow::anyhow!("incorrect number of DICE layers for Oak Containers")),
            }
        } else if let Ok(application_layer_data) =
            extract_application_layer_data(final_layer_claims)
        {
            match &evidence.layers[..] {
                [kernel_layer] => {
                    let kernel_layer = Some(
                        extract_kernel_values(
                            &claims_set_from_serialized_cert(&kernel_layer.eca_certificate)
                                .context("couldn't parse kernel DICE layer certificate")?,
                        )
                        .context("couldn't extract kernel values")?,
                    );

                    let application_layer = Some(application_layer_data);
                    Ok(EvidenceValues::OakRestrictedKernel(OakRestrictedKernelData {
                        root_layer,
                        kernel_layer,
                        application_layer,
                    }))
                }
                _ => Err(anyhow::anyhow!("incorrect number of DICE layers for Oak Containers")),
            }
        } else {
            match &evidence.layers[..] {
                [kernel_layer, system_layer, application_layer] => {
                    let kernel_layer = Some(
                        extract_event_data(
                            &claims_set_from_serialized_cert(&kernel_layer.eca_certificate)
                                .context("couldn't parse CB kernel DICE layer certificate")?,
                        )
                        .context("couldn't extract kernel values")?,
                    );
                    let system_layer = Some(
                        extract_event_data(
                            &claims_set_from_serialized_cert(&system_layer.eca_certificate)
                                .context("couldn't parse CB system DICE layer certificate")?,
                        )
                        .context("couldn't extract system values")?,
                    );
                    let application_layer = Some(
                        extract_event_data(
                            &claims_set_from_serialized_cert(&application_layer.eca_certificate)
                                .context("couldn't parse CB application DICE layer certificate")?,
                        )
                        .context("couldn't extract application values")?,
                    );

                    Ok(EvidenceValues::Cb(CbData {
                        root_layer,
                        kernel_layer,
                        system_layer,
                        application_layer,
                    }))
                }
                _ => Err(anyhow::anyhow!("incorrect number of DICE layers for CB")),
            }
        }
    }
}

/// Extracts values from the attestation report.
fn extract_root_values(root_layer: &RootLayerEvidence) -> anyhow::Result<RootLayerData> {
    match root_layer.platform() {
        TeePlatform::Unspecified => Err(anyhow::anyhow!("unspecified TEE platform")),
        TeePlatform::AmdSevSnp => {
            let report = AttestationReport::ref_from(&root_layer.remote_attestation_report)
                .context("invalid AMD SEV-SNP attestation report")?;

            report.validate().map_err(|msg| anyhow::anyhow!(msg))?;

            let current_tcb = Some(TcbVersion {
                boot_loader: report.data.current_tcb.boot_loader.into(),
                tee: report.data.current_tcb.tee.into(),
                snp: report.data.current_tcb.snp.into(),
                microcode: report.data.current_tcb.microcode.into(),
            });
            let reported_tcb = Some(TcbVersion {
                boot_loader: report.data.reported_tcb.boot_loader.into(),
                tee: report.data.reported_tcb.tee.into(),
                snp: report.data.reported_tcb.snp.into(),
                microcode: report.data.reported_tcb.microcode.into(),
            });
            let debug = report.has_debug_flag().map_err(|error| anyhow::anyhow!(error))?;
            let hardware_id = report.data.chip_id.as_ref().to_vec();
            let initial_measurement = report.data.measurement.as_ref().to_vec();
            let report_data = report.data.report_data.as_ref().to_vec();
            let vmpl = report.data.vmpl;

            Ok(RootLayerData {
                report: Some(Report::SevSnp(AmdAttestationReport {
                    current_tcb,
                    reported_tcb,
                    debug,
                    initial_measurement,
                    hardware_id,
                    report_data,
                    vmpl,
                })),
            })
        }
        TeePlatform::IntelTdx => Err(anyhow::anyhow!("not supported")),
        TeePlatform::None => {
            // We use an unsigned, mostly empty AMD SEV-SNP attestation report as a fake
            // when not running in a TEE.
            let report = AttestationReport::ref_from(&root_layer.remote_attestation_report)
                .context("invalid fake attestation report")?;

            report.validate().map_err(|msg| anyhow::anyhow!(msg))?;

            let report_data = report.data.report_data.as_ref().to_vec();

            Ok(RootLayerData { report: Some(Report::Fake(FakeAttestationReport { report_data })) })
        }
    }
}

/// Extracts application key values. There are two possible cases where
/// in the first case application keys are defined as cose key and another case
/// application keys are part of payload.
fn extract_application_key_values(
    application_keys: &ApplicationKeys,
) -> anyhow::Result<ApplicationKeyValues> {
    let application_key_values = || -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
        let encryption_claims = claims_set_from_serialized_cert(
            &application_keys.encryption_public_key_certificate[..],
        )
        .context("getting encryption claims")?;
        let encryption_cose_key = get_public_key_from_claims_set(&encryption_claims)
            .map_err(|msg| anyhow::anyhow!(msg))
            .context("getting encryption cose key")?;
        let encryption_public_key = cose_key_to_hpke_public_key(&encryption_cose_key)
            .map_err(|msg| anyhow::anyhow!(msg))
            .context("converting encryption cose key")?;

        let signing_claims =
            claims_set_from_serialized_cert(&application_keys.signing_public_key_certificate[..])
                .context("getting signing claims")?;
        let signing_cose_key: CoseKey = get_public_key_from_claims_set(&signing_claims)
            .map_err(|msg| anyhow::anyhow!(msg))
            .context("getting signing cose key")?;
        let signing_verifying_key = cose_key_to_verifying_key(&signing_cose_key)
            .map_err(|msg| anyhow::anyhow!(msg))
            .context("getting signing verifying key")?;
        let signing_public_key = signing_verifying_key.to_sec1_bytes().to_vec();

        Ok((encryption_public_key, signing_public_key))
    };
    let (encryption_public_key, signing_public_key) = match application_key_values() {
        Ok(values) => values,
        Err(_) => {
            let encryption_cert = coset::CoseSign1::from_slice(
                &application_keys.encryption_public_key_certificate,
            )
            .map_err(|_cose_err| anyhow::anyhow!("could not parse encryption certificate"))?;
            let encryption_payload = encryption_cert
                .payload
                .ok_or_else(|| anyhow::anyhow!("no encryption cert payload"))?;
            let encryption_claims = ClaimsSet::from_slice(&encryption_payload)
                .map_err(|_cose_err| anyhow::anyhow!("could not parse encryption claims set"))?;
            let encryption_public_key =
                extract_bytes_from_claims_set(&encryption_claims, APPLICATION_KEY_ID)
                    .context("key ID not found")?;

            let signing_cert =
                coset::CoseSign1::from_slice(&application_keys.signing_public_key_certificate)
                    .map_err(|_cose_err| {
                        anyhow::anyhow!("could not parse encryption certificate")
                    })?;
            let signing_payload =
                signing_cert.payload.ok_or_else(|| anyhow::anyhow!("no signing cert payload"))?;
            let signing_claims = ClaimsSet::from_slice(&signing_payload)
                .map_err(|_cose_err| anyhow::anyhow!("could not parse signing claims set"))?;
            let signing_public_key =
                extract_bytes_from_claims_set(&signing_claims, APPLICATION_KEY_ID)
                    .context("key ID not found")?;

            (encryption_public_key, signing_public_key)
        }
    };
    Ok(ApplicationKeyValues { encryption_public_key, signing_public_key })
}

/// Extracts the measurement values for the event data.
fn extract_event_data(claims: &ClaimsSet) -> anyhow::Result<EventData> {
    let values =
        extract_value_from_claims_set(claims, EVENT_ID).context("event data layer ID not found")?;
    let event = Some(value_to_raw_digest(values)?);
    Ok(EventData { event })
}

/// Extracts the measurement values for the kernel layer.
fn extract_kernel_values(claims: &ClaimsSet) -> anyhow::Result<KernelLayerData> {
    let values =
        extract_layer_data(claims, KERNEL_LAYER_ID).context("kernel layer ID not found")?;
    let kernel_image = Some(value_to_raw_digest(extract_value(values, KERNEL_MEASUREMENT_ID)?)?);
    let kernel_setup_data =
        Some(value_to_raw_digest(extract_value(values, SETUP_DATA_MEASUREMENT_ID)?)?);
    let kernel_cmd_line =
        Some(value_to_raw_digest(extract_value(values, KERNEL_COMMANDLINE_MEASUREMENT_ID)?)?);
    let kernel_raw_cmd_line = extract_value(values, KERNEL_COMMANDLINE_ID)
        .ok()
        .map(|v| String::from(v.as_text().expect("kernel_raw_cmd_line found but is not a string")));
    let init_ram_fs = Some(value_to_raw_digest(extract_value(values, INITRD_MEASUREMENT_ID)?)?);
    let memory_map = Some(value_to_raw_digest(extract_value(values, MEMORY_MAP_MEASUREMENT_ID)?)?);
    let acpi = Some(value_to_raw_digest(extract_value(values, ACPI_MEASUREMENT_ID)?)?);
    #[allow(deprecated)]
    Ok(KernelLayerData {
        kernel_image,
        kernel_setup_data,
        kernel_cmd_line,
        kernel_raw_cmd_line,
        init_ram_fs,
        memory_map,
        acpi,
    })
}

/// Extracts the measurement values for the system image layer.
fn extract_system_layer_data(claims: &ClaimsSet) -> anyhow::Result<SystemLayerData> {
    let values =
        extract_layer_data(claims, SYSTEM_IMAGE_LAYER_ID).context("system layer ID not found")?;
    let system_image =
        Some(value_to_raw_digest(extract_value(values, LAYER_2_CODE_MEASUREMENT_ID)?)?);
    Ok(SystemLayerData { system_image })
}

/// Extracts the measurement values for the system image layer.
fn extract_container_layer_data(claims: &ClaimsSet) -> anyhow::Result<ContainerLayerData> {
    let values = extract_layer_data(claims, CONTAINER_IMAGE_LAYER_ID)
        .context("system layer ID not found")?;
    let bundle = Some(value_to_raw_digest(extract_value(values, LAYER_3_CODE_MEASUREMENT_ID)?)?);
    let config =
        Some(value_to_raw_digest(extract_value(values, FINAL_LAYER_CONFIG_MEASUREMENT_ID)?)?);
    Ok(ContainerLayerData { bundle, config })
}

/// Extracts the measurement values for the enclave application layer.
fn extract_application_layer_data(claims: &ClaimsSet) -> anyhow::Result<ApplicationLayerData> {
    let values = extract_layer_data(claims, ENCLAVE_APPLICATION_LAYER_ID)
        .context("system layer ID not found")?;
    let binary = Some(value_to_raw_digest(extract_value(values, LAYER_2_CODE_MEASUREMENT_ID)?)?);
    let config =
        Some(value_to_raw_digest(extract_value(values, FINAL_LAYER_CONFIG_MEASUREMENT_ID)?)?);
    Ok(ApplicationLayerData { binary, config })
}

/// Parses the CBOR map from a serialized certificate.
fn claims_set_from_serialized_cert(slice: &[u8]) -> anyhow::Result<ClaimsSet> {
    let cert = coset::CoseSign1::from_slice(slice)
        .map_err(|_cose_err| anyhow::anyhow!("could not parse certificate"))?;
    let payload = cert.payload.ok_or_else(|| anyhow::anyhow!("no signing cert payload"))?;
    ClaimsSet::from_slice(&payload)
        .map_err(|_cose_err| anyhow::anyhow!("could not parse claims set"))
}

/// Extracts the claim that contains the values for the specified layer.
fn extract_layer_data(claims: &ClaimsSet, layer_id: i64) -> anyhow::Result<&Vec<(Value, Value)>> {
    let target = RegisteredLabelWithPrivate::PrivateUse(layer_id);
    claims
        .rest
        .iter()
        .find_map(|(label, value)| {
            if let Value::Map(map) = value
                && label == &target
            {
                Some(map)
            } else {
                None
            }
        })
        .context("couldn't find layer values")
}

/// Extracts bytes for the label from claim.
fn extract_bytes_from_claims_set(claims: &ClaimsSet, label_id: i64) -> anyhow::Result<Vec<u8>> {
    match extract_value_from_claims_set(claims, label_id)? {
        Value::Bytes(bytes) => Ok(bytes.to_vec()),
        _ => Err(anyhow::anyhow!("value was not of type Bytes")),
    }
}

/// Extracts a value for the label from claim.
fn extract_value_from_claims_set(claims: &ClaimsSet, label_id: i64) -> anyhow::Result<&Value> {
    let target_label = RegisteredLabelWithPrivate::PrivateUse(label_id);
    claims
        .rest
        .iter()
        .find_map(|(label, value)| if label == &target_label { Some(value) } else { None })
        .context(format!("couldn't find value {label_id}"))
}

/// Extracts a value for the label from the layer's mapping between labels and
/// values.
fn extract_value(values: &[(Value, Value)], label_id: i64) -> anyhow::Result<&Value> {
    let target_key = Value::Integer(label_id.into());
    values
        .iter()
        .find_map(|(key, value)| if key == &target_key { Some(value) } else { None })
        .context(format!("couldn't find measurement {label_id}"))
}

/// Extracts the individual digests from a value that represents a set of
/// digests.
fn value_to_raw_digest(value: &Value) -> anyhow::Result<RawDigest> {
    if let Value::Map(map) = value {
        let mut result = RawDigest::default();
        for (key, digest) in map.iter() {
            if let Value::Bytes(bytes) = digest {
                if key == &Value::Integer(SHA2_256_ID.into()) {
                    result.sha2_256 = bytes.to_vec();
                } else {
                    anyhow::bail!("digest is not a byte array");
                }
            } else {
                anyhow::bail!("digest is not a byte array");
            }
        }
        return Ok(result);
    }
    Err(anyhow::anyhow!("value is not a map of digests"))
}
