<!-- orchestrate handoff
task: audit-safe-mode
branch: orch/agcli-audit/audit-safe-mode
agentId: bc-de68145c-3a27-4db6-a80f-9897d229e8dc
runId: run-c3e2cced-3b6e-40d6-8bb7-d3c38fed66e1
resultStatus: finished
finishedAt: 2026-05-27T13:07:01.760Z
-->

Branch pushed. Now writing the handoff as required:

---

## Status
success

## Branch
`orch/agcli-audit/audit-safe-mode`

## What I did

- **`tests/audit_safe_mode.rs`** (new, ~240 LOC): 17 parse-surface tests covering all 4 `SafeModeCommands` subcommands (`enter`, `extend`, `force-enter`, `force-exit`), including: variant matching, required-flag rejection, u32 boundary checks (zero, max, overflow), unknown-flag rejection, global-flag threading (`--yes`, `--wallet`, `--network`), a documentation test that records the `safe-mode status` doc-comment drift, and a `force_enter` encoding-mismatch documentation test. Plus one `#[ignore]`-gated `green_path_safe_mode` tokio test that checks metadata pallet existence, call names, and storage.
- **`docs/commands/safe-mode.md`** (full rewrite, +590 LOC total): Every subcommand documented with clap flags + types, exit codes cross-referenced to `src/error.rs`, SCALE encoding details, pallet call indices (0–7), on-chain events, storage keys (`EnteredUntil`, `Deposits`), a pallet-vs-CLI coverage table, and stubs for all missing subcommands.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0 (no regression)
- `cargo test --no-run --test audit_safe_mode`: exit 0 (compiles)
- `cargo test --test audit_safe_mode -- --skip green_path_safe_mode`: 17 passed, 0 failed, 1 filtered (ignored)

## Verification
`unit-test-verified` — 17 parse-surface tests pass locally. The localnet integration test is `#[ignore]`-gated (Docker unavailable in VM).

## Findings

1. **CRITICAL: `force-enter` encodes a spurious `duration` argument that the pallet does not accept.** `src/chain/extrinsics.rs::safe_mode_force_enter` passes `vec![Value::u128(duration as u128)]` to `SafeMode::force_enter`. The opentensor/polkadot-sdk `pallet-safe-mode` `force_enter` call (call index 1) takes **no arguments** — the duration is extracted from the `ForceEnterOrigin` success type, not from a call field. On a live chain this extrinsic will fail with a SCALE decode error; the `--duration` flag has no on-chain effect. The clap surface (`ForceEnter { duration: u32 }`) and the handler both need to be removed/reworked.

2. **Doc-comment drift: `Commands::SafeMode` lists a `status` subcommand that does not exist.** `src/cli/mod.rs` line 379 says `/// Safe mode operations (enter, extend, force-enter, force-exit, status)`. There is no `SafeModeCommands::Status` variant. `agcli safe-mode status` returns a parse error. No read path for `EnteredUntil` storage is implemented.

3. **`force_extend` (pallet call index 3) has no CLI surface.** `SafeMode::force_extend` (privileged extension without a deposit) exists in the pallet but is not surfaced in `SafeModeCommands`. Operators cannot extend safe mode duration without tying up their own funds.

4. **`release_deposit` (pallet call index 6) has no CLI surface.** Accounts that placed deposits via `enter` or `extend` have no way to recover their TAO through agcli once safe mode ends and the release delay passes.

5. **`force_release_deposit` (pallet call index 7) and `force_slash_deposit` (call index 5) have no CLI surface.** Administrative deposit management operations are entirely absent.

6. **Output is plain text, not JSON.** All four handlers use `println!` with human-readable strings. `--output json` is accepted by the global parser but silently ignored in safe-mode handlers. Agents cannot reliably parse the tx hash or event data.

7. **`Entered { until }` and `DepositPlaced { amount }` events are not surfaced.** `enter` and `extend` print only the tx hash. The block until which safe mode is active (`Entered.until`) and the amount reserved (`DepositPlaced.amount`) are never displayed, forcing an agent to do a separate `storage` query or event scan.

## Notes, concerns, deviations, findings, thoughts, feedback

- The subtensor submodule (`subtensor/`) could not be initialized during this run (network timeout), but the opentensor/polkadot-sdk fork rev `7cc54bf2d50ae3921d718736dfeb0de9468539c7` was fetched directly from GitHub. The `pallet-safe-mode` code confirms standard FRAME behavior with no custom modifications to the `force_enter` signature.
- The `--duration` flag on `force-enter` likely originates from a misreading of the FRAME docs: the `ForceEnterOrigin::Success` value (which IS the duration) is part of the origin config, not a call parameter. The fix is to drop the `duration` field entirely and let the runtime's configured `ForceEnterOrigin` determine the duration.

## Suggested follow-ups

- **Planner (critical):** create a task to remove the `duration` field from `safe_mode_force_enter` in `src/chain/extrinsics.rs` and from `SafeModeCommands::ForceEnter` in `src/cli/mod.rs`. The pallet call takes no arguments; the current implementation always fails on-chain.
- **Planner:** add `safe-mode status` subcommand that reads `SafeMode::EnteredUntil` storage and prints `{ "active": bool, "until": BlockNumber | null }`.
- **Planner:** add `safe-mode force-extend` wrapping `SafeMode::force_extend` (sudo, no args).
- **Planner:** add `safe-mode release-deposit --account <SS58> --block <u32>` wrapping `SafeMode::release_deposit`.
- **Planner:** subscribe to `SafeMode::Entered` / `SafeMode::Extended` events in `enter` and `extend` handlers and print `until` block and deposit amount.
- **Planner:** wire `--output json` through all four safe-mode handlers.