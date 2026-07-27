//! Contacts resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{Contact, ContactData, ContactModel, Page, Query};

/// CardDAV contacts CRUD.
pub struct Contacts {
    http: Client,
}

impl Contacts {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Create a contact.
    pub async fn create(&self, data: ContactData) -> Result<ContactModel, HermesError> {
        let mut body = serde_json::to_value(&data)
            .map_err(|e| HermesError::new(e.to_string(), 0, "serialize_error"))?;
        if body.get("meta").is_none() {
            body["meta"] = json!({});
        }
        self.http.post("/user/contacts", &body, None).await
    }

    /// List contacts.
    pub async fn list(&self, query: Option<Query>) -> Result<Page<Contact>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/contacts", opts.as_ref()).await
    }

    /// List by group.
    pub async fn group(&self, group: &str) -> Result<Page<Contact>, HermesError> {
        self.http
            .get(&format!("/user/contacts/group/{group}"), None)
            .await
    }

    /// Search contacts.
    pub async fn search(&self, q: &str) -> Result<Page<Contact>, HermesError> {
        self.http
            .get(&format!("/user/contacts/search/{q}"), None)
            .await
    }

    /// Patch vCard.
    pub async fn update_vcard(
        &self,
        hex: &str,
        vcard: &str,
        name: Option<&str>,
    ) -> Result<Value, HermesError> {
        let mut body = json!({ "vcard": vcard });
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        self.http
            .patch(&format!("/user/contacts/{hex}/vcard"), &body, None)
            .await
    }

    /// Patch emails.
    pub async fn update_emails(&self, hex: &str, emails: &[String]) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/user/contacts/{hex}/emails"),
                &json!({ "emails": emails }),
                None,
            )
            .await
    }

    /// Patch phones.
    pub async fn update_phones(&self, hex: &str, phones: &[String]) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/user/contacts/{hex}/phones"),
                &json!({ "phones": phones }),
                None,
            )
            .await
    }

    /// Patch groups.
    pub async fn update_groups(&self, hex: &str, groups: &[String]) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/user/contacts/{hex}/groups"),
                &json!({ "groups": groups }),
                None,
            )
            .await
    }

    /// Patch meta.
    pub async fn update_meta(&self, hex: &str, meta: &Value) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/user/contacts/{hex}/meta"),
                &json!({ "meta": meta }),
                None,
            )
            .await
    }

    /// Delete a contact.
    pub async fn remove(&self, hex: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/user/contacts/{hex}"), None)
            .await
    }
}
