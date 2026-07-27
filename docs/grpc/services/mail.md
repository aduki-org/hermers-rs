# Mail (`client.mail`)

Wraps `hermes.mail.MailService`. Tenant/owner are injected from whoami where required.

**Unlike REST:** gRPC has `retrieve` / `GetMessage`, `send` takes `to: string[]` + raw bytes, and mailbox create is `{ name, role? }` only — not the REST `MailboxData` body. Proto `Message` uses `date` / `from` / `Flag[]`, not REST `internaldate` / `sender`.

```rust
let boxes = client.mail.list_mailboxes().await?;
let msgs = client.mail.list_messages(&boxes.items[0].hex, None, Some(50)).await?;
```

## Methods

| Method | Signature | Returns |
| --- | --- | --- |
| `list_mailboxes` | `() → Result<ListMailboxesResp>` | `{ items: Mailbox[] }` |
| `list_messages` | `({ mailbox, cursor?, limit? }) → Result<ListMessagesResp>` | `{ items, next }` |
| `retrieve` | `(hex) → Result<Message>` | Message |
| `send` | `({ from, to, raw }) → Result<SendResp>` | `{ hex }` |
| `move` | `({ hex, dest }) → Result<MoveResp>` | `{ hex, uid }` |
| `set_flags` | `({ hex, add?, remove? }) → Result<void>` | Empty |
| `expunge` | `({ mailbox, uids }) → Result<{ expunged: number[] }>` | UIDs removed |
| `create_mailbox` | `({ name, role? }) → Result<Mailbox>` | Mailbox |
| `update_mailbox` | `({ hex, name?, role? }) → Result<Mailbox>` | Mailbox |
| `remove_mailbox` | `(hex) → Result<void>` | Empty |

### Injected fields

| Method | Injected |
| --- | --- |
| `list_mailboxes`, `create_mailbox` | `tenant`, `owner` |
| `send` | `tenant` |

### `send` params

| Field | Type | Description |
| --- | --- | --- |
| `from` | `string` | Envelope from |
| `to` | `string[]` | Recipients |
| `raw` | `Uint8Array` | Raw RFC822 bytes |

### `set_flags` params

`add` / `remove`: `MailFlag[]` (`FLAG_SEEN`, `FLAG_ANSWERED`, `FLAG_FLAGGED`, `FLAG_DELETED`, `FLAG_DRAFT`).

## Return types

### `Mailbox`

| Field | Type |
| --- | --- |
| `hex` | `string` |
| `tenant` | `string` |
| `owner` | `string` |
| `name` | `string` |
| `uidnext` | `number` |
| `uidvalidity` | `number` |
| `exists` | `number` |
| `unseen` | `number` |

### `Message`

| Field | Type |
| --- | --- |
| `hex` | `string` |
| `mailbox` | `string` |
| `uid` | `number` |
| `flags` | `Flag[]` |
| `subject` | `string` |
| `from` | `string` |
| `to` | `string[]` |
| `blob` | `string` |
| `size` | `number` |
| `date` | `Date?` |
| `created` | `Date?` |

### List responses

```rust
// ListMailboxesResp
{ items: Mailbox[] }

// ListMessagesResp
{ items: Message[]; next: string }

// SendResp
{ hex: string }

// MoveResp
{ hex: string; uid: number }
```

## Errors

`HermesGrpcError` for auth, missing mailbox, or invalid flags.
