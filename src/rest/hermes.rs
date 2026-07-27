//! Root REST client.

use std::sync::Arc;

use crate::rest::calendar::Calendar;
use crate::rest::config::Config;
use crate::rest::contacts::Contacts;
use crate::rest::error::HermesError;
use crate::rest::events::Events;
use crate::rest::feeds::Feeds;
use crate::rest::http::{Backend, Client, Identity};
use crate::rest::keys::Keys;
use crate::rest::mail::Mail;
use crate::rest::scheduling::Scheduling;
use crate::rest::tenant::Tenant;
use crate::rest::user::User;

/// Hermes REST client (Stripe/Square style).
///
/// On construction (inside a Tokio runtime), starts `GET /auth/whoami` and caches
/// user/tenant. Resource methods never require tenant/user hex arguments.
pub struct Hermes {
    /// Shared HTTP client.
    pub http: Client,
    /// Contacts.
    pub contacts: Contacts,
    /// Mail.
    pub mail: Mail,
    /// API keys.
    pub keys: Keys,
    /// User profile.
    pub user: User,
    /// Tenant admin.
    pub tenant: Tenant,
    /// Calendars.
    pub calendar: Calendar,
    /// Events.
    pub events: Events,
    /// Feeds.
    pub feeds: Feeds,
    /// Scheduling.
    pub scheduling: Scheduling,
}

impl Hermes {
    /// Construct with the default reqwest backend.
    pub fn new(api_key: impl Into<String>) -> Result<Self, HermesError> {
        Self::with_options(api_key, Config::default())
    }

    /// Construct with options.
    pub fn with_options(api_key: impl Into<String>, options: Config) -> Result<Self, HermesError> {
        let http = Client::new(api_key, options)?;
        Ok(Self::from_http(http))
    }

    /// Construct with a custom HTTP backend (unit tests).
    pub fn with_backend(
        api_key: impl Into<String>,
        options: Config,
        backend: Arc<dyn Backend>,
    ) -> Result<Self, HermesError> {
        let http = Client::with_backend(api_key, options, backend)?;
        Ok(Self::from_http(http))
    }

    fn from_http(http: Client) -> Self {
        Self {
            contacts: Contacts::new(http.clone()),
            mail: Mail::new(http.clone()),
            keys: Keys::new(http.clone()),
            user: User::new(http.clone()),
            tenant: Tenant::new(http.clone()),
            calendar: Calendar::new(http.clone()),
            events: Events::new(http.clone()),
            feeds: Feeds::new(http.clone()),
            scheduling: Scheduling::new(http.clone()),
            http,
        }
    }

    /// Cached identity after whoami resolves.
    pub fn me(&self) -> Option<Identity> {
        self.http.me()
    }

    /// Await until `GET /auth/whoami` has populated the identity cache.
    pub async fn ready(&self) -> Result<Identity, HermesError> {
        self.http.ready().await
    }

    /// Resolve (and cache) the authenticated identity.
    pub async fn whoami(&self) -> Result<Identity, HermesError> {
        self.http.whoami().await
    }
}
