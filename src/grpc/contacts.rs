//! ContactService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::contact::{
    Contact, CreateReq, GetReq, ListReq, ListResp, RemoveReq, SyncReq, SyncResp, UpdateReq,
};
use crate::grpc::transport::Transport;
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

/// ContactService — proto shapes (gRPC-only).
pub struct Contacts {
    transport: Transport,
}

impl Contacts {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// List contacts.
    pub async fn list(
        &self,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ListResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.contacts.clone();
        let resp = client
            .list(ListReq {
                tenant,
                cursor: cursor.unwrap_or("").to_string(),
                limit: limit.unwrap_or(50),
            })
            .await?;
        Ok(resp.into_inner())
    }

    /// Get by hex.
    pub async fn retrieve(&self, hex: &str) -> Result<Contact, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.contacts.clone();
        Ok(client
            .get(GetReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Create from vCard.
    pub async fn create(&self, vcard: &str) -> Result<Contact, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let owner = self.transport.require_user().await?;
        let mut client = self.transport.contacts.clone();
        Ok(client
            .create(CreateReq {
                tenant,
                owner,
                vcard: vcard.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Update with etag.
    pub async fn update(
        &self,
        hex: &str,
        vcard: &str,
        etag: &str,
    ) -> Result<Contact, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.contacts.clone();
        Ok(client
            .update(UpdateReq {
                hex: hex.to_string(),
                vcard: vcard.to_string(),
                etag: etag.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Remove.
    pub async fn remove(&self, hex: &str) -> Result<(), HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.contacts.clone();
        client
            .remove(RemoveReq {
                hex: hex.to_string(),
            })
            .await?;
        Ok(())
    }

    /// Sync since timestamp (unix seconds).
    pub async fn sync(&self, since_unix: i64) -> Result<SyncResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.contacts.clone();
        Ok(client
            .sync(SyncReq {
                tenant,
                since: Some(Timestamp {
                    seconds: since_unix,
                    nanos: 0,
                }),
            })
            .await?
            .into_inner())
    }

    /// Sync since [`SystemTime`].
    pub async fn sync_since(&self, since: SystemTime) -> Result<SyncResp, HermesGrpcError> {
        let secs = since
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.sync(secs).await
    }
}
