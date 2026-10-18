# Code review — work-tokens

Date: 2026-10-19  
Scope: full repository (prototype / learning scaffold)  
Status: findings accepted; remediation started (see [Remediation tracker](#remediation-tracker))

## 1. System overview

**work-tokens** is an early Rust prototype for a blockchain where tokens are earned for work and meant to expire to discourage hoarding. The crate (`core` in `Cargo.toml`) is split into modules: `blockchain` (blocks, txs, ledger, Merkle tree), `consensus` (PoW + validator), `crypto` (hash/keys/signatures), `token` (issuance/expiration/balances), `wallet`, `network` (TCP P2P + messages), `storage` (SQLite), `api` (Warp RPC stubs), and `config`.

Intended flow: wallet → signed tx → mempool/P2P → PoW block → ledger + DB, with RPC for clients. In practice modules were not wired together and several public surfaces did not compile. README historically documented CLI flags, wallet subcommands, `docs/`, and `scripts/` that were missing.

**Assumption:** target stage is local prototype, not a production network.

## 2. Overall assessment

Module layout is a reasonable sketch of a node. The project needed a compile-green vertical slice before further feature work: broken exports, consensus not enforced, product token model disconnected from the ledger, and stale integration tests.

| Area | Rating | Reason |
| --- | --- | --- |
| Architecture and design | Needs work | Clear folders, but no orchestration; dual runtimes; token vs chain models diverge |
| Correctness | Critical | Broken validation, PoW not enforced, token transfer can destroy funds, block hash/merkle not fully checked |
| Readability / maintainability | Needs work | Duplicated keygen APIs, stubs, inconsistent address types |
| Error handling / resilience | Needs work | Widespread `expect`/`unwrap` on network I/O and crypto parse paths |
| Security | Critical | Unauthenticated RPC; plaintext keys; vulnerable `ed25519-dalek` 1.x; P2P panic on bad input |
| Performance | Adequate | Fine for prototype scale |
| Testing | Critical | Integration tests out of date with APIs; high-risk paths untested |
| Database design | Needs work | Thin schema; FK target not unique; no migrations; chain not loaded from DB |
| API design | Needs work | Stub endpoints, always-success submit |
| Frontend | N/A | No frontend |
| Observability | Needs work | `println!` only; `log`/`env_logger` unused |
| Dependencies / config | Needs work | Outdated crypto; settings path mismatch |
| Build / CI / DX | Critical | No CI; `.gitignore` looks like a rustc copy |
| Documentation | Needs work | README oversold features |

## 3. Findings

### Critical

1. **Project did not compile — broken module surface**  
   `main.rs`, `wallet/mod.rs`, `token/mod.rs`, `consensus/validator.rs`  
   Missing imports/re-exports; `Validator` called non-existent APIs and wrong `verify_signature` shape.

2. **Ledger accepts blocks without PoW, tx validity, or index checks**  
   `ledger.rs` `add_block`, `block.rs` `validate`, `proof_of_work.rs` `mine_block`  
   Mining did not persist `block.hash`; difficulty never checked on accept.

3. **Block integrity incomplete**  
   `block.rs` `validate` did not recompute/compare merkle root against current transactions.

4. **Token transfer can delete balances and over-spend**  
   `token/management.rs` `transfer_tokens`  
   Failed transfers left tokens removed; success could move more than requested.

5. **RPC accepts any transaction and always succeeds**  
   `api/rpc.rs` — no validation, unused network handle.

6. **Integration tests stale / assert wrong properties**  
   `tests/blockchain_tests.rs` — wrong `Transaction::new` types, missing `mine`, expects behaviour ledger never implemented.

### High

7. Product token model disconnected from chain (`token/*` vs `blockchain/transaction.rs`).  
8. P2P unframed + panics on bad input (`network/p2p.rs`).  
9. Private keys stored as raw unencrypted bytes (`crypto/keys.rs`).  
10. `ed25519-dalek` 1.0 — CVE-2022-50237 / RUSTSEC-2022-0093 (`Cargo.toml`).  
11. Signature verification panics on malformed bytes (`crypto/signatures.rs`).  
12. Config path (`config/settings`) did not match checked-in TOML location.

### Medium

13. Duplicate crypto keygen APIs (`keys` vs `signatures`).  
14. Merkle proof order-unsafe / no proof builder.  
15. SQLite schema gaps (non-unique FK target, no `PRAGMA foreign_keys`, no migrations).  
16. Async (Tokio/Warp) vs sync TCP split; RPC never started from `main`.  
17. Observability gap.  
18. README / package naming drift (`core` vs work-tokens).

### Low

19. `.gitignore` appears copied from the Rust compiler repo.  
20. Key tests can leave `unused_private.key`.  
21. `from: PublicKey` / `to: String` inconsistency.  
22. Toy PoW hex-string target comparison.

## 4. What's working well

- Module boundaries match a real node’s concerns.
- Hash helper has known-answer unit tests.
- `Transaction::{sign, verify_signature, validate}` is the right validation surface.
- Serialization utilities are small and tested.
- SQLite round-trip tests point in the right direction.
- P2P message (de)serialization unit test is a good pattern.

## 5. Recommended plan

### Quick wins

1. Make the crate compile (exports, `main`, `Validator`).  
2. Non-panicking signature parse; fix config path.  
3. Fix `TokenManager::transfer_tokens` (select-then-commit + change).  
4. Repair or quarantine integration tests.  
5. Align README with prototype reality.

### Vertical slice (next)

6. Wallet → sign tx → validate → mine (update hash) → `Ledger::add_block` (PoW + txs + merkle) → `Database::save_block`.  
7. Wire `main` + RPC to shared ledger state.  
8. Upgrade `ed25519-dalek` / encrypt keys before any real-network use.

### Larger refactors

9. Unify token expiration with chain validation.  
10. Tokio framed P2P protocol.  
11. Proper SQLite schema + migrations.  
12. CI: `fmt`, `clippy -D warnings`, `test`.

## 6. Open questions

1. Intended maturity: learning scaffold vs testnet?  
2. Account model: UTXO with per-token expiry vs account balances?  
3. Who issues work credits (trusted issuer, multi-sig, other)?  
4. Deployment: single local node vs multi-peer?  
5. Keep expanding docs/CLI now, or trim until implemented?

## Remediation tracker

| Item | Status |
| --- | --- |
| Document review (`docs/CODE_REVIEW.md`) | Done |
| Compile-green (main, exports, Validator) | Done (pending local `cargo check`) |
| Signature parse returns `false` instead of panic | Done |
| Config path + `config/settings.toml` | Done |
| Token transfer correctness | Done |
| PoW hash update + ledger/block validation | Done |
| Fix integration tests | Done |
| README accuracy | Done |
| Vertical slice wiring | Done (main mines, validates, persists one block) |
| Crypto upgrade / keystore | Pending |
| P2P framing | Pending |
| CI pipeline | Pending |
