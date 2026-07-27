//! SessionService resource (subset).

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::session::{
    ListSessionsReq, ListSessionsResp, LoadReq, RevokeReq, Session, WhoamiReq,
};
use crate::grpc::transport::Transport;

/// SessionService — whoami/load/revoke/list only (no Issue/Login/Refresh).
///
/// Named `Sessions` (not `Session`) to avoid clashing with the pb `Session` message.
pub struct Sessions {
    transport: Transport,
}

impl Sessions {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Whoami (uses cache when warm).
    pub async fn whoami(&self) -> Result<Session, HermesGrpcError> {
        let id = self.transport.whoami().await?;
        if let Some(raw) = id.raw {
            return Ok(raw);
        }
        let mut client = self.transport.session.clone();
        Ok(client
            .whoami(WhoamiReq {
                token: String::new(),
            })
            .await?
            .into_inner())
    }

    /// Load by JTI.
    pub async fn load(&self, jti: &str) -> Result<Session, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.session.clone();
        Ok(client
            .load(LoadReq {
                jti: jti.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Revoke by JTI.
    pub async fn revoke(&self, jti: &str) -> Result<(), HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.session.clone();
        client
            .revoke(RevokeReq {
                jti: jti.to_string(),
            })
            .await?;
        Ok(())
    }

    /// List sessions for the authenticated user.
    pub async fn list(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListSessionsResp, HermesGrpcError> {
        let user = self.transport.require_user().await?;
        let mut client = self.transport.session.clone();
        Ok(client
            .list(ListSessionsReq {
                user,
                page: page.unwrap_or(1),
                limit: limit.unwrap_or(50),
                after: after.unwrap_or("").to_string(),
            })
            .await?
            .into_inner())
    }
}
