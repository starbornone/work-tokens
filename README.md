# work-tokens

Rust prototype of a blockchain where work tokens can expire to discourage hoarding. The Cargo package name is `core`.

This is an early scaffold: core modules exist (ledger, PoW, crypto, SQLite, P2P stubs, RPC stubs), but networking, wallet CLI, and on-chain expiration are not fully wired yet.

## Docs

- [Code review and remediation plan](docs/CODE_REVIEW.md)

## Prerequisites

- Rust (stable) via [rustup](https://rustup.rs/)
- Cargo (included with Rust)

## Setup

```bash
git clone git@github.com:starbornone/work-tokens.git
cd work-tokens
cargo build
cargo test
```

Optional config: `config/settings.toml` (defaults are used if missing). Override with `APP__` env vars, e.g. `APP__CONSENSUS__DIFFICULTY=3`.

## Usage

```bash
cargo run
```

Prototype startup: load settings → create ledger → sign a sample tx → mine block 1 → validate → persist to SQLite. P2P listen/peer dial and wallet CLI are not implemented yet.

## Layout

```plaintext
work-tokens/
├── Cargo.toml
├── config/settings.toml
├── docs/CODE_REVIEW.md
├── src/                 # library + binary
│   ├── blockchain/
│   ├── consensus/
│   ├── crypto/
│   ├── network/
│   ├── storage/
│   ├── token/
│   ├── wallet/
│   ├── api/
│   ├── config/
│   └── utils/
└── tests/blockchain_tests.rs
```

## Contributing

1. Fork and clone
2. `git checkout -b feature/your-feature-name`
3. Make changes; run `cargo test`
4. Commit, push, open a PR

See [docs/CODE_REVIEW.md](docs/CODE_REVIEW.md) for prioritized work.
