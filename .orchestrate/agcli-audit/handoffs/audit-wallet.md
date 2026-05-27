<!-- orchestrate handoff
task: audit-wallet
branch: orch/agcli-audit/audit-wallet
agentId: bc-83564caa-480d-4d9b-b7ed-3a1ccbce1c24
runId: run-51ee8377-bae1-4a2c-9f6b-57aa91638a64
resultStatus: finished
finishedAt: 2026-05-27T12:43:04.353Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-wallet`

## What I did

- **`tests/audit_wallet.rs`** (new, 888 LOC): 
  - 23 parse-surface tests: every `WalletCommands` variant parsed via `Cli::try_parse_from` with realistic args
  - 14 handler-level green-path and error-path tests via `handle_wallet()` (no chain required)
  - 1 `#[ignore]` integration test (`green_path_associate_hotkey_localnet`) gated on a running localnet at `ws://127.0.0.1:9944`
  - All 42 non-ignored tests pass; 1 ignored

- **`docs/commands/wallet.md`** (full rewrite, 53 → 516 LOC):
  - All 13 subcommands documented with clap flags + types, exit codes, JSON output schemas, pallet refs, storage key (for `check-swap`), and on-chain events (for `associate-hotkey`)
  - Exit code table cross-referenced with `src/error.rs::exit_code`
  - Corrected `coldkeypub.txt` description (stores raw hex pubkey, not SS58)
  - Added audit findings as inline notes

## Measurements
- `cargo check --all-targets`: passes → passes
- `cargo test --no-run --test audit_wallet`: compiles → compiles
- `LOC(docs/commands/wallet.md)`: 139 (body) → 516
- `LOC(tests/audit_wallet.rs)`: 0 → 888
- test count (audit_wallet): 0 → 42 passing, 1 ignored

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings

1. **`WalletCommands::Create::hotkey_name` naming collision with global `Cli::hotkey_name`** — Both fields resolve to the CLI flag `--hotkey-name` (the global also has `alias = "hotkey"`). The global has `default_value = "default"`, so the subcommand field is always `Some("default")` rather than `None` when the user omits `--hotkey-name`. The handler's `.unwrap_or("default")` call works correctly either way, so behavior is correct, but the `Option` distinction (user-provided vs. default) is silently lost.

2. **`require_mnemonic()` security guard is undocumented** — When `--mnemonic` is passed via CLI and `AGCLI_MNEMONIC` env var is *not* set, the handler returns a hard error ("Refusing --mnemonic flag: mnemonic phrases are visible in `ps` output"). This security guard is correct behavior but is not documented in `wallet.md` (any of create/import/regen-coldkey/regen-hotkey). Agents that try to pass `--mnemonic` directly without the env var will get an opaque error.

3. **`coldkeypub.txt` stores raw hex pubkey, not SS58 address** — The previous docs said "SS58 address". Actual content is `hex::encode(public.0)` — a 64-hex-char string with no `0x` prefix. The `Wallet::open()` reads this and derives SS58 internally. The file is compatible with Python `bittensor-wallet` JSON format (which also stores `publicKey` + `ss58Address`). Docs corrected.

4. **`wallet sign` output format ignores `--json`/`--csv` flags** — The handler always calls `crate::cli::helpers::print_json()` regardless of the `OutputFormat` value. This diverges from the other subcommands which check `output.is_json()` etc. The behavior is arguably correct (signatures should be machine-parseable), but undocumented. `wallet verify` has the same behavior.

5. **`try_associate_hotkey` SCALE encoding uses `Value::from_bytes(hk.0)`** — `hk.0` is an `AccountId` struct's inner `[u8; 32]`. `Value::from_bytes` wraps it as a bytes sequence. For substrate's dynamic dispatch, `AccountId` parameters expect a 32-byte array, not a typed bytes value. This *may* work because subxt's dynamic codec normalizes byte sequences to `[u8; 32]` arrays at the encoding stage, but it would be safer to use `Value::unnamed_composite([Value::from_bytes(hk.0)])` or the typed `AccountId32` approach. No confirmed breakage seen from the code, but worth a dedicated localnet test.

6. **`check-swap` JSON output key name inconsistency** — The previous docs showed `"new_coldkey"` but the actual code emits `"new_coldkey_hash"`. Fixed in updated docs.

7. **No `swap_coldkey` agcli surface in the `wallet` group** — The dispatchable `SubtensorModule::swap_coldkey` (which *initiates* a coldkey swap) is documented as accessible via `agcli swap coldkey` (a separate command group). There is no wallet-subgroup alias or cross-reference. Users looking under `wallet` for swap initiation won't find it. Discovery.md confirms the `swap` group exists; the finding is purely about documentation/discoverability.

## Suggested follow-ups

- **Source fix**: Rename `WalletCommands::Create::hotkey_name` arg to `--initial-hotkey-name` (or use a different env var) to avoid the naming collision with the global `Cli::hotkey_name`.
- **Source fix**: Document the `require_mnemonic` security guard in CLI `--help` text for all mnemonic-accepting subcommands.
- **Source fix**: Consider changing `try_associate_hotkey` to use typed `AccountId` encoding for clarity (though current behavior may be correct).
- **Localnet test**: Add a localnet-gated test that verifies `try_associate_hotkey` succeeds against `devnet-ready` image and confirms the `Owner` storage map is updated.
- **Docs**: Add cross-reference from `wallet check-swap` to `agcli swap coldkey` for the swap initiation path.
- **Source**: `wallet sign` and `wallet verify` should either respect `--json`/`--csv`/`--table` format flags, or add `#[arg(long, hide = true)]` overrides and document the always-JSON behavior.