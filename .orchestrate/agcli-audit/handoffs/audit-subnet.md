<!-- orchestrate handoff
task: audit-subnet
branch: orch/agcli-audit/audit-subnet
agentId: bc-89380b0b-11ed-4793-a90f-ea0fe0c8d2b8
runId: run-a9e4922f-7d74-4a9a-93a2-aa9ce5949c0c
resultStatus: finished
finishedAt: 2026-05-27T11:48:16.079Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-subnet`

## What I did
- Updated `docs/commands/subnet.md` with an audited, command-by-command reference for all `SubnetCommands` entries, including:
  - clap flags + types
  - exit-code model (from `src/error.rs`)
  - output JSON schema notes
  - pallet/runtime/storage references
  - emitted-event references
  - explicit audit notes for call-shape drift
- Added `tests/audit_subnet.rs` with:
  - a parse-surface test that calls `agcli::cli::Cli::try_parse_from` across the full subnet command surface
  - one `#[ignore]` async local-chain smoke test (`green_path_subnet_local_chain_smoke`) gated for localnet availability
- Ran required verification commands:
  - `cargo check --all-targets`
  - `cargo test --no-run --test audit_subnet`
- Ran targeted parse test:
  - `cargo test --test audit_subnet parse_surface_subnet_all_subcommands`
- Committed and pushed changes to `origin/orch/agcli-audit/audit-subnet`.

## Measurements
- documented subnet headings in `docs/commands/subnet.md`: 31 → 35
- `SubnetCommands` variants in `src/cli/subnet_cmds.rs` vs documented headings: 35 == 35
- `cargo test --test audit_subnet parse_surface_subnet_all_subcommands`: 0 passing → 1 passing

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- Findings (concrete drift):
  - `register-leased` call-shape mismatch:
    - agcli encodes `register_leased_network(hotkey, end_block)` in `src/chain/extrinsics.rs` (around `register_leased_network`)
    - pallet expects `register_leased_network(emissions_share, end_block)` in `subtensor/pallets/subtensor/src/macros/dispatches.rs` (call index 110)
  - `terminate-lease` call-shape mismatch:
    - agcli CLI takes `--netuid` and submits one field via `terminate_lease(netuid)` in `src/chain/extrinsics.rs`
    - pallet expects `terminate_lease(lease_id, hotkey)` in `subtensor/.../macros/dispatches.rs` (call index 111)
  - `register-with-identity` identity-encoding mismatch:
    - agcli builds identity without `logo_url` (7 fields) in `src/chain/extrinsics.rs`
    - pallet `SubnetIdentityV3` includes `logo_url` (8 fields) in `subtensor/pallets/subtensor/src/lib.rs`
  - `subnet dissolve` semantic drift:
    - CLI labels this as owner flow
    - call used is `dissolve_network` which is root-gated in pallet (`ensure_root`), so behavior does not match command UX/wording
  - missing subnet surface for in-scope dispatchable:
    - `SubtensorModule::register_limit` exists in pallet (`dispatches.rs`), but no `agcli subnet` command exposes it
- Additional implementation note:
  - `subnet check-start` currently derives `active` from `NetworksAdded` (existence) path; this can diverge from “emissions started” intent.
- Deviation:
  - Local-chain integration test is intentionally `#[ignore]`; Docker/localnet was not started in this VM session.
- Environment:
  - Installed Doppler CLI per workspace rule; secret fetch failed here due missing Doppler token in this session (`you must provide a token`).

## Suggested follow-ups
- Fix lease extrinsic encodings in `src/chain/extrinsics.rs` + CLI args:
  - `register_leased_network` should encode `emissions_share` (Percent-compatible) + `end_block`
  - `terminate_lease` should take/encode `lease_id` + beneficiary `hotkey`
- Fix `register_network_with_identity` encoding to include `logo_url` for `SubnetIdentityV3`.
- Rework `subnet dissolve` to match intended owner flow (or rename/re-scope UX if root-only behavior is intentional).
- Add `subnet register-limit` command surface for `SubtensorModule::register_limit`.
- Review `subnet check-start` active-state source to use an emission-start signal rather than subnet-exists flag.