//! MailService resource.

use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::mail::{
    CreateMailboxReq, DeleteMailboxReq, ExpungeReq, ExpungeResp, Flag, GetMessageReq,
    ListMailboxesReq, ListMailboxesResp, ListMessagesReq, ListMessagesResp, Mailbox, Message,
    MoveReq, MoveResp, SendReq, SendResp, SetFlagsReq, UpdateMailboxReq,
};
use crate::grpc::transport::Transport;

/// MailService.
pub struct Mail {
    transport: Transport,
}

impl Mail {
    pub(crate) fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// List mailboxes.
    pub async fn list_mailboxes(&self) -> Result<ListMailboxesResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let owner = self.transport.require_user().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .list_mailboxes(ListMailboxesReq { tenant, owner })
            .await?
            .into_inner())
    }

    /// List messages in a mailbox.
    pub async fn list_messages(
        &self,
        mailbox: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ListMessagesResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .list_messages(ListMessagesReq {
                mailbox: mailbox.to_string(),
                cursor: cursor.unwrap_or("").to_string(),
                limit: limit.unwrap_or(50),
            })
            .await?
            .into_inner())
    }

    /// Get message.
    pub async fn retrieve(&self, hex: &str) -> Result<Message, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .get_message(GetMessageReq {
                hex: hex.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Send raw RFC822.
    pub async fn send(
        &self,
        from: &str,
        to: Vec<String>,
        raw: Vec<u8>,
    ) -> Result<SendResp, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .send(SendReq {
                tenant,
                from: from.to_string(),
                to,
                raw,
            })
            .await?
            .into_inner())
    }

    /// Move message.
    pub async fn relocate(&self, hex: &str, dest: &str) -> Result<MoveResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .r#move(MoveReq {
                hex: hex.to_string(),
                dest: dest.to_string(),
            })
            .await?
            .into_inner())
    }

    /// Set flags.
    pub async fn set_flags(
        &self,
        hex: &str,
        add: Vec<Flag>,
        remove: Vec<Flag>,
    ) -> Result<(), HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        client
            .set_flags(SetFlagsReq {
                hex: hex.to_string(),
                add: add.into_iter().map(|f| f as i32).collect(),
                remove: remove.into_iter().map(|f| f as i32).collect(),
            })
            .await?;
        Ok(())
    }

    /// Expunge UIDs.
    pub async fn expunge(
        &self,
        mailbox: &str,
        uids: Vec<u32>,
    ) -> Result<ExpungeResp, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .expunge(ExpungeReq {
                mailbox: mailbox.to_string(),
                uids,
            })
            .await?
            .into_inner())
    }

    /// Create mailbox.
    pub async fn create_mailbox(
        &self,
        name: &str,
        role: Option<&str>,
    ) -> Result<Mailbox, HermesGrpcError> {
        let tenant = self.transport.require_tenant().await?;
        let owner = self.transport.require_user().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .create_mailbox(CreateMailboxReq {
                tenant,
                owner,
                name: name.to_string(),
                role: role.unwrap_or("").to_string(),
            })
            .await?
            .into_inner())
    }

    /// Delete mailbox.
    pub async fn delete_mailbox(&self, hex: &str) -> Result<(), HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        client
            .delete_mailbox(DeleteMailboxReq {
                hex: hex.to_string(),
            })
            .await?;
        Ok(())
    }

    /// Update mailbox.
    pub async fn update_mailbox(
        &self,
        hex: &str,
        name: Option<&str>,
        role: Option<&str>,
    ) -> Result<Mailbox, HermesGrpcError> {
        let _ = self.transport.whoami().await?;
        let mut client = self.transport.mail.clone();
        Ok(client
            .update_mailbox(UpdateMailboxReq {
                hex: hex.to_string(),
                name: name.unwrap_or("").to_string(),
                role: role.unwrap_or("").to_string(),
            })
            .await?
            .into_inner())
    }
}
