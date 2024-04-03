/*
 * Copyright 2023 The Project Oak Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "cc/attestation/verification/insecure_attestation_verifier.h"

#include <chrono>
#include <string>

#include "absl/status/statusor.h"
#include "cc/attestation/verification/utils.h"
#include "proto/attestation/endorsement.pb.h"
#include "proto/attestation/evidence.pb.h"
#include "proto/attestation/verification.pb.h"

namespace oak::attestation::verification {

namespace {
using ::oak::attestation::v1::AttestationResults;
using ::oak::attestation::v1::Endorsements;
using ::oak::attestation::v1::Evidence;
}  // namespace

absl::StatusOr<AttestationResults> InsecureAttestationVerifier::Verify(
    std::chrono::time_point<std::chrono::system_clock> now,
    const Evidence& evidence, const Endorsements& endorsements) const {
  absl::StatusOr<std::string> encryption_public_key =
      ExtractEncryptionPublicKey(evidence);
  if (!encryption_public_key.ok()) {
    return encryption_public_key.status();
  }

  AttestationResults attestation_results;
  attestation_results.set_status(AttestationResults::STATUS_SUCCESS);
  *attestation_results.mutable_encryption_public_key() = *encryption_public_key;

  return attestation_results;
}

}  // namespace oak::attestation::verification
