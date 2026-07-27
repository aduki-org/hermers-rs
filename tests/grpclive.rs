//! Live gRPC tests — require `HERMERS_API_KEY`.

use hermers::HermesGrpc;

fn api_key() -> Option<String> {
    std::env::var("HERMERS_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tokio::test]
async fn live_ready_whoami_and_contacts() {
    let Some(key) = api_key() else {
        eprintln!("skipping live gRPC test: HERMERS_API_KEY unset");
        return;
    };
    let client = HermesGrpc::connect(key).await.expect("connect");
    let id = client.ready().await.expect("whoami");
    assert!(!id.user.is_empty());
    assert!(!id.tenant.is_empty());
    assert!(client.me().is_some());

    let list = client
        .contacts
        .list(None, Some(10))
        .await
        .expect("contacts.list");
    let _ = list.items.len();

    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let vcard = format!(
        "BEGIN:VCARD\nVERSION:4.0\nFN:rs-grpc-live-{stamp}\nEMAIL:rs-grpc-live@example.com\nEND:VCARD"
    );
    let created = client.contacts.create(&vcard).await.expect("contacts.create");
    assert!(!created.hex.is_empty());
    client
        .contacts
        .remove(&created.hex)
        .await
        .expect("contacts.remove");
}
