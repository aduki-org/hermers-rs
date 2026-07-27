//! TierService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::tier::{ChangeReq, ChangeResp, Plan, ResolveReq, TierInfo};
use crate::grpc::transport::Transport;

/// TierService.
pub struct Tier {
    transport: Transport,
}

impl Tier {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Resolve current tier.
    pub async fn resolve(&self) -> Result<TierInfo, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.tier.clone();
        Ok(client
            .resolve(ResolveReq { tenant })
            .await?
            .into_inner())
    }

    /// Change plan.
    pub async fn change(
        &self,
        plan: Plan,
        payment_method: Option<&str>,
    ) -> Result<ChangeResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.tier.clone();
        Ok(client
            .change(ChangeReq {
                tenant,
                plan: plan as i32,
                payment_method: payment_method.unwrap_or("").to_string(),
            })
            .await?
            .into_inner())
    }
}
