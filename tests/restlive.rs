//! Live REST tests — require `HERMERS_API_KEY`.

use hermers::types::{ContactData, Query};
use hermers::Hermes;

fn api_key() -> Option<String> {
    std::env::var("HERMERS_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tokio::test]
async fn live_ready_whoami_and_contacts() {
    let Some(key) = api_key() else {
        eprintln!("skipping live REST test: HERMERS_API_KEY unset");
        return;
    };
    let hermes = Hermes::new(key).expect("construct");
    assert_eq!(
        hermes.http.api_base,
        "https://hermers.aduki.pro/v1",
        "production base"
    );

    let id = hermes.ready().await.expect("whoami");
    assert!(!id.user.is_empty());
    assert!(!id.tenant.is_empty());
    assert!(hermes.me().is_some());
    let again = hermes.whoami().await.expect("cached whoami");
    assert_eq!(again.user, id.user);
    assert_eq!(again.tenant, id.tenant);

    let page = hermes
        .contacts
        .list(Some(Query {
            limit: Some(10),
            ..Default::default()
        }))
        .await
        .expect("contacts.list");
    assert!(page.total >= 0);

    // create + cleanup
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("rs-sdk-live-{stamp}");
    let vcard = format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN:{name}\nEMAIL:rs-sdk-live@example.com\nEND:VCARD"
    );
    let created = hermes
        .contacts
        .create(ContactData {
            name: name.clone(),
            vcard,
            emails: Some(vec!["rs-sdk-live@example.com".into()]),
            phones: None,
            groups: None,
            meta: Some(serde_json::json!({})),
        })
        .await
        .expect("contacts.create");
    assert!(!created.hex.is_empty());
    hermes
        .contacts
        .remove(&created.hex)
        .await
        .expect("contacts.remove");
}
