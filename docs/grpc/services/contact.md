# Contact (`client.contacts`)

Wraps `hermes.contact.ContactService`. `tenant` / `owner` on Create, List, and Sync are injected from the whoami cache.

**Unlike REST:** gRPC exposes `retrieve` (`Get`) and create only needs `vcard` (no required `name` / `meta`). The returned `Contact` is the gRPC message (`tenant`, `owner`, `vcard`, `etag`, `created`/`updated`) — not the REST create response or list row.

```rust
let client = hermers::HermesGrpc::connect(std::env::var("HERMERS_API_KEY").unwrap()).await?;
client.ready().await?;
let list = client.contacts.list(None, Some(50)).await?;
let contact = client.contacts.create("BEGIN:VCARD\nVERSION:4.0\nFN:Ada\nEND:VCARD").await?;
```

## Methods

| Method | Signature | RPC | Returns |
| --- | --- | --- | --- |
| `list` | `({ cursor?, limit? }?) → Result<ListResp>` | `List` | Items + cursor |
| `retrieve` | `(hex: string) → Result<Contact>` | `Get` | Contact |
| `create` | `({ vcard }) → Result<Contact>` | `Create` | Contact |
| `update` | `({ hex, vcard, etag }) → Result<Contact>` | `Update` | Contact |
| `remove` | `(hex: string) → Result<void>` | `Remove` | Empty |
| `sync` | `({ since: Date }) → Result<SyncResp>` | `Sync` | Delta |

### Caller-facing params

| Method | Fields you pass | Injected by SDK |
| --- | --- | --- |
| `list` | `cursor?`, `limit?` (default 50) | `tenant` |
| `create` | `vcard` | `tenant`, `owner` |
| `update` | `hex`, `vcard`, `etag` | — |
| `sync` | `since` | `tenant` |

## Return types

### `Contact`

| Field | Type | Description |
| --- | --- | --- |
| `hex` | `string` | Contact id |
| `tenant` | `string` | Tenant hex |
| `owner` | `string` | Owner user hex |
| `vcard` | `string` | Raw vCard 4.0 |
| `etag` | `string` | Concurrency token |
| `created` | `Date?` | |
| `updated` | `Date?` | |

### `ListResp`

```json
{ items: Contact[]; next: string }
```

### `SyncResp`

```json
{ changed: Contact[]; removed: string[] }
```

### Example

```json
{
  hex: 'C0X…',
  tenant: 'T0X…',
  owner: 'U0X…',
  vcard: 'BEGIN:VCARD\nVERSION:4.0\nFN:Ada\nEND:VCARD',
  etag: '"1"',
  created: new Date('…'),
  updated: new Date('…'),
}
```

## Errors

`HermesGrpcError` — e.g. `NOT_FOUND`, `FAILED_PRECONDITION` (etag mismatch), `PERMISSION_DENIED`.
