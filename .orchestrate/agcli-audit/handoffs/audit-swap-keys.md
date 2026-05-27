<!-- orchestrate handoff
task: audit-swap-keys
branch: orch/agcli-audit/audit-swap-keys
agentId: bc-2028ef5c-481e-4895-a3d6-5d4ac5d9ff0c
runId: run-e0e880de-4279-487f-b3d5-e667de3ea594
resultStatus: finished
finishedAt: 2026-05-27T13:03:20.129Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-swap-keys`

## What I did
- Read `.orchestrate/agcli-audit/discovery.md` for environment constraints and the subtensor pallet cross-reference map.
- Enumerated all 3 `SwapCommands` variants (`Hotkey`, `Coldkey`, `EvmKey`) in `src/cli/mod.rs` (L1728–L1754) and traced each through `handle_swap` in `src/cli/network_cmds.rs` (L278–L361) to its extrinsic in `src/chain/extrinsics.rs`.
- Cross-referenced every call against the actual pallet dispatches in `subtensor/pallets/subtensor/src/macros/dispatches.rs` and events in `events.rs`.
- **Created `tests/audit_swap_keys.rs`**: 15 parse-surface tests (happy-path, field-value assertions, missing-arg rejection) covering all 3 variants + 1 `#[ignore]` localnet green-path stub. All 15 non-ignored tests pass.
- **Rewrote `docs/commands/swap.md`**: every subcommand documented with clap flags + types, exit codes, output JSON schema, pallet call ref + call index, SCALE argument table, events emitted, storage keys written. New §Missing CLI Surface table lists 7 unimplemented dispatchables.
- Ran `cargo check --all-targets` (clean) and `cargo test --test audit_swap_keys` (15 passed, 1 ignored).

## Measurements
- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --test audit_swap_keys`: 15 passed; 0 failed; 1 ignored
- `tests/audit_swap_keys.rs`: 0 LOC → 213 LOC
- `docs/commands/swap.md`: 43 LOC → 194 LOC

## Verification
`unit-test-verified` — 15 parse-surface tests cover every `SwapCommands` variant via `Cli::try_parse_from` and assert correct field values. `cargo test --test audit_swap_keys` passes in this VM.

## Notes, concerns, deviations, findings, thoughts, feedback

**Findings:**

1. **`swap coldkey` always fails on-chain — calls deprecated `schedule_swap_coldkey`**. `src/chain/extrinsics.rs:726` submits `api::tx().subtensor_module().schedule_swap_coldkey(new_id)`. The pallet dispatch at call index 73 is `#[deprecated]` and immediately returns `Err(Error::Deprecated)`. Any agent calling `agcli swap coldkey` will receive a chain error on the current network. The replacement two-step flow (`announce_coldkey_swap` → `swap_coldkey_announced`) is entirely absent from the CLI.

2. **`swap evm-key` is missing `--netuid` and has a `block_number` type mismatch**. The pallet's `associate_evm_key` (call index 93) signature is `(origin, netuid: NetUid, evm_key: H160, block_number: u64, signature: Signature)`. The agcli extrinsic at `src/chain/extrinsics.rs:2279` submits only `[evm_address, block_number, signature]` — `netuid` is entirely absent. Additionally, the CLI parses `--block-number` as `u32` but the pallet expects `u64`. Both mismatches will produce SCALE decode errors at the node — this subcommand cannot succeed on-chain.

3. **Seven pallet swap dispatchables have no CLI surface**: `announce_coldkey_swap` (index 125), `swap_coldkey_announced` (index 126), `dispute_coldkey_swap` (index 127), `reset_coldkey_swap` (index 128), `clear_coldkey_swap_announcement` (index 133), `swap_coldkey` (root-only, index 71), `swap_hotkey_v2` (index 72, adds per-subnet and `keep_stake` options). An agent cannot perform the current coldkey rotation flow, dispute a swap, clear an announcement, or use the enhanced hotkey swap variant through `agcli`.

4. **`swap hotkey` ignores `--netuid` option** — the handler hardcodes `netuid: None` in the `swap_hotkey` call, always swapping across all subnets. There is no way to do a subnet-scoped hotkey swap via agcli (which would require `swap_hotkey_v2` with a `Some(netuid)` argument).

5. **All swap write subcommands ignore `--output json`** — `swap hotkey` and `swap coldkey` use `println!()` directly without checking `ctx.output.is_json()`. Only `swap evm-key` uses the `print_tx_result` helper (which is JSON-aware). This is inconsistent and breaks machine-readable output for the most commonly-used swap operations.

6. **`swap coldkey` docs previously described the old `schedule_swap_coldkey` as a working two-phase flow** — the docs listed `announce_coldkey_swap` + `swap_coldkey_announced` as the "on-chain two-phase" behavior but the code actually calls the deprecated single-step extrinsic. The docs were aspirational rather than descriptive.

## Suggested follow-ups
- Fix `swap coldkey` to call `announce_coldkey_swap(BlakeTwo256::hash(new_coldkey.encode()))` instead of `schedule_swap_coldkey`. Add a separate `swap coldkey-exec --new-coldkey <SS58>` subcommand for `swap_coldkey_announced`.
- Add `swap coldkey-dispute`, `swap coldkey-clear`, `swap coldkey-reset --coldkey <SS58>` subcommands for the remaining coldkey rotation lifecycle calls.
- Fix `swap evm-key` to add `--netuid <N>` flag and change `--block-number` from `u32` to `u64`; update `src/chain/extrinsics.rs:associate_evm_key` to pass `netuid` as the first argument.
- Add `swap hotkey-v2 --new-hotkey <SS58> [--netuid <N>] [--keep-stake]` subcommand wiring to `swap_hotkey_v2`.
- Add `--output json` support to `swap hotkey` and `swap coldkey` handlers (parallel issue across all write operations as noted in the proxy audit).