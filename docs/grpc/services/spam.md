# Spam (`client.spam`)

Wraps `hermes.spam.SpamService`. Tenant (and user on report) come from whoami.

```rust
use hermers::grpc::pb::spam::Verdict;

let resp = client.spam.classify("M0X…", raw_eml, "inbound").await?;
client.spam.report("M0X…", Verdict::Spam, "user_report").await?;
```

## Methods

| Method | Signature | Returns |
| --- | --- | --- |
| `classify` | `({ msg, raw, direction }) → Result<ClassifyResp>` | Verdict + scores |
| `report` | `({ msg, verdict, source }) → Result<void>` | Empty |

### Params

| Method | You pass | Injected |
| --- | --- | --- |
| `classify` | `msg` (message hex), `raw` (`Uint8Array` eml), `direction` (`"inbound"` \| `"outbound"`) | `tenant` |
| `report` | `msg`, `verdict` (`SpamVerdict`), `source` (`"user_report"` \| `"dmarc_feedback"` \| `"honeypot"`) | `tenant`, `user` |

## Return types

### `Verdict` (`SpamVerdict`)

| Name | Value |
| --- | --- |
| `CLEAN` | `0` |
| `SPAM` | `1` |
| `BULK` | `2` |

### `Scores`

| Field | Type |
| --- | --- |
| `rules` | `number` |
| `bayes` | `number` |
| `ml` | `number` |
| `reputation` | `number` |
| `composite` | `number` |

### `ClassifyResp`

| Field | Type |
| --- | --- |
| `verdict` | `Verdict` |
| `scores` | `Scores?` |
| `reason` | `string` |

```json
{
  verdict: 1, // SPAM
  scores: { rules: 2.1, bayes: 0.9, ml: 0.8, reputation: 0.1, composite: 3.9 },
  reason: 'matched bulk template',
}
```

## Errors

Unknown message → `NOT_FOUND`. Throws `HermesGrpcError`.
