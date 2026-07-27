//! # hermers
//!
//! Open-source Rust clients for Hermers. The Hermers **server is private /
//! proprietary** — this crate describes the public client APIs only.
//!
//! | Client | Feature | Default endpoint |
//! | --- | --- | --- |
//! | [`Hermes`] | `rest` | `https://hermers.aduki.pro/v1` |
//! | [`HermesGrpc`] | `grpc` | `grpc.aduki.pro:443` (TLS) |
//!
//! Authentication is **API key only**: `Authorization: Key <key>`.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

#[cfg(feature = "rest")]
pub mod rest;

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "rest")]
#[cfg_attr(docsrs, doc(cfg(feature = "rest")))]
pub use rest::{
    crypto::{generate_key, hash_key, prefix_key},
    errors::HermesError,
    hermes::Hermes,
    http::{HttpClient, Identity as RestIdentity, RequestOptions},
    config::{HermesOptions, BASE_URL},
    types,
};

#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub use grpc::{
    config::{HermesGrpcOptions, BASE_ENDPOINT},
    errors::HermesGrpcError,
    hermes::HermesGrpc,
    transport::{GrpcTransport, Identity as GrpcIdentity},
};
