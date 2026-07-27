//! Root gRPC client.

use crate::grpc::config::Config;
use crate::grpc::contacts::Contacts;
use crate::grpc::error::HermesGrpcError;
use crate::grpc::feeds::Feeds;
use crate::grpc::mail::Mail;
use crate::grpc::security::Security;
use crate::grpc::session::Sessions;
use crate::grpc::spam::Spam;
use crate::grpc::storage::Storage;
use crate::grpc::sync::Sync;
use crate::grpc::tier::Tier;
use crate::grpc::transport::{Identity, Transport};
use crate::grpc::usage::Usage;

/// Hermes native gRPC client (Stripe/Square style).
///
/// Default endpoint: `grpc.aduki.pro:443` (TLS).
/// On connect, starts `SessionService.Whoami` and caches user/tenant.
/// Resource methods never require tenant/user hex arguments.
pub struct HermesGrpc {
    /// Shared transport.
    pub transport: Transport,
    /// Contacts.
    pub contacts: Contacts,
    /// Mail.
    pub mail: Mail,
    /// Feeds.
    pub feeds: Feeds,
    /// Storage.
    pub storage: Storage,
    /// Sync.
    pub sync: Sync,
    /// Security.
    pub security: Security,
    /// Spam.
    pub spam: Spam,
    /// Tier.
    pub tier: Tier,
    /// Usage.
    pub usage: Usage,
    /// Sessions.
    pub sessions: Sessions,
}

impl HermesGrpc {
    /// Connect with defaults.
    pub async fn connect(api_key: impl Into<String>) -> Result<Self, HermesGrpcError> {
        Self::connect_with(api_key, Config::default()).await
    }

    /// Connect with options.
    pub async fn connect_with(
        api_key: impl Into<String>,
        options: Config,
    ) -> Result<Self, HermesGrpcError> {
        let transport = Transport::connect(api_key, options).await?;
        Ok(Self::from_transport(transport))
    }

    fn from_transport(transport: Transport) -> Self {
        Self {
            contacts: Contacts::new(transport.clone()),
            mail: Mail::new(transport.clone()),
            feeds: Feeds::new(transport.clone()),
            storage: Storage::new(transport.clone()),
            sync: Sync::new(transport.clone()),
            security: Security::new(transport.clone()),
            spam: Spam::new(transport.clone()),
            tier: Tier::new(transport.clone()),
            usage: Usage::new(transport.clone()),
            sessions: Sessions::new(transport.clone()),
            transport,
        }
    }

    /// Cached identity.
    pub fn me(&self) -> Option<Identity> {
        self.transport.me()
    }

    /// Await whoami cache.
    pub async fn ready(&self) -> Result<Identity, HermesGrpcError> {
        self.transport.ready().await
    }

    /// Resolve whoami.
    pub async fn whoami(&self) -> Result<Identity, HermesGrpcError> {
        self.transport.whoami().await
    }
}
