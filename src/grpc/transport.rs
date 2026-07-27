//! Shared gRPC transport: TLS by default, API-key metadata, whoami cache.

use std::sync::{Arc, Mutex};

use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status};

use crate::grpc::config::{Config, BASE_ENDPOINT};
use crate::grpc::error::HermesGrpcError;
use crate::grpc::pb::{
    contact::contact_service_client::ContactServiceClient,
    feeds::feed_service_client::FeedServiceClient,
    mail::mail_service_client::MailServiceClient,
    security::security_service_client::SecurityServiceClient,
    session::session_service_client::SessionServiceClient,
    session::{Session, WhoamiReq},
    spam::spam_service_client::SpamServiceClient,
    storage::storage_service_client::StorageServiceClient,
    sync::sync_service_client::SyncServiceClient,
    tier::tier_service_client::TierServiceClient,
    usage::usageervice_client::UsageerviceClient,
};

/// Cached identity from `SessionService.Whoami`.
#[derive(Debug, Clone)]
pub struct Identity {
    /// Session / JTI hex.
    pub hex: Option<String>,
    /// Authenticated user hex.
    pub user: String,
    /// Authenticated tenant hex.
    pub tenant: String,
    /// Owner flag.
    pub owner: Option<bool>,
    /// Scopes.
    pub scopes: Option<Vec<String>>,
    /// Deny list.
    pub deny: Option<Vec<String>>,
    /// Tier.
    pub tier: Option<String>,
    /// Raw session message.
    pub raw: Option<Session>,
}

/// Injects `authorization: Key …` on every outbound gRPC call.
#[derive(Clone)]
pub struct AuthGuard {
    auth: MetadataValue<Ascii>,
}

impl Interceptor for AuthGuard {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.auth.clone());
        Ok(request)
    }
}

type Authed<T> = tonic::service::interceptor::InterceptedService<T, AuthGuard>;

/// Shared gRPC transport.
#[derive(Clone)]
pub struct Transport {
    /// Endpoint `host:port`.
    pub endpoint: String,
    /// API key.
    pub api_key: String,
    pub(crate) session: SessionServiceClient<Authed<Channel>>,
    pub(crate) contacts: ContactServiceClient<Authed<Channel>>,
    pub(crate) mail: MailServiceClient<Authed<Channel>>,
    pub(crate) feeds: FeedServiceClient<Authed<Channel>>,
    pub(crate) storage: StorageServiceClient<Authed<Channel>>,
    pub(crate) sync: SyncServiceClient<Authed<Channel>>,
    pub(crate) security: SecurityServiceClient<Authed<Channel>>,
    pub(crate) spam: SpamServiceClient<Authed<Channel>>,
    pub(crate) tier: TierServiceClient<Authed<Channel>>,
    pub(crate) usage: UsageerviceClient<Authed<Channel>>,
    identity_cache: Arc<Mutex<Option<Identity>>>,
    whoami_lock: Arc<tokio::sync::Mutex<()>>,
}

impl Transport {
    /// Connect and build clients. When inside a Tokio runtime, starts whoami eagerly.
    pub async fn connect(
        api_key: impl Into<String>,
        options: Config,
    ) -> Result<Self, HermesGrpcError> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(HermesGrpcError::new(
                "API key is required (e.g. hm_live_...)",
                "INVALID_ARGUMENT",
            ));
        }
        let endpoint = options
            .endpoint
            .unwrap_or_else(|| BASE_ENDPOINT.to_string());

        let channel = build_channel(&endpoint, options.insecure).await?;
        let auth: MetadataValue<Ascii> = format!("Key {api_key}")
            .parse()
            .map_err(|_| {
                HermesGrpcError::new("invalid API key for metadata", "INVALID_ARGUMENT")
            })?;
        let interceptor = AuthGuard { auth };

        let transport = Self {
            endpoint,
            api_key,
            session: SessionServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            contacts: ContactServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            mail: MailServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            feeds: FeedServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            storage: StorageServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            sync: SyncServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            security: SecurityServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            spam: SpamServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            tier: TierServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            usage: UsageerviceClient::with_interceptor(channel, interceptor),
            identity_cache: Arc::new(Mutex::new(None)),
            whoami_lock: Arc::new(tokio::sync::Mutex::new(())),
        };
        transport.maybe_spawn_eager_whoami();
        Ok(transport)
    }

    fn maybe_spawn_eager_whoami(&self) {
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let this = self.clone();
        handle.spawn(async move {
            let _ = this.whoami().await;
        });
    }

    /// Cached identity snapshot.
    pub fn me(&self) -> Option<Identity> {
        self.identity_cache.lock().unwrap().clone()
    }

    /// Await whoami.
    pub async fn ready(&self) -> Result<Identity, HermesGrpcError> {
        self.whoami().await
    }

    /// Resolve and cache identity.
    pub async fn whoami(&self) -> Result<Identity, HermesGrpcError> {
        if let Some(id) = self.me() {
            return Ok(id);
        }
        let _guard = self.whoami_lock.lock().await;
        if let Some(id) = self.me() {
            return Ok(id);
        }
        let id = self.fetch_whoami().await?;
        *self.identity_cache.lock().unwrap() = Some(id.clone());
        Ok(id)
    }

    async fn fetch_whoami(&self) -> Result<Identity, HermesGrpcError> {
        let mut client = self.session.clone();
        let session = client
            .whoami(WhoamiReq {
                token: String::new(),
            })
            .await?
            .into_inner();
        if session.user.is_empty() || session.tenant.is_empty() {
            return Err(HermesGrpcError::new(
                "whoami response missing user or tenant",
                "FAILED_PRECONDITION",
            ));
        }
        Ok(Identity {
            hex: Some(session.hex.clone()).filter(|s| !s.is_empty()),
            user: session.user.clone(),
            tenant: session.tenant.clone(),
            owner: Some(session.owner),
            scopes: Some(session.scopes.clone()),
            deny: Some(session.deny.clone()),
            tier: Some(session.tier.clone()).filter(|s| !s.is_empty()),
            raw: Some(session),
        })
    }

    /// Ensure identity is warm, then run `f`.
    pub async fn with_identity<T, F, Fut>(&self, f: F) -> Result<T, HermesGrpcError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, HermesGrpcError>>,
    {
        let _ = self.whoami().await?;
        f().await
    }

    /// Tenant hex from cache.
    pub async fn require_tenant(&self) -> Result<String, HermesGrpcError> {
        Ok(self.whoami().await?.tenant)
    }

    /// User hex from cache.
    pub async fn require_user(&self) -> Result<String, HermesGrpcError> {
        Ok(self.whoami().await?.user)
    }
}

async fn build_channel(endpoint: &str, insecure: bool) -> Result<Channel, HermesGrpcError> {
    let uri = if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.to_string()
    } else if insecure {
        format!("http://{endpoint}")
    } else {
        format!("https://{endpoint}")
    };

    let mut ep = Endpoint::from_shared(uri)
        .map_err(|e| HermesGrpcError::new(e.to_string(), "INVALID_ARGUMENT"))?;

    if !insecure {
        let tls = ClientTlsConfig::new().with_enabled_roots();
        ep = ep
            .tls_config(tls)
            .map_err(|e| HermesGrpcError::new(e.to_string(), "UNAVAILABLE"))?;
    }

    ep.connect()
        .await
        .map_err(|e| {
            let mut msg = format!("{e}");
            let mut src = std::error::Error::source(&e);
            while let Some(s) = src {
                msg.push_str(": ");
                msg.push_str(&s.to_string());
                src = s.source();
            }
            HermesGrpcError::new(msg, "UNAVAILABLE")
        })
}
