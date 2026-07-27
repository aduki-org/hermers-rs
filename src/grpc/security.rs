//! SecurityService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::security::{StatusReq, StatusResp};
use crate::grpc::transport::Transport;

/// SecurityService.
pub struct Security {
    transport: Transport,
}

impl Security {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Security status snapshot.
    pub async fn status(&self) -> Result<StatusResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.security.clone();
        Ok(client.status(StatusReq {}).await?.into_inner())
    }
}
