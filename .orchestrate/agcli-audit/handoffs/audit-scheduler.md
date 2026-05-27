<!-- orchestrate handoff
task: audit-scheduler
branch: orch/agcli-audit/audit-scheduler
agentId: bc-9ef1dd41-e414-4fe7-bde5-12dfd77910d2
runId: run-be17fce9-bd8d-4ff5-8263-d5a8e94c8f12
resultStatus: finished
finishedAt: 2026-05-27T11:45:11.101Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-scheduler`

## What I did
- Audited the full scheduler CLI surface (`schedule`, `schedule-named`, `cancel`, `cancel-named`) from clap (`src/cli/mod.rs`) through handler (`src/cli/network_cmds.rs`) into subxt calls (`src/chain/extrinsics.rs`), and cross-referenced against `pallet_scheduler` dispatchables/storage/events used by subtensor runtime.
- Rewrote `docs/commands/scheduler.md` to document every scheduler subcommand with:
  - clap flags + types
  - exit-code behavior (from `src/error.rs`)
  - output JSON schema status
  - pallet dispatchable mapping
  - relevant scheduler storage keys and events
  - explicit note of unsurfaced in-scope dispatchables (`schedule_after`, `schedule_named_after`)
- Added `tests/audit_scheduler.rs` with:
  - parse-surface coverage using `agcli::cli::Cli::try_parse_from` for every scheduler subcommand
  - one `#[ignore]` async local-chain green-path test (`green_path_scheduler_named_local_chain`) that schedules and cancels a named task against a local endpoint.

## Measurements
- `docs/commands/scheduler.md LOC: 55 → 227`
- `tests/audit_scheduler.rs LOC: 0 → 115`
- `cargo check --all-targets exit_code: 0 == 0`
- `cargo test --no-run --test audit_scheduler exit_code: 0 == 0`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- ### Findings
  - **Dispatchable coverage gap:** Scheduler pallet includes `schedule_after` and `schedule_named_after`, but `SchedulerCommands` exposes neither subcommand. `Client::schedule_after` exists, but no CLI path calls it; `schedule_named_after` wrapper is absent.
  - **Named ID encoding drift:** CLI/docs imply named IDs are string-friendly (and clap comment says “will be hashed”), but runtime scheduler expects `TaskName = [u8; 32]`. Current path forwards `id.as_bytes()` directly (`handle_scheduler` → `schedule_named_call` / `cancel_named_scheduled`), without hashing/padding to fixed 32 bytes.
  - **Output format drift:** scheduler handlers use plain `println!` only and do not emit structured JSON even under JSON output mode; docs previously did not call this out.
  - **Privilege requirement not surfaced in command UX:** runtime config sets `Scheduler::ScheduleOrigin = EnsureRoot<AccountId>`, so non-root signers will fail on-chain for schedule/cancel flows.
  - **Exit code granularity:** scheduler-specific runtime failures (e.g. `Scheduler::NotFound`, `TargetBlockNumberInPast`) collapse into generic chain classification (`13`) via heuristic error classification; no scheduler-specific discrimination.
- Local-chain integration was intentionally left `#[ignore]` per scope guidance; compiled successfully, not executed in this VM session.
- No source files under `src/` were modified.

## Suggested follow-ups
- Add CLI subcommands for `scheduler schedule-after` and `scheduler schedule-named-after`, wiring to pallet dispatchables.
- Normalize scheduler named IDs to runtime `TaskName` semantics (explicit 32-byte handling or deterministic hash) and align clap/docs/comments with actual behavior.
- Add structured JSON output for scheduler handlers (success + error payloads) to match global output mode expectations.
- Improve scheduler error surfacing in `src/error.rs` with explicit scheduler pallet variants/hints (e.g., `NotFound`, `TargetBlockNumberInPast`, `Named`).