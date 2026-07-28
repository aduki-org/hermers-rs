# Whoami

Both transports resolve identity with **whoami** and cache it. Prefer `await client.ready()` before the first resource call. Resource methods never take tenant/user hex.

**REST:** `GET /auth/whoami`  
**gRPC:** `SessionService.Whoami` → `Session` message

## Identity fields (REST wire + SDK cache)

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `hex` | string | yes (wire) | Session / JTI (`A0S…`) |
| `user` | string | yes | User hex (`U0X…`) — **string, not object** |
| `tenant` | string | yes | Tenant hex (`T0X…`) — **string, not object** |
| `owner` | boolean | yes | Tenant owner |
| `scopes` | string[] | yes | Flattened `"domain.scope"` patterns |
| `deny` | string[] | yes | Deny patterns |
| `tier` | string | yes | Plan slug (e.g. `free`) |
| `ip` | string | yes | Always `""` today |
| `agent` | string | yes | Always `""` today |

## REST example

```rust
use hermers::Hermes;

let hermes = Hermes::new("hm_live_…")?;
let me = hermes.ready().await?;
println!("{} / {}", me.tenant, me.user);
```

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

## gRPC

```rust
use hermers::HermesGrpc;

let client = HermesGrpc::connect("hm_live_…").await?;
client.ready().await?; // SessionService.Whoami
```

See also [Authentication & API keys](rest/services/auth.md).
