<!-- orchestrate handoff
task: audit-explain
branch: orch/agcli-audit/audit-explain
agentId: bc-66f3b332-876b-4b4a-bc48-d7d0443e37a8
runId: run-af08afe8-68f0-4018-be01-f3c9f1eaa65e
resultStatus: finished
finishedAt: 2026-05-27T11:33:31.087Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-explain`

## What I did
- Updated `docs/commands/explain.md` with a full end-to-end audit of the explain command group:
  - documented every explain execution mode (`explain`, `--topic`, `--full`, `--topic --full`)
  - added clap flags/types, JSON schemas per mode, and exit-code behavior tied to `src/error.rs`/`src/main.rs`
  - added a complete 32-topic catalog from `src/utils/explain.rs` including aliases
  - added pallet/storage/event references per topic (or explicit read-only / not-applicable notes where appropriate)
  - documented that `Commands::Explain` has no subxt dispatch path and no SCALE-encoded extrinsic submission
- Added `tests/audit_explain.rs`:
  - parse-surface coverage for all canonical explain topics via `Cli::try_parse_from`
  - parse-surface coverage for alias topics
  - parse coverage for list/full/full-topic modes with realistic args
  - one `#[ignore]` localnet integration smoke test that starts localnet, connects with `agcli::Client`, and validates explain parsing in a local-chain context
- Ran required compile verification and ensured branch is committed and pushed.

## Measurements
- `cargo check --all-targets (exit code)`: 0 == 0
- `cargo test --no-run --test audit_explain (exit code)`: 0 == 0
- `canonical explain topics documented in docs table`: 32 == 32

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- Findings:
  - `explain` is fully offline in current code paths. `Commands::Explain` dispatches straight to `handle_explain` and never constructs a chain client, so there are no pallet dispatchables or SCALE argument encodings to validate for command execution itself.
  - Error-code behavior is inconsistent across explain failure modes: unknown topic is classified as validation (`12`), but `--full` file-resolution failures (missing docs dir/doc file/invalid topic string) currently fall through to generic (`1`).
  - `--output csv` is accepted globally but explain has no CSV-specific renderer; non-JSON explain output is plain text, so CSV callers do not get structured CSV.
  - The `senate` concept exists in explain topics, but current custom pallets under `subtensor/pallets/*` do not provide an active dedicated senate/triumvirate pallet surface; references are effectively runtime/deprecation context, not a live custom pallet API.
- Deviation: localnet integration test is intentionally `#[ignore]` per scope and environment variability (Docker/localnet availability is not guaranteed in this VM for test execution).

## Suggested follow-ups
- Normalize explain failure exit codes so all invalid topic/doc resolution failures classify consistently (likely validation or IO instead of generic).
- Either implement explicit CSV output for explain modes or reject `--output csv` for explain with a validation error.
- Consider splitting topic metadata into a typed registry (topic -> aliases + pallet/storage/event refs) to keep docs/test/source synchronized automatically.