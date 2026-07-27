# Usage (`client.usage`)

Wraps `hermes.usage.Usageervice` (proto service name spelling). Tenant is injected from whoami on every call.

```rust
let incr = client.usage.increment("sends", Some(1)).await?;
let check = client.usage.check("sends").await?;
```

## Methods

| Method | Signature | Returns |
| --- | --- | --- |
| `increment` | `({ metric, by? }) → Result<IncrResp>` | New value + over-limit flag |
| `check` | `({ metric }) → Result<CheckResp>` | Used vs limit |
| `get` | `({ metric, window }) → Result<Usage>` | Usage row |
| `reset` | `({ metric, window }) → Result<void>` | Empty |

### Params

| Method | You pass | Injected | Notes |
| --- | --- | --- | --- |
| `increment` | `metric`, optional `by` (default `1`) | `tenant` | |
| `check` | `metric` | `tenant` | |
| `get` / `reset` | `metric`, `window` (`YYYY-MM-DD` or `YYYY-MM`) | `tenant` | |

## Return types

### `Usage`

| Field | Type | Description |
| --- | --- | --- |
| `tenant` | `string` | |
| `metric` | `string` | Metric name |
| `value` | `number` | Current counter |
| `window` | `string` | `YYYY-MM-DD` or `YYYY-MM` |

### `IncrResp`

| Field | Type |
| --- | --- |
| `value` | `number` |
| `overLimit` | `boolean` |

### `CheckResp`

| Field | Type |
| --- | --- |
| `used` | `number` |
| `limit` | `number` |
| `over` | `boolean` |

```rust
// check
{ used: 42, limit: 100, over: false }

// increment
{ value: 43, overLimit: false }

// get
{ tenant: 'T0X…', metric: 'sends', value: 43, window: '2026-07' }
```

## Errors

Unknown metric → `INVALID_ARGUMENT`. Throws `HermesGrpcError`.
