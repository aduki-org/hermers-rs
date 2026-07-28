# Calendar (`hermes.calendar`)

CalDAV calendars under `/user/calendars`.

```rust
use hermers::types::Query;

let page = hermes.calendar.list(Some(Query { limit: Some(50), ..Default::default() })).await?;
let created = hermes.calendar.create(
    "Work",
    None,
    Some("#336699"),
    Some("America/New_York"),
).await?;
// created: CalendarCreated { hex, etag, sync_token }
```

## Methods

| SDK | HTTP | Returns |
| --- | --- | --- |
| `list(query?)` | `GET /user/calendars` | `Page<Calendar>` |
| `search(q)` | `GET /user/calendars/search/{q}` | `Page<Calendar>` |
| `create(name, description?, color?, timezone?)` | `POST /user/calendars` | `CalendarCreated` `{ hex, etag, sync_token }` |
| `events(query?)` | `GET /user/calendars/events` | `Page<Event>` (list shape) |

## Create request

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `name` | string | yes | |
| `description` | string | no | |
| `color` | string | no | |
| `timezone` | string | no | server default `Etc/UTC` |

## Create response

```json
{ "hex": "L0X…", "etag": "…", "sync_token": "…" }
```

Rust type: `CalendarCreated` with field `sync_token` (wire key `sync_token`).

## List item (`Calendar`)

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `name` / `timezone` | string | no |
| `description` / `color` | string | yes |
| `created` | datetime | no |
| `total` | number | yes (list window) |


## Update & delete

| SDK | HTTP | Body / returns |
| --- | --- | --- |
| `update(hex, body)` | `PATCH /user/calendars/{hex}` | JSON → `CalendarCreated { hex, etag, sync_token }` |
| `remove(hex)` | `DELETE /user/calendars/{hex}` | JSON value / empty |

## Errors

`{ "error": "…", "message": "…" }` → `HermesError`. See [Types](../../types/index.md).
