<!-- orchestrate handoff
task: audit-evm
branch: orch/agcli-audit/audit-evm
agentId: bc-7c929b1a-c547-4d01-aef6-c3c84cd0d81b
runId: run-30ef5b1e-2016-485e-b1b9-5429f072f065
resultStatus: finished
finishedAt: 2026-05-27T11:33:28.039Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-evm`

## What I did
- Updated `docs/commands/evm.md` end-to-end for the full `agcli evm` CLI surface (`call`, `withdraw`) with:
  - clap flags + Rust types + defaults
  - handler/dispatch path (`handle_evm` -> `Client` -> subxt dynamic call)
  - SCALE argument mapping vs pallet expectations
  - exit code mapping aligned to `src/error.rs`
  - output JSON schema notes
  - pallet references, storage keys, and emitted events
  - explicit coverage note that `create`/`create2` exist in pallet/client helpers but not in CLI
- Added `tests/audit_evm.rs` with:
  - parse-surface coverage for every `EvmCommands` subcommand using `Cli::try_parse_from`
  - default-value parse assertions for `evm call`
  - one `#[ignore]` localnet integration test that attempts a green-path `evm_call` + `evm_withdraw` flow against Docker localnet
- Ran required compile checks and targeted test execution.

## Measurements
- `tests/audit_evm.rs`: 0 → 1
- `cargo check --all-targets`: fail → pass
- `cargo test --no-run --test audit_evm`: fail → pass
- `cargo test --test audit_evm parse_surface_evm -- --nocapture`: 0 passing → 2 passing

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- ### Findings
  - `agcli evm` is missing CLI surface for pallet dispatchables `EVM::create` and `EVM::create2` (only `call` and `withdraw` are exposed in `EvmCommands`), despite helper methods existing in `src/chain/extrinsics.rs`.
  - `Client::evm_call` submits dynamic tx args without an `authorization_list` argument, while upstream `pallet-evm` `call` signature includes it (`call(..., access_list, authorization_list)`), creating likely runtime-compatibility drift.
  - `handle_evm` prints plain text directly and does not use output-format helpers; `--output json` is effectively ignored for `evm call`/`evm withdraw` (ill-formatted for machine consumers).
  - `Client::evm_call` encodes `gas_limit` as `Value::u128` while pallet expects `u64`; this may coerce but is a type-shape mismatch.
  - `Client::evm_create` / `evm_create2` helpers encode `value` and fee fields as `u128` although pallet expects `U256`, and also do not pass `authorization_list`.
- The ignored integration test was added per scope; parse tests were run and passed, but localnet EVM flow remains ignored by default.

## Suggested follow-ups
- Add `agcli evm create` and `agcli evm create2` commands in CLI and docs, wired through `handle_evm`.
- Update EVM extrinsic builders to match current pallet signatures exactly (including `authorization_list`) and use explicit `U256`-correct dynamic encoding.
- Normalize `evm` command output via shared output helpers (`print_tx_result`/JSON pathways) so `--output json` is actually honored.
- Add a non-ignored EVM localnet e2e path once CI/local agent Docker availability is standardized.