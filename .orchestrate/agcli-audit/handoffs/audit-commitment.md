<!-- orchestrate handoff
task: audit-commitment
branch: orch/agcli-audit/audit-commitment
agentId: bc-93f861ab-b9a6-4053-a45a-f1fab6020a42
runId: run-d38de302-19bf-46f4-99f2-a49af7e8309d
resultStatus: finished
finishedAt: 2026-05-27T11:33:30.143Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-commitment`

## What I did
- Updated `docs/commands/commitment.md` end-to-end for all `CommitmentCommands` subcommands (`set`, `get`, `list`) with:
  - clap flags + Rust types
  - exit-code mapping aligned to `src/error.rs`
  - JSON output schemas from `handle_commitment`
  - pallet/storage references from `subtensor/pallets/commitments`
  - emitted events coverage from the commitments pallet
  - explicit note of in-scope pallet functions not surfaced by `agcli commitment`
- Added `tests/audit_commitment.rs` containing:
  - parse-surface coverage for every commitment subcommand via `agcli::cli::Cli::try_parse_from`
  - one `#[ignore]` local-chain integration test (`green_path_commitment_local_chain`) that connects to a local WS endpoint and exercises commitment read path, with optional write roundtrip when `AGCLI_AUDIT_COMMITMENT_WRITE=1`
- Traced `handle_commitment` to subxt paths and pallet implementations:
  - `Set` -> `Client::set_commitment` -> dynamic tx `"Commitments"."set_commitment"` with `netuid` + `CommitmentInfo.fields`
  - `Get` -> `Client::get_commitment` -> storage `CommitmentOf(netuid, account)`
  - `List` -> `Client::get_all_commitments` -> `CommitmentOf` prefix iteration

## Measurements
- `tests/audit_commitment.rs files: 0 → 1`
- `commitment subcommands documented in docs/commands/commitment.md: 3 == 3`
- `cargo check --all-targets exit_code: 0 == 0`
- `cargo test --no-run --test audit_commitment exit_code: 0 == 0`
- `cargo test --test audit_commitment parse_surface_commitment_subcommands_with_realistic_args: 0 failing == 0 failing`

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- Findings:
  - Missing CLI surface for pallet functions in audit scope: `set_max_space` (root dispatchable), `reveal_timelocked_commitments` (hook/helper), and `purge_netuid` (helper) are present in `subtensor/pallets/commitments/src/lib.rs` but not exposed under `CommitmentCommands`.
  - Encoding capability drift: `Client::set_commitment` currently only emits `RawN` variants from comma-split string fields and silently truncates each field to 128 bytes (`src/chain/extrinsics.rs`), while pallet `Data` supports richer variants (`TimelockEncrypted`, hash variants, `BigRaw`, `ResetBondsFlag`).
  - Output-shape inconsistency: `commitment get --output json` returns two incompatible object shapes (`{hotkey,netuid,block,fields}` vs `{hotkey,netuid,found:false}`), while `commitment list --output json` returns a bare array without `netuid` context. This is awkward for strict downstream schema consumers.
  - Terminology mismatch risk: CLI argument is named `--hotkey-address` for `get`, but `set` signs with `wallet.coldkey()?` in `handle_commitment`, so docs/users can infer hotkey semantics that are not enforced at command level.
- Deviations:
  - Per scope constraints, no edits were made under `src/`; source-level issues are listed as follow-ups only.
  - Ignored local-chain integration test was added but not executed in this VM against Docker/localnet; compile and targeted parse test were executed.

## Suggested follow-ups
- Add explicit commitment subcommands for administrative/maintenance surfaces where appropriate (at minimum `set-max-space` as root/sudo; evaluate safe exposure for reveal/purge flows).
- Replace silent 128-byte per-field truncation in commitment encoding with explicit validation/error (or add typed CLI flags for selecting `Data` variants including timelocked/hash/bigraw/reset).
- Normalize commitment JSON outputs into stable envelope schemas (e.g., always include `found`, `netuid`, and predictable object shape for `get`; include `netuid` and `count` in `list`).
- Clarify and enforce signer/address semantics (`hotkey` vs signing key) for commitment set/get to prevent operator confusion and mismatched queries.