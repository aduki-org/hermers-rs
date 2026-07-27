//! StorageService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::storage::{GetReq, PutReq, PutResp, RemoveReq};
use crate::grpc::transport::Transport;
use tokio_stream::StreamExt;

/// StorageService.
pub struct Storage {
    transport: Transport,
}

impl Storage {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Put blob.
    pub async fn put(&self, key: &str, data: Vec<u8>) -> Result<PutResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.storage.clone();
        Ok(client
            .put(PutReq {
                tenant,
                key: key.to_string(),
                data,
            })
            .await?
            .into_inner())
    }

    /// Get blob bytes (collects stream).
    pub async fn get(&self, hex: &str) -> Result<Vec<u8>, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.storage.clone();
        let mut stream = client
            .get(GetReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner();
        let mut out = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            out.extend_from_slice(&chunk.data);
        }
        Ok(out)
    }

    /// Remove blob.
    pub async fn remove(&self, hex: &str) -> Result<(), HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.storage.clone();
        client
            .remove(RemoveReq {
                hex: hex.to_string(),
            })
            .await?;
        Ok(())
    }
}
