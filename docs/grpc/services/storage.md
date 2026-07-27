# Storage (`client.storage`)

Wraps `hermes.storage.StorageService`. `put` injects `tenant` from whoami. `get` collects the server stream into one buffer.

```rust
let put = client.storage.put("notes/a.txt", b"hello".to_vec()).await?;
let bytes = client.storage.get(&put.r#ref.as_ref().unwrap().hex).await?;
```

## Methods

| Method | Signature | RPC | Returns |
| --- | --- | --- | --- |
| `put` | `({ key, data }) → Result<{ ref?: BlobRef }>` | `Put` | Blob reference |
| `get` | `(hex: string) → Result<Uint8Array>` | `Get` (stream) | Concatenated bytes |
| `remove` | `(hex: string) → Result<void>` | `Remove` | Empty |

### `put` params

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `key` | `string` | yes | Object key |
| `data` | `Uint8Array` | yes | Payload |

SDK adds `tenant` from whoami.

## Return types

### `BlobRef`

| Field | Type | Description |
| --- | --- | --- |
| `hex` | `string` | Blob id |
| `tenant` | `string` | |
| `backend` | `string` | Storage backend id |
| `bucket` | `string` | |
| `key` | `string` | |
| `size` | `number` | Bytes |
| `hash` | `Buffer` / `Uint8Array` | Content hash |

### Put response

```json
{ ref?: BlobRef }
```

### Get

Returns a single `Uint8Array` assembled from streamed `Chunk { data: Buffer }` messages.

## Errors

Missing blob → `NOT_FOUND`. Over quota → `RESOURCE_EXHAUSTED`. Throws `HermesGrpcError`.
