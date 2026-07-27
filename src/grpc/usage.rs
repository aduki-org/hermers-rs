//! Usageervice resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::usage::{CheckReq, CheckResp, GetReq, IncrReq, IncrResp, ResetReq, Usage as UsageMsg};
use crate::grpc::transport::Transport;

/// Usageervice (proto name kept).
pub struct Usage {
    transport: Transport,
}

impl Usage {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Increment a metric.
    pub async fn increment(
        &self,
        metric: &str,
        by: Option<i64>,
    ) -> Result<IncrResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.usage.clone();
        Ok(client
            .increment(IncrReq {
                tenant,
                metric: metric.to_string(),
                by: by.unwrap_or(1),
            })
            .await?
            .into_inner())
    }

    /// Check a metric.
    pub async fn check(&self, metric: &str) -> Result<CheckResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.usage.clone();
        Ok(client
            .check(CheckReq {
                tenant,
                metric: metric.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Get usage for a window.
    pub async fn get(&self, metric: &str, window: &str) -> Result<UsageMsg, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.usage.clone();
        Ok(client
            .get(GetReq {
                tenant,
                metric: metric.to_string(),
                window: window.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Reset a metric window.
    pub async fn reset(&self, metric: &str, window: &str) -> Result<(), HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.usage.clone();
        client
            .reset(ResetReq {
                tenant,
                metric: metric.to_string(),
                window: window.to_string(),
            })
            .await?;
        Ok(())
    }
}
