# hermers

Open-source **Rust** client for Hermers — REST and native gRPC in **one crate**.

The Hermers **server is private / proprietary**. This package is open source; it does not include or publish server source.

| Client | Feature | Default endpoint |
| --- | --- | --- |
| `Hermes` | `rest` (default) | `https://hermers.aduki.pro/v1` |
| `HermesGrpc` | `grpc` (default) | `grpc.aduki.pro:443` (TLS) |

## Install

```toml
[dependencies]
hermers = "0.2"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

REST only (smaller tree):

```toml
hermers = { version = "0.2", default-features = false, features = ["rest"] }
```

## Quickstart (REST)

```rust
use hermers::Hermes;

#[tokio::main]
async fn main() -> Result<(), hermers::HermesError> {
    let hermes = Hermes::new("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx")?;
    let me = hermes.ready().await?;
    println!("{} / {}", me.tenant, me.user);
    let contacts = hermes.contacts.list(None).await?;
    println!("{} contacts", contacts.total);
    Ok(())
}
```

## Quickstart (gRPC)

```rust
use hermers::HermesGrpc;

#[tokio::main]
async fn main() -> Result<(), hermers::HermesGrpcError> {
    let client = HermesGrpc::connect("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx").await?;
    client.ready().await?;
    let list = client.contacts.list(None, Some(50)).await?;
    println!("{} items", list.items.len());
    Ok(())
}
```

## Authentication

**API key only** — `Authorization: Key hm_live_…` (REST) / metadata `authorization: Key …` (gRPC).

## Testing

```bash
cargo test
export HERMERS_API_KEY=hm_live_…   # never commit secrets
cargo test
```

## Docs

Developer documentation lives under [`docs/`](docs/README.md) — REST in `docs/rest/`, gRPC in `docs/grpc/`.

Documentation is published with **[mdBook](https://rust-lang.github.io/mdBook/)** (Rust-book UI).
Book root is this package (`book.toml` + `book/`); chapters are assembled from `docs/` and
the crate README into `book/`, then built to `site/` for GitHub Pages.

Requires [`mdbook`](https://github.com/rust-lang/mdBook) on `PATH` (e.g. `cargo install mdbook`) and Node.js ≥18 for the prepare/check scripts.

```bash
make docs-prepare   # sync docs/ + crate README → book/
make docs-build     # prepare + mdbook build → site/
make docs-serve     # prepare + mdbook serve (http://localhost:3000)
make docs           # build + link/forbidden-string check
```

Or: `./scripts/docs-build.sh` / `./scripts/docs-serve.sh`.

## License

MIT
