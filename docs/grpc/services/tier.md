# Tier (`client.tier`)

Wraps `hermes.tier.TierService`. Tenant is filled from whoami.

```rust
use hermers::grpc::pb::tier::Plan;

let info = client.tier.resolve().await?;
let changed = client.tier.change(Plan::Pro, None).await?;
```

## Methods

| Method | Signature | Returns |
| --- | --- | --- |
| `resolve` | `() → Result<TierInfo>` | Current plan + limits |
| `change` | `({ plan, paymentMethod? }) → Result<ChangeResp>` | Updated info |

### Params

| Method | You pass | Injected |
| --- | --- | --- |
| `resolve` | — | `tenant` |
| `change` | `plan` (`TierPlan`), optional `paymentMethod` (default `''`) | `tenant` |

## Return types

### `Plan` (`TierPlan`)

| Name | Value |
| --- | --- |
| `FREE` | `0` |
| `STARTER` | `1` |
| `PRO` | `2` |
| `BUSINESS` | `3` |
| `ENTERPRISE` | `4` |

### `Limits`

| Field | Type |
| --- | --- |
| `sendsDay` | `number` |
| `sendsMonth` | `number` |
| `mailboxes` | `number` |
| `domains` | `number` |
| `members` | `number` |
| `storageMb` | `number` |
| `apiKeys` | `number` |
| `webhooks` | `number` |
| `mlEnabled` | `boolean` |

### `TierInfo`

| Field | Type |
| --- | --- |
| `tenant` | `string` |
| `plan` | `Plan` |
| `limits` | `Limits?` |
| `trial` | `boolean` |
| `trialEnds` | `string` |

### `ChangeResp`

```json
{ info?: TierInfo }
```

```json
{
  tenant: 'T0X…',
  plan: 0, // FREE
  limits: {
    sendsDay: 100,
    sendsMonth: 1000,
    mailboxes: 5,
    domains: 1,
    members: 3,
    storageMb: 1024,
    apiKeys: 5,
    webhooks: 2,
    mlEnabled: false,
  },
  trial: false,
  trialEnds: '',
}
```

## Errors

Billing / plan change failures → `FAILED_PRECONDITION` / `INVALID_ARGUMENT`. Owner scope typically required for `change`. Throws `HermesGrpcError`.
