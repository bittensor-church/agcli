<!-- orchestrate handoff
task: audit-doctor
branch: orch/agcli-audit/audit-doctor
agentId: bc-5a72f2c2-fa57-4ab3-98fd-29a875084d82
runId: run-c9ba9f13-8b8c-4040-92f8-6f840827a910
resultStatus: finished
finishedAt: 2026-05-27T11:33:26.885Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-doctor`

## What I did
- Audited `Commands::Doctor` dispatch in `src/cli/commands.rs` and `handle_doctor` in `src/cli/system_cmds.rs`, tracing all on-chain calls:
  - `Client::connect_network` (RPC transport connect),
  - `Client::get_block_number` (best-head probe),
  - `Client::get_total_networks` (`SubtensorModule::TotalNetworks` storage read).
- Cross-referenced subtensor storage/events for doctor-relevant chain state:
  - `subtensor/pallets/subtensor/src/lib.rs` (`TotalNetworks` storage),
  - `subtensor/pallets/subtensor/src/macros/events.rs` (`NetworkAdded`/`NetworkRemoved` writers tied to that storage).
- Updated `docs/commands/doctor.md` to fully document doctor surface with:
  - explicit “no subcommands” statement,
  - clap flags + types,
  - exit-code behavior vs `src/error.rs`,
  - JSON output schema,
  - pallet/storage references,
  - event-emission notes for read-only probes.
- Added `tests/audit_doctor.rs` containing:
  - parse-surface coverage for realistic doctor invocations via `Cli::try_parse_from`,
  - explicit rejection test proving doctor has no nested subcommands,
  - one `#[ignore]` local-chain integration green-path (`ws://127.0.0.1:9944`) that executes doctor end-to-end.

## Measurements
(none)

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- `cargo check --all-targets` passed.
- `cargo test --no-run --test audit_doctor` passed.
- `cargo test --test audit_doctor` passed (`2 passed`, `1 ignored`).
- Findings:
  - `doctor` is a single command, not a true command group with subcommands; any nested token after `doctor` is a clap parse error.
  - CLI help text for `Doctor` claims a chain-version diagnostic, but `handle_doctor` does not call `system_chain`/`system_version` (or equivalent runtime/version RPC probe).
  - `--output csv` is accepted globally, but `doctor` only implements JSON vs non-JSON; CSV currently falls back to table-style text (shape mismatch for automation expecting CSV).
  - In `handle_doctor`, the `Err` branch for `Wallet::open(...)` is effectively unreachable in normal missing-wallet cases because `Wallet::open` returns `Ok` with missing keys; doctor reports “coldkey: missing” rather than “wallet not found”.
  - `doctor` does no extrinsic submission, so there is no dispatchable/SCALE-args path in scope; only read-only probes (RPC + storage) are exercised.

## Suggested follow-ups
- Add explicit chain identity/version checks to doctor (e.g., `system_chain`, `system_version`, and optionally runtime spec version), or update command help text to match current behavior.
- Implement real CSV output path for doctor (or reject `--output csv` for doctor with clear validation).
- Rework wallet health semantics in doctor to distinguish “wallet directory missing” vs “wallet present but missing coldkey/hotkeys”.
- Consider surfacing/use-testing `--best` behavior for doctor (currently not part of doctor’s connection path).