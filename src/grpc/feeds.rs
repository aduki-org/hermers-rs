//! FeedService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::feeds::{
    CreateReq, Feed, GetReq, ListReq, ListResp, RemoveReq, RemoveResp, SyncReq, SyncResp,
    UpdateReq,
};
use crate::grpc::transport::Transport;

/// FeedService.
pub struct Feeds {
    transport: Transport,
}

impl Feeds {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Create feed.
    pub async fn create(
        &self,
        connection: &str,
        remote: &str,
        name: &str,
        color: Option<String>,
        block: bool,
    ) -> Result<Feed, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client
            .create(CreateReq {
                connection: connection.to_string(),
                remote: remote.to_string(),
                name: name.to_string(),
                color,
                block,
            })
            .await?
            .into_inner())
    }

    /// List feeds.
    pub async fn list(&self) -> Result<ListResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client.list(ListReq {}).await?.into_inner())
    }

    /// Get feed.
    pub async fn retrieve(&self, hex: &str) -> Result<Feed, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client
            .get(GetReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Update feed.
    pub async fn update(
        &self,
        hex: &str,
        color: Option<String>,
        block: Option<bool>,
        active: Option<bool>,
        name: Option<String>,
    ) -> Result<Feed, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client
            .update(UpdateReq {
                hex: hex.to_string(),
                color,
                block,
                active,
                name,
            })
            .await?
            .into_inner())
    }

    /// Remove feed.
    pub async fn remove(&self, hex: &str) -> Result<RemoveResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client
            .remove(RemoveReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Sync feed.
    pub async fn sync(&self, hex: &str) -> Result<SyncResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.feeds.clone();
        Ok(client
            .sync(SyncReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner())
    }
}
