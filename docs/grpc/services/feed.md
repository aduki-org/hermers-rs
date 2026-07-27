# Feed (`client.feeds`)

Wraps `hermes.feeds.FeedService` — external calendar ICS feeds.

```rust
let feeds = client.feeds.list().await?;
let feed = client.feeds.create("CAL0X…", "https://example.com/cal.ics", "Holidays", None, false).await?;
```

## Methods

| Method | Signature | Returns |
| --- | --- | --- |
| `create` | `({ connection, remote, name, color?, block? }) → Result<Feed>` | Feed |
| `list` | `() → Result<ListResp>` | `{ items: Feed[] }` |
| `retrieve` | `(hex) → Result<Feed>` | Feed |
| `update` | `({ hex, color?, block?, active?, name? }) → Result<Feed>` | Feed |
| `remove` | `(hex) → Result<{ removed: boolean }>` | Ack |
| `sync` | `(hex) → Result<SyncResp>` | Sync stats |

### `create` params

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `connection` | `string` | yes | Target calendar hex |
| `remote` | `string` | yes | HTTPS URL of `.ics` |
| `name` | `string` | yes | Display name |
| `color` | `string` | no | |
| `block` | `boolean` | no | Default `false` |

## Return types

### `Feed`

| Field | Type | Description |
| --- | --- | --- |
| `hex` | `string` | Feed id |
| `tenant` | `string` | |
| `user` | `string` | |
| `connection` | `string` | Calendar hex |
| `remote` | `string` | ICS URL |
| `name` | `string` | |
| `color` | `string?` | |
| `block` | `boolean` | |
| `active` | `boolean` | |
| `last` | `string?` | ISO-8601 last sync |

### `SyncResp`

```json
{ ok: boolean; inserted: number; updated: number }
```

### `RemoveResp`

```json
{ removed: boolean }
```

## Errors

Invalid URL / calendar → `INVALID_ARGUMENT` / `NOT_FOUND`. Throws `HermesGrpcError`.
