<!-- orchestrate handoff
task: audit-audit-cmd
branch: orch/agcli-audit/audit-audit-cmd
agentId: bc-5a916090-892a-4bf8-9c89-3c75c6e92971
runId: run-4da7bcd2-3162-43fc-8509-9579162bd7ac
resultStatus: finished
finishedAt: 2026-05-27T11:47:00.283Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-audit-cmd`

## What I did
- Added `tests/audit_audit_cmd.rs`:
  - Parse-surface tests for `agcli audit` (default address resolution path and explicit `--address` + `--output json` path) using `agcli::cli::Cli::try_parse_from`.
  - One `#[ignore]` local-chain integration test (`green_path_audit_queries_local_chain`) that exercises the same read path used by `handle_audit` (balance, stake, identity, proxies, delegate info, coldkey-swap announcement, child/pending child keys).
- Updated `docs/commands/view.md` with a full `audit` command section including:
  - Clap flags + types.
  - Exit code mapping aligned with `src/error.rs`.
  - JSON output schema for `handle_audit`.
  - Subxt mapping table for each query call in `handle_audit` (pallet/runtime API, storage key, SCALE arg/key shape).
  - On-chain event references (and explicit note that `agcli audit` emits none since it submits no extrinsic).
- Traced `Commands::Audit` (`src/cli/mod.rs`) through dispatch (`src/cli/commands.rs`) into `view_cmds::handle_audit`, then into chain query methods and corresponding subtensor/proxy/registry/runtime-api definitions.

## Measurements
- `docs/commands/view.md LOC`: `178 → 280`
- `tests/audit_audit_cmd.rs LOC`: `0 → 78`
- `cargo check --all-targets exit code`: `0 == 0`
- `cargo test --no-run --test audit_audit_cmd exit code`: `0 == 0`
- `cargo test --test audit_audit_cmd parse_audit_command`: `0 passing → 2 passing`

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- `Commands::Audit` is a single top-level command variant (`Audit { address: Option<String> }`), not a nested clap subcommand tree.
- Findings:
  - **Docs drift (fixed in this branch):** `docs/commands/view.md` previously only referenced `handle_audit()` in one line and lacked audit command clap surface, exit codes, JSON schema, storage/runtime mappings, and event references.
  - **Mixed-block read in source (follow-up):** `handle_audit` pins a block hash for most reads but calls `get_all_dynamic_info()` (latest, unpinned), so `price/tao_in_pool` can come from a different block than stake/proxy/identity/child-key state.
  - **Validation hint gap in source (follow-up):** `src/error.rs::hint()` has specific `portfolio --address` guidance but no `audit --address`-specific hint branch; invalid audit address still classifies to exit code `12`, but guidance is less targeted.
  - **Formatting issue in source (follow-up):** coldkey swap finding message runs `short_ss58()` on a hash string (`0x...`), which is semantically an SS58 formatter used for non-SS58 data.
  - **Dispatchable surface in scope:** none. `agcli audit` submits no extrinsic; it is entirely query/runtime-API based.
- Local-chain integration test is intentionally `#[ignore]` (as requested/gated). I did not execute Docker/localnet in this VM for this task.

## Suggested follow-ups
- Pin dynamic info in `handle_audit` to the same block (`get_all_dynamic_info_at_block(pin)`) to keep audit snapshots fully consistent.
- Add `audit --address` hint handling in `src/error.rs::hint()` for parity with other address-validation flows.
- Replace SS58 shortener for coldkey swap hash messaging with a hash-oriented formatter.
- Refresh the broader `docs/commands/view.md` source-summary block, which still lists an incomplete set of view handlers (outside this scoped audit command work).