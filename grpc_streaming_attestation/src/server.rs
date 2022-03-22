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

//! Server-side implementation of the bidirectional streaming gRPC remote attestation handshake
//! protocol.

use crate::proto::{
    streaming_session_server::StreamingSession, StreamingRequest, StreamingResponse,
};
use anyhow::Context;
use futures::{Stream, StreamExt};
use oak_remote_attestation::handshaker::{AttestationBehavior, Encryptor, ServerHandshaker};
use oak_utils::LogError;
use std::{pin::Pin, sync::Arc};
use tonic::{Request, Response, Status, Streaming};

/// Handler for subsequent encrypted requests from the stream after the handshake is completed.
struct EncryptedRequestHandler<F, S>
where
    F: Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync,
{
    /// The stream of incoming requests.
    request_stream: Streaming<StreamingRequest>,
    /// Session-specific encryptor and decryptor that uses key generated by the remote attestation
    /// handshake.
    encryptor: Encryptor,
    /// Handler function that processes the decrypted subsequent requests from the client.
    cleartext_handler: F,
}

impl<F, S> EncryptedRequestHandler<F, S>
where
    F: Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync,
{
    pub fn new(
        request_stream: Streaming<StreamingRequest>,
        encryptor: Encryptor,
        cleartext_handler: F,
    ) -> Self {
        Self {
            request_stream,
            encryptor,
            cleartext_handler,
        }
    }

    /// Decrypts a client request, runs [`EncryptedRequestHandler::cleartext_handler`] on the
    /// resulting cleartext and encrypts the response.
    ///
    /// Returns `Ok(None)` to indicate that the corresponding gRPC stream has ended.
    pub async fn handle_next_request(&mut self) -> anyhow::Result<Option<StreamingResponse>> {
        if let Some(encrypted_request) = self.request_stream.next().await {
            let encrypted_request = encrypted_request.context("Couldn't receive request")?;
            let decrypted_request = self
                .encryptor
                .decrypt(&encrypted_request.body)
                .context("Couldn't decrypt request")?;

            let response = (self.cleartext_handler.clone())(decrypted_request).await?;
            let encrypted_response = self
                .encryptor
                .encrypt(&response)
                .context("Couldn't decrypt response")?;

            Ok(Some(StreamingResponse {
                body: encrypted_response,
            }))
        } else {
            Ok(None)
        }
    }
}

/// gRPC Attestation Service implementation.
pub struct AttestationServer<F, L: LogError> {
    /// PEM encoded X.509 certificate that signs TEE firmware key.
    tee_certificate: Vec<u8>,
    /// Processes data from client requests and creates responses.
    request_handler: F,
    /// Configuration information to provide to the client for the attestation step.
    additional_info: Arc<Vec<u8>>,
    /// Error logging function that is required for logging attestation protocol errors.
    /// Errors are only logged on server side and are not sent to clients.
    error_logger: L,
}

impl<F, S, L> AttestationServer<F, L>
where
    F: Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync,
    L: Send + Sync + Clone + LogError,
{
    pub fn create(
        tee_certificate: Vec<u8>,
        request_handler: F,
        additional_info: Vec<u8>,
        error_logger: L,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            tee_certificate,
            request_handler,
            additional_info: Arc::new(additional_info),
            error_logger,
        })
    }
}

#[tonic::async_trait]
impl<F, S, L> StreamingSession for AttestationServer<F, L>
where
    F: 'static + Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync + 'static,
    L: Send + Sync + Clone + LogError + 'static,
{
    type StreamStream =
        Pin<Box<dyn Stream<Item = Result<StreamingResponse, Status>> + Send + 'static>>;

    async fn stream(
        &self,
        request_stream: Request<Streaming<StreamingRequest>>,
    ) -> Result<Response<Self::StreamStream>, Status> {
        let tee_certificate = self.tee_certificate.clone();
        let request_handler = self.request_handler.clone();
        let error_logger = self.error_logger.clone();
        let additional_info = self.additional_info.clone();

        let response_stream = async_stream::try_stream! {
            let mut request_stream = request_stream.into_inner();
            /// Attest a single gRPC streaming conection.
            let mut handshaker = ServerHandshaker::new(
                AttestationBehavior::create_self_attestation(&tee_certificate)
                    .map_err(|error| {
                        error_logger.log_error(&format!("Couldn't create self attestation behavior: {:?}", error));
                        Status::internal("")
                    })?,
                    additional_info
            );
            while !handshaker.is_completed() {
                let incoming_message = request_stream.next()
                    .await
                    .ok_or_else(|| {
                        error_logger.log_error("Stream stopped preemptively");
                        Status::internal("")
                    })?
                    .map_err(|error| {
                        error_logger.log_error(&format!("Couldn't get next message: {:?}", error));
                        Status::internal("")
                    })?;

                let outgoing_message = handshaker
                    .next_step(&incoming_message.body)
                    .map_err(|error| {
                        error_logger.log_error(&format!("Couldn't process handshake message: {:?}", error));
                        Status::aborted("")
                    })?;
                if let Some(outgoing_message) = outgoing_message {
                    yield StreamingResponse {
                        body: outgoing_message,
                    };
                }
            }
            let encryptor = handshaker
                .get_encryptor()
                .map_err(|error| {
                    error_logger.log_error(&format!("Couldn't get encryptor: {:?}", error));
                    Status::internal("")
                })?;

            let mut handler = EncryptedRequestHandler::<F, S>::new(request_stream, encryptor, request_handler);
            while let Some(response) = handler
                .handle_next_request()
                .await
                .map_err(|error| {
                    error_logger.log_error(&format!("Couldn't handle request: {:?}", error));
                    Status::aborted("")
                })?
            {
                yield response;
            }
        };

        Ok(Response::new(Box::pin(response_stream)))
    }
}
