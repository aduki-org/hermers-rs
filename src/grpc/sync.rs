//! SyncService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::sync::{ContactSyncReq, ContactSyncResp, MailboxSyncReq, MailboxSyncResp};
use crate::grpc::transport::Transport;
use prost_types::Timestamp;

/// SyncService.
pub struct Sync {
    transport: Transport,
}

impl Sync {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// Sync contacts since unix seconds.
    pub async fn contacts(&self, since_unix: i64) -> Result<ContactSyncResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.sync.clone();
        Ok(client
            .contacts(ContactSyncReq {
                tenant,
                since: Some(Timestamp {
                    seconds: since_unix,
                    nanos: 0,
                }),
            })
            .await?
            .into_inner())
    }

    /// Sync mailbox.
    pub async fn mailboxes(
        &self,
        mailbox: &str,
        knownuidvalidity: Option<u32>,
        knownmodseq: Option<u64>,
    ) -> Result<MailboxSyncResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.sync.clone();
        Ok(client
            .mailboxes(MailboxSyncReq {
                mailbox: mailbox.to_string(),
                known_uidvalidity: knownuidvalidity.unwrap_or(0),
                known_modseq: knownmodseq.unwrap_or(0),
            })
            .await?
            .into_inner())
    }
}
