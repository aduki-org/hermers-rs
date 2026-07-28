# Feeds (`hermes.feeds`)

External calendar ICS feeds under `/user/feeds`.

```rust
let feeds = hermes.feeds.list().await?;
let feed = hermes.feeds.create(
    "CAL0X…",
    "https://example.com/cal.ics",
    "Public holidays",
    Some("#00aa00"),
    Some(false),
).await?;
```

## Methods

| SDK | HTTP | Returns |
| --- | --- | --- |
| `create(data)` | `POST /user/feeds` | `Feed` model |
| `list()` | `GET /user/feeds` | `Feed[]` |
| `retrieve(hex)` | `GET /user/feeds/{hex}` | `Feed` model |
| `remove(hex)` | `DELETE /user/feeds/{hex}` | `null` |

(gRPC also has `update` / `sync` — see [gRPC Feed](../../grpc/services/feed.md).)

## Create request

| Field | Type | Required | Default |
| --- | --- | --- | --- |
| `connection` | string | yes | target calendar hex |
| `remote` | string | yes | HTTPS `.ics` URL |
| `name` | string | yes | |
| `color` | string | no | |
| `block` | boolean | no | `false` |

## Response — `Feed` model

| Field | Type | Nullable |
| --- | --- | --- |
| `id` | number | no |
| `hex` / `tenant` / `user` / `connection` / `remote` / `name` | string | no |
| `color` | string | yes |
| `block` / `active` | boolean | no |
| `sync` | string | yes |
| `meta` | object | no |
| `last` | datetime | yes |
| `created` / `updated` | datetime | no |

```json
{
  "id": 1,
  "hex": "F0X…",
  "tenant": "T0X…",
  "user": "U0X…",
  "connection": "L0X…",
  "remote": "https://example.com/calendar.ics",
  "name": "Holidays",
  "color": "#336699",
  "block": false,
  "sync": null,
  "active": true,
  "meta": {},
  "last": null,
  "created": "2026-07-28T12:00:00",
  "updated": "2026-07-28T12:00:00"
}
```


## Update & sync

| SDK | HTTP | Body / returns |
| --- | --- | --- |
| `update(hex, body)` | `PATCH /user/feeds/{hex}` | JSON object → `Feed` |
| `sync(hex)` | `POST /user/feeds/{hex}/sync` | `FeedSync { hex, ok }` |

```rust
hermes.feeds.update("F0X…", &serde_json::json!({ "color": "#336699", "active": true })).await?;
let synced = hermes.feeds.sync("F0X…").await?;
```

## Errors

`{ "error": "…", "message": "…" }` — see [Types](../../types/index.md).
