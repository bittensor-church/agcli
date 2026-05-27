<!-- orchestrate handoff
task: audit-preimage
branch: orch/agcli-audit/audit-preimage
agentId: bc-3001f981-16b2-401c-9f30-d0c097788355
runId: run-d972e1d1-ab20-4ae5-9148-e71812ec7aeb
resultStatus: finished
finishedAt: 2026-05-27T11:33:29.069Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-preimage`

## What I did
- Audited `PreimageCommands` end-to-end from clap surface (`src/cli/mod.rs`) through `handle_preimage` (`src/cli/network_cmds.rs`) into subxt dynamic extrinsics (`src/chain/extrinsics.rs`) and runtime pallet wiring (`subtensor/runtime/src/lib.rs` + pinned `pallet-preimage` source at `opentensor/polkadot-sdk@7cc54bf...`).
- Updated `docs/commands/preimage.md` to fully document:
  - every current `preimage` subcommand (`note`, `unnote`)
  - clap flags + types
  - SCALE encoding path
  - exit codes (aligned to `src/error.rs`)
  - success/error JSON/output schema behavior
  - pallet/storage/event references, including in-scope unsurfaced dispatchables (`request_preimage`, `unrequest_preimage`).
- Added `tests/audit_preimage.rs` with:
  - parse-surface tests using `agcli::cli::Cli::try_parse_from` for all current preimage subcommands
  - one `#[ignore]` async local-chain green-path test that does `note_preimage` then `unnote_preimage`.

## Measurements
- `cargo check --all-targets exit code: 101 → 0`
- `cargo test --no-run --test audit_preimage exit code: 0 == 0`
- `documented preimage subcommands in docs/commands/preimage.md: 2 == 2`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- Findings:
  - `Preimage::request_preimage` and `Preimage::unrequest_preimage` exist on-chain but have no `agcli preimage` CLI surface today (missing command coverage for in-scope pallet dispatchables).
  - `agcli preimage note` / `unnote` ignore `--output json` on successful paths and always print plaintext; only dry-run preview/error paths are structured JSON. This is inconsistent with command groups that return structured success JSON.
  - `--args` conversion path (`parse_json_args` -> `json_to_subxt_value`) only maps numeric JSON to `u128/i128` when values fit `u64/i64`; larger integers become strings, which can break SCALE typing for high-range numeric arguments in preimage inner calls.
  - Existing parse-surface tests elsewhere currently include `preimage note --args "{}"` variants that pass clap but fail runtime validation (handler requires a JSON array), indicating parse-level tests can overstate green-path viability.
  - No panic was found in the audited preimage handler path; hash-length failures are handled with explicit errors.
- Deviation:
  - Local-chain integration was added as required but left `#[ignore]`; not executed in this VM.

## Suggested follow-ups
- Add `agcli preimage request` and `agcli preimage unrequest` subcommands, with matching `Client` extrinsic helpers and docs/tests.
- Standardize successful preimage command output to structured JSON when `--output json` is set.
- Improve `json_to_subxt_value` numeric handling to support full-width integer inputs safely (including values beyond `u64`).
- Add preimage-specific hints/classification coverage in `src/error.rs` for common pallet errors (`AlreadyNoted`, `NotNoted`, `NotAuthorized`, `Requested`, `NotRequested`, `TooBig`).
- Tighten parse test realism by aligning `--args` examples with handler validation rules (JSON array required).