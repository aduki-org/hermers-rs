# Authentication & API keys

`hermers` (REST) uses an **API key only**. No login / password / JWT refresh in the package.

## Whoami

```http
GET /v1/auth/whoami
Authorization: Key hm_live_…
```

```rust
let hermes = hermers::Hermes::new("hm_live_…")?;
hermes.ready().await?;
// hermes.me() == Some(Identity { … })
```

### Response

| Field | Type | Notes |
| --- | --- | --- |
| `hex` | string | JTI |
| `user` | string | user hex |
| `tenant` | string | tenant hex |
| `owner` | boolean | |
| `scopes` | string[] | `"domain.scope"` flattened |
| `deny` | string[] | same |
| `tier` | string | |
| `ip` | string | always `""` |
| `agent` | string | always `""` |

```json
{
  "hex": "A0S1C3905B195668274E",
  "user": "U0X3BFF58E91EC7",
  "tenant": "T0X9E68DD4B15C6",
  "owner": true,
  "scopes": ["user.user.**", "tenant.tenant.**"],
  "deny": [],
  "tier": "free",
  "ip": "",
  "agent": ""
}
```

## API keys (`hermes.keys`)

| SDK | HTTP | Returns |
| --- | --- | --- |
| `list` | `GET /user/keys` | `Page<ApiKey>` |
| `list_tenant` | `GET /tenant/keys` | `Page<ApiKey>` |
| `list_active` | `GET /tenant/keys/active` | `Page<ApiKey>` |
| `retrieve` | `GET /tenant/keys/{hex}` | key detail / model |
| `create` | `POST /tenant/keys` | SDK: `Keypair { hex, key }`; HTTP body: `{ hex }` |
| `update_name` / `update_scopes` | `PATCH …` | ack / null |
| `remove` | `DELETE /tenant/keys/{hex}` | null |

### Create — HTTP body (what the server accepts)

| Field | Type | Required |
| --- | --- | --- |
| `name` | string | yes |
| `hash` | string | yes — SHA-256 of raw key |
| `prefix` | string | yes — ≤16 chars |
| `scopes` | (string\|null)[] | no |
| `meta` | object | no |
| `expires` | datetime string | no |

The SDK generates a raw `hm_live_…` key, sends hash+prefix, and returns `Keypair { hex, key }` once.

```rust
use hermers::{generate_key, hash_key, prefix_key};

let created = hermes.keys.create(
    "ci",
    &["contacts:read".into()],
    None,       // or Some(&generate_key())
    None,       // meta
    None,       // expires
).await?;
// created.key is shown once — store it; server only has hash_key(&created.key)
```

### List item (`ApiKey`)

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `name` / `prefix` | string | no |
| `active` | boolean | no |
| `expires` / `last` | datetime | yes |
| `created` | datetime | no |
| `tenant` | `{ hex, name }` | no |
| `user` | `{ hex, name, email }` \| null | yes |
| `total` | number | no |

Detail adds `scopes` (jsonb array/object as stored).


## Key filters & patches

| SDK | HTTP | Returns |
| --- | --- | --- |
| `list_expired()` | `GET /tenant/keys/expired` | `Page<ApiKey>` |
| `list_by_user(user)` | `GET /tenant/keys/user/{user}` | `Page<ApiKey>` |
| `lookup_prefix(prefix)` | `POST /tenant/keys/lookup/prefix` `{ prefix }` | `ApiKey` |
| `update_hash(hex, hash)` | `PATCH /tenant/keys/{hex}/hash` `{ hash }` | `Ack` / value |
| `update_last(hex, last)` | `PATCH /tenant/keys/{hex}/last` `{ last }` | `Ack` / value |

`prefix` max 16 characters. `last` is a datetime string.

## Errors

```json
{ "error": "unauthorized", "message": "unauthorized" }
```

| `error` code | HTTP |
| --- | --- |
| `unauthorized` | 401 |
| `forbidden` | 403 |
| `validation` | 422 |
| `not_found` | 404 |
| `over_limit` | 429 |
| `conflict` | 409 |
| `database` / `storage` / `kafka` / `internal` | 500 |

Flat object — not `{ error: { code, message } }`.
