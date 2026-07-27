//! gRPC unit tests (no network).

use hermers::grpc::config::{Config, BASE_ENDPOINT};
use hermers::grpc::error::HermesGrpcError;
use hermers::HermesGrpc;

#[test]
fn base_endpoint_is_production() {
    assert_eq!(BASE_ENDPOINT, "grpc.aduki.pro:443");
}

#[test]
fn empty_api_key_rejected() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = rt.block_on(HermesGrpc::connect(""));
    assert!(matches!(result, Err(e) if e.code == "INVALID_ARGUMENT"));
}

#[test]
fn grpc_error_display() {
    let err = HermesGrpcError::new("boom", "NOT_FOUND");
    assert!(err.to_string().contains("boom"));
    assert!(err.to_string().contains("NOT_FOUND"));
}

#[test]
fn options_default_insecure_false() {
    let opts = Config::default();
    assert!(!opts.insecure);
    assert!(opts.endpoint.is_none());
}
