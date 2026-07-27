//! SpamService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::spam::{ClassifyReq, ClassifyResp, ReportReq, Verdict};
use crate::grpc::transport::Transport;

/// SpamService.
pub struct Spam {
    transport: Transport,
}

impl Spam {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Classify a message.
    pub async fn classify(
        &self,
        msg: &str,
        raw: Vec<u8>,
        direction: &str,
    ) -> Result<ClassifyResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.spam.clone();
        Ok(client
            .classify(ClassifyReq {
                tenant,
                msg: msg.to_string(),
                raw,
                direction: direction.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Report a verdict.
    pub async fn report(
        &self,
        msg: &str,
        verdict: Verdict,
        source: &str,
    ) -> Result<(), HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let user = self.transport.require_user().await?;
        let mut client = self.transport.spam.clone();
        client
            .report(ReportReq {
                tenant,
                msg: msg.to_string(),
                user,
                verdict: verdict as i32,
                source: source.to_string(),
            })
            .await?;
        Ok(())
    }
}
