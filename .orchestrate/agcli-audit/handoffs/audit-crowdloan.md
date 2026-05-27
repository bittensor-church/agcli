<!-- orchestrate handoff
task: audit-crowdloan
branch: orch/agcli-audit/audit-crowdloan
agentId: bc-7a5ff33b-cc4a-4a7a-bc23-604514a9ef46
runId: run-a1b8c5ef-603b-4294-834d-30d418aa117c
resultStatus: finished
finishedAt: 2026-05-27T11:46:25.485Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-crowdloan`

## What I did
- Updated `docs/commands/crowdloan.md` to fully document all 12 crowdloan subcommands:
  - clap flags + types
  - exit code mapping from `src/error.rs`
  - output JSON shapes (including current tuple-array behavior / text-only paths)
  - pallet dispatchable refs
  - storage keys from `subtensor/pallets/crowdloan/src/lib.rs`
  - emitted events for create/contribute/withdraw/finalize/refund/dissolve/update_min_contribution/update_end/update_cap
- Added `tests/audit_crowdloan.rs` with:
  - parse-surface coverage using `agcli::cli::Cli::try_parse_from` for every crowdloan subcommand
  - one `#[ignore]` localnet green-path integration test (`green_path_crowdloan_localnet`) that starts localnet, submits `create` + `contribute`, and asserts `get_crowdloan_info(0)` exists
- Audited `CrowdloanCommands`/`handle_crowdloan` to `Client`/subxt call mapping against the subtensor crowdloan pallet dispatchables/events/storage and captured drift findings below.

## Measurements
- `documented crowdloan subcommand sections: 6 → 12`
- `LOC(docs/commands/crowdloan.md): 63 → 357`
- `LOC(tests/audit_crowdloan.rs): 0 → 182`
- `cargo check --all-targets exit code: 0 == 0`
- `cargo test --no-run --test audit_crowdloan exit code: 0 == 0`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- Findings:
  - `src/chain/queries.rs` uses storage key `"Contributors"` for contributors iteration (`get_crowdloan_contributors`), but pallet storage is `Contributions` (`subtensor/pallets/crowdloan/src/lib.rs`), so this read path is drifted and likely fails or returns wrong behavior.
  - `list_crowdloans` / `get_crowdloan_info` decode `Crowdloan::Crowdloans` into a tuple shape that does not match the current pallet `CrowdloanInfo` field layout (current pallet includes `funds_account` and `contributors_count`, and different field ordering). This is a high-risk decode drift.
  - `crowdloan create` hardcodes `call = None` in `src/chain/extrinsics.rs`, so the pallet’s optional dispatchable call surface in `Crowdloan::create(..., call, ...)` is not exposed through agcli.
  - Write subcommands in `handle_crowdloan` print plain text `"<action>. Tx: <hash>"` regardless of `--output json`; output-format behavior is inconsistent with broader CLI expectations.
  - `crowdloan info` is text-only (no JSON/CSV shaping), while `list` and `contributors` do have output-mode branching; output ergonomics are inconsistent within the same command group.
- Deviation: localnet green-path integration test is intentionally `#[ignore]` and was compiled but not executed in this run.

## Suggested follow-ups
- Fix contributors query storage key in `src/chain/queries.rs` from `"Contributors"` to `"Contributions"` and add a regression test.
- Replace manual tuple decoding of crowdloan storage with metadata-derived typed decoding aligned to current `CrowdloanInfo` layout.
- Add optional `--call` surface to `agcli crowdloan create` (or explicitly document that agcli supports target-only crowdloans).
- Normalize crowdloan output-mode handling so write commands and `info` support structured JSON consistently (ideally object-shaped JSON instead of tuple arrays).