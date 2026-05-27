<!-- orchestrate handoff
task: audit-liquidity
branch: orch/agcli-audit/audit-liquidity
agentId: bc-06828c32-6eda-4a37-9dfd-68c1e804b4d1
runId: run-94bc4014-daa4-42c7-925c-f1fec42fb7ca
resultStatus: finished
finishedAt: 2026-05-27T11:45:56.828Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-liquidity`

## What I did
- Updated `docs/commands/swap.md` to fully document all `LiquidityCommands` subcommands (`add`, `remove`, `modify`, `toggle`) with:
  - clap flags and Rust types
  - exit-code mapping from `src/error.rs`
  - output JSON schema notes
  - pallet dispatch + SCALE argument mapping
  - storage keys and emitted events
  - swap-pallet coverage snapshot including in-scope dispatchables without agcli surface.
- Added `tests/audit_liquidity.rs` with:
  - parse-surface coverage for every liquidity subcommand via `agcli::cli::Cli::try_parse_from`
  - assertions on parsed values/types
  - one ignored local-chain integration test (`green_path_liquidity_toggle_local_chain`) that connects to a local node and submits `toggle_user_liquidity`.
- Traced `handle_liquidity` end-to-end (`src/cli/network_cmds.rs`) through `src/chain/extrinsics.rs` into `subtensor/pallets/swap/src/pallet/mod.rs` to verify dispatchable names and argument order/encoding.
- Ran required compile checks and test-target compile, then committed and pushed only allowed paths.

## Measurements
(none)

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- ### Findings
  - `agcli liquidity add` currently cannot succeed on-chain: `Swap::add_liquidity` immediately returns `Err(UserLiquidityDisabled)` and the functional body is commented out (`subtensor/pallets/swap/src/pallet/mod.rs:390-453`). This blocks a true add-liquidity green path.
  - `agcli liquidity toggle` dispatches, but runtime state mutation/event emission is currently disabled in pallet code (`EnabledUserLiquidity` insert + `UserLiquidityToggled` deposit commented at `subtensor/pallets/swap/src/pallet/mod.rs:371-373`), so successful tx may have no state/event effect.
  - Liquidity command output is not JSON-aware despite global `--output json`: `handle_liquidity` always uses `println!` paths and never uses `print_tx_result`/`ctx.output` for formatting (`src/cli/network_cmds.rs:2059-2151`), causing schema drift vs other tx commands.
  - Swap-pallet dispatchable coverage gap in agcli surface: `set_fee_rate` and `disable_lp` exist in pallet (`subtensor/pallets/swap/src/pallet/mod.rs:325,627`) but are not exposed by `LiquidityCommands` (`src/cli/mod.rs:2169-2224`).
  - SCALE mapping in agcli uses broad numeric dynamic values (`Value::u128`/`Value::i128`) for narrower pallet types (`u16`, `i32`, `u64`, `i64`), which currently coerces correctly but is less strict than exact-width encoding (`src/chain/extrinsics.rs:1006-1093`).
- No panicking paths were identified in `handle_liquidity` itself during this scoped audit.
- Ignored integration test is present but not executed by default; local Docker/localnet runtime execution remains environment-dependent.

## Suggested follow-ups
- Implement Swap-pallet-backed liquidity add path (or remove/feature-gate it) so `agcli liquidity add` has a real green path instead of deterministic `UserLiquidityDisabled`.
- Restore or intentionally redesign `toggle_user_liquidity` storage/event side effects in pallet code to match command/operator expectations.
- Add agcli surfaces for `Swap::set_fee_rate` and `Swap::disable_lp` (likely admin/root-scoped commands).
- Normalize liquidity command output handling to honor `--output json` with consistent tx schema (`{"tx_hash":"..."}`) and add event/receipt fields where available.