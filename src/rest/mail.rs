//! Mail resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{Hex, Mailbox, MailboxData, MailboxModel, Message, Page, Query, Thread};

/// Messages + mailboxes.
pub struct Mail {
    http: Client,
}

impl Mail {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Send a simple message.
    pub async fn send(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        text: &str,
    ) -> Result<Hex, HermesError> {
        self.http
            .post(
                "/user/mail/send",
                &json!({ "from": from, "to": to, "subject": subject, "text": text }),
                None,
            )
            .await
    }

    /// Inbox.
    pub async fn inbox(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/inbox", opts.as_ref()).await
    }

    /// Unread inbox.
    pub async fn unread(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/inbox/unread", opts.as_ref()).await
    }

    /// Flagged inbox.
    pub async fn flagged(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/inbox/flagged", opts.as_ref()).await
    }

    /// Messages with attachments.
    pub async fn attachments(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/user/mail/inbox/attachments", opts.as_ref())
            .await
    }

    /// Inbox by sender.
    pub async fn by_sender(
        &self,
        sender: &str,
        query: Option<Query>,
    ) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(&format!("/user/mail/inbox/sender/{sender}"), opts.as_ref())
            .await
    }

    /// Sent.
    pub async fn sent(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/sent", opts.as_ref()).await
    }

    /// Sent by recipient.
    pub async fn by_recipient(
        &self,
        recipient: &str,
        query: Option<Query>,
    ) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/user/mail/sent/recipient/{recipient}"),
                opts.as_ref(),
            )
            .await
    }

    /// Drafts.
    pub async fn drafts(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/draft", opts.as_ref()).await
    }

    /// Trash.
    pub async fn trash(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/trash", opts.as_ref()).await
    }

    /// Starred.
    pub async fn starred(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/starred", opts.as_ref()).await
    }

    /// Spam.
    pub async fn spam(&self, query: Option<Query>) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/spam", opts.as_ref()).await
    }

    /// Spam by score.
    pub async fn scored(&self, score: &str) -> Result<Page<Message>, HermesError> {
        self.http
            .get(&format!("/user/mail/spam/scored/{score}"), None)
            .await
    }

    /// Folder.
    pub async fn folder(
        &self,
        folder: &str,
        query: Option<Query>,
    ) -> Result<Page<Message>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(&format!("/user/mail/folder/{folder}"), opts.as_ref())
            .await
    }

    /// Search.
    pub async fn search(&self, q: &str) -> Result<Page<Message>, HermesError> {
        self.http
            .get(&format!("/user/mail/search/{q}"), None)
            .await
    }

    /// Advanced search.
    pub async fn search_advanced(
        &self,
        q: &str,
        mailbox: Option<&str>,
        sender: Option<&str>,
    ) -> Result<Page<Message>, HermesError> {
        let mut body = json!({});
        if let Some(m) = mailbox {
            body["mailbox"] = json!(m);
        }
        if let Some(s) = sender {
            body["sender"] = json!(s);
        }
        self.http
            .post(&format!("/user/mail/search/{q}/advanced"), &body, None)
            .await
    }

    /// Threads.
    pub async fn threads(&self, query: Option<Query>) -> Result<Page<Thread>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mail/threads", opts.as_ref()).await
    }

    /// Thread messages.
    pub async fn thread(&self, thread: &str) -> Result<Page<Message>, HermesError> {
        self.http
            .get(&format!("/user/mail/thread/{thread}"), None)
            .await
    }

    /// Delete message.
    pub async fn remove(&self, hex: &str) -> Result<Value, HermesError> {
        self.http.delete(&format!("/user/mail/{hex}"), None).await
    }

    /// Clear mailbox.
    pub async fn clear_mailbox(&self, mailbox: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/user/mail/mailbox/{mailbox}"), None)
            .await
    }

    /// Update flags.
    pub async fn update_flags(
        &self,
        hex: &str,
        add: Option<&[String]>,
        remove: Option<&[String]>,
    ) -> Result<Value, HermesError> {
        let mut body = json!({});
        if let Some(a) = add {
            body["add"] = json!(a);
        }
        if let Some(r) = remove {
            body["remove"] = json!(r);
        }
        self.http
            .patch(&format!("/user/mail/{hex}/flags"), &body, None)
            .await
    }

    /// Create mailbox.
    pub async fn create_mailbox(&self, data: MailboxData) -> Result<MailboxModel, HermesError> {
        self.http.post("/user/mailbox", &data, None).await
    }

    /// List mailboxes.
    pub async fn list_mailboxes(&self, query: Option<Query>) -> Result<Page<Mailbox>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/mailbox", opts.as_ref()).await
    }

    /// Unread mailboxes.
    pub async fn unread_mailboxes(&self) -> Result<Page<Mailbox>, HermesError> {
        self.http.get("/user/mailbox/unread", None).await
    }

    /// Empty mailboxes.
    pub async fn empty_mailboxes(&self) -> Result<Page<Mailbox>, HermesError> {
        self.http.get("/user/mailbox/empty", None).await
    }

    /// Mailbox by name.
    pub async fn mailbox_by_name(&self, name: &str) -> Result<Page<Mailbox>, HermesError> {
        self.http
            .get(&format!("/user/mailbox/name/{name}"), None)
            .await
    }

    /// Search mailboxes.
    pub async fn search_mailboxes(&self, q: &str) -> Result<Page<Mailbox>, HermesError> {
        self.http
            .get(&format!("/user/mailbox/search/{q}"), None)
            .await
    }

    /// Update mailbox basic fields.
    pub async fn update_mailbox(&self, hex: &str, data: &Value) -> Result<MailboxModel, HermesError> {
        self.http
            .patch(&format!("/user/mailbox/{hex}/basic"), data, None)
            .await
    }

    /// Rename mailbox.
    pub async fn rename_mailbox(&self, hex: &str, name: &str) -> Result<MailboxModel, HermesError> {
        self.http
            .patch(
                &format!("/user/mailbox/{hex}/name"),
                &json!({ "name": name }),
                None,
            )
            .await
    }

    /// Delete mailbox.
    pub async fn delete_mailbox(&self, hex: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/user/mailbox/{hex}"), None)
            .await
    }
}
