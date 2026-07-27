# Events (`hermes.events`)

Calendar events under `/user/events`.

```rust
use hermers::types::Query;
use serde_json::json;

let page = hermes.events.list(Some(Query { limit: Some(50), ..Default::default() })).await?;
let created = hermes.events.create(&json!({
    "calendar": "L0X…",
    "uid": "evt-1@example.com",
    "ical": "BEGIN:VCALENDAR\n…\nEND:VCALENDAR",
    "summary": "Sync",
    "start": "2026-08-01T10:00:00",
    "end": "2026-08-01T11:00:00",
})).await?;
// { hex, etag, uid }
```

## Methods

| SDK | HTTP | Returns |
| --- | --- | --- |
| `list` / `range` / `recurring` / `search` / `upcoming` / `past` | `GET /user/events…` | `Page<Event>` |
| `create(body)` | `POST /user/events` | JSON `{ hex, etag, uid }` |
| `update(hex, body)` | `PATCH /user/events/{hex}` | JSON `{ hex, etag, uid }` |
| `remove(hex)` | `DELETE /user/events/{hex}` | JSON `null` |

### Path details

| Method | Path |
| --- | --- |
| `list` | `GET /user/events` |
| `range(start, end)` | `GET /user/events/range/{start}/{end}` |
| `recurring` | `GET /user/events/recurring` |
| `search(q)` | `GET /user/events/search/{q}` |
| `upcoming` | `GET /user/events/upcoming` |
| `past` | `GET /user/events/past` |

## Create request

| Field | Type | Required |
| --- | --- | --- |
| `calendar` | string | yes |
| `uid` | string | yes |
| `ical` | string | yes |
| `href` | string | no |
| `start` / `end` | datetime string | no |
| `summary` / `description` / `location` / `kind` / `rrule` / `timezone` | string | no |
| `attendees` | string[] | no |
| `recurring` | boolean | no |

## List item (`Event`)

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `uid` | string | no |
| `start` / `end` | datetime | yes |
| `created` | datetime | no |
| `total` | number | yes |

List rows do **not** include `summary` / `ical` / `attendees`.

## Create / update response

```json
{ "hex": "E0X…", "etag": "\"1\"", "uid": "evt-1@example.com" }
```

## Errors

`{ "error": "…", "message": "…" }` → `HermesError`. See [Types](../../types/index.md).
