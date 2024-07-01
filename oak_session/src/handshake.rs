//
// Copyright 2024 The Project Oak Authors
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

//! This module provides an implementation of the Handshaker, which
//! handles cryptographic handshake and secure session creation.

use alloc::vec::Vec;
use core::convert::TryInto;

use anyhow::{anyhow, Context};
use oak_crypto::noise_handshake::{client::HandshakeInitiator, Crypter};
use oak_proto_rust::oak::{
    crypto::v1::SessionKeys,
    session::v1::{
        handshake_request, handshake_response, HandshakeRequest, HandshakeResponse,
        NoiseHandshakeMessage,
    },
};

use crate::{config::HandshakerConfig, ProtocolEngine};

const EPHEMERAL_PUB_KEY_LEN: usize = 65;
const HANDSHAKE_HASH_LEN: usize = 32;

pub trait EncryptionKeyHandle {
    fn get_public_key(&self) -> anyhow::Result<Vec<u8>>;
}

#[derive(Copy, Clone)]
pub enum HandshakeType {
    NoiseKK,
    NoiseKN,
    NoiseNK,
    NoiseNN,
}

pub trait Handshaker {
    fn derive_session_keys(self) -> Option<SessionKeys>;
}

/// Client-side Handshaker that initiates the crypto handshake with the server.
#[allow(dead_code)]
pub struct ClientHandshaker<'a> {
    handshaker_config: HandshakerConfig<'a>,
    handshake_initiator: HandshakeInitiator,
    handshake_result: Option<([u8; HANDSHAKE_HASH_LEN], Crypter)>,
}

impl<'a> ClientHandshaker<'a> {
    pub fn create(handshaker_config: HandshakerConfig<'a>) -> anyhow::Result<Self> {
        let handshake_type = handshaker_config.handshake_type;
        let peer_static_public_key = handshaker_config.peer_static_public_key.clone();
        Ok(Self {
            handshaker_config,
            handshake_initiator: match handshake_type {
                HandshakeType::NoiseKK => core::unimplemented!(),
                HandshakeType::NoiseKN => core::unimplemented!(),
                HandshakeType::NoiseNK => HandshakeInitiator::new_nk(
                    peer_static_public_key
                        .context("handshaker_config missing the peer public key")?
                        .as_slice()
                        .try_into()
                        .map_err(|error| anyhow!("invalid peer public key: {:?}", error))?,
                ),
                HandshakeType::NoiseNN => HandshakeInitiator::new_nn(),
            },
            handshake_result: None,
        })
    }
}

impl<'a> Handshaker for ClientHandshaker<'a> {
    fn derive_session_keys(mut self) -> Option<SessionKeys> {
        self.handshake_result.take().map(|(_handshake_hash, crypter)| crypter.into())
    }
}

impl<'a> ProtocolEngine<HandshakeResponse, HandshakeRequest> for ClientHandshaker<'a> {
    fn get_outgoing_message(&mut self) -> anyhow::Result<Option<HandshakeRequest>> {
        let mut initial_message = self.handshake_initiator.build_initial_message();
        let (ciphertext, ephemeral_public_key) =
            (initial_message.split_off(EPHEMERAL_PUB_KEY_LEN), initial_message);
        Ok(Some(HandshakeRequest {
            r#handshake_type: Some(handshake_request::HandshakeType::NoiseHandshakeMessage(
                NoiseHandshakeMessage {
                    ephemeral_public_key,
                    static_public_key: match self.handshaker_config.handshake_type {
                        HandshakeType::NoiseKK
                        | HandshakeType::NoiseKN
                        | HandshakeType::NoiseNK
                        | HandshakeType::NoiseNN => vec![],
                    },
                    ciphertext,
                },
            )),
            attestation_binding: None,
        }))
    }

    fn put_incoming_message(
        &mut self,
        incoming_message: &HandshakeResponse,
    ) -> anyhow::Result<Option<()>> {
        match incoming_message.r#handshake_type.as_ref() {
            Some(handshake_response::HandshakeType::NoiseHandshakeMessage(noise_message)) => {
                let handshake_response = [
                    noise_message.ephemeral_public_key.as_slice(),
                    noise_message.static_public_key.as_slice(),
                    noise_message.ciphertext.as_slice(),
                ]
                .concat();
                self.handshake_result =
                    Some(self.handshake_initiator.process_response(handshake_response.as_slice()));
                Ok(Some(()))
            }
            None => Err(anyhow!("Missing handshake_type")),
        }
    }
}

/// Server-side Handshaker that responds to the crypto handshake request from
/// the client.
#[allow(dead_code)]
pub struct ServerHandshaker<'a> {
    handshaker_config: HandshakerConfig<'a>,
}

impl<'a> ServerHandshaker<'a> {
    pub fn new(handshaker_config: HandshakerConfig<'a>) -> Self {
        Self { handshaker_config }
    }
}

impl<'a> Handshaker for ServerHandshaker<'a> {
    fn derive_session_keys(self) -> Option<SessionKeys> {
        core::unimplemented!();
    }
}

impl<'a> ProtocolEngine<HandshakeRequest, HandshakeResponse> for ServerHandshaker<'a> {
    fn get_outgoing_message(&mut self) -> anyhow::Result<Option<HandshakeResponse>> {
        core::unimplemented!();
    }

    fn put_incoming_message(
        &mut self,
        _incoming_message: &HandshakeRequest,
    ) -> anyhow::Result<Option<()>> {
        core::unimplemented!();
    }
}
