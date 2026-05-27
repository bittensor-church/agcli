<!-- orchestrate handoff
task: fix-events-pretty
branch: orch/agcli-audit/fix-events-pretty
agentId: bc-2ed0cc06-b2ff-41b5-ae4a-486fdbf77216
runId: run-e23586a7-8639-4add-933c-6c5bbc892dd1
resultStatus: finished
finishedAt: 2026-05-27T13:59:42.631Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-events-pretty`

## What I did
- Updated `src/events.rs` filter variant sets so class filters match real on-chain events:
  - fixed `SWAP_VARIANTS` to real swap pallet events (`FeeRateSet`, `UserLiquidityToggled`, `LiquidityAdded`, `LiquidityRemoved`, `LiquidityModified`)
  - fixed `KEY_VARIANTS` to real coldkey/hotkey lifecycle events (removed `ColdkeySwapScheduled`, added announced/reset/disputed/cleared lifecycle variants, etc.)
  - fixed `STAKING_VARIANTS` (removed phantom `AllStakeRemoved`, added `AddStakeBurn`)
  - fixed `CROWDLOAN_VARIANTS` (removed phantom `Edited`, added `Finalized`, `MinContributionUpdated`, `EndUpdated`, `CapUpdated`)
  - extended delegation filter set with `AutoParentDelegationEnabledSet`.
- Added exhaustive subtensor event variant coverage in `src/events.rs` via `SUBTENSOR_VARIANTS` (125 entries) and routed non-JSON output through pretty variant naming.
- Added an exhaustive guard test (`pretty_subtensor_variants_are_exhaustive`) that parses `subtensor/pallets/subtensor/src/macros/events.rs` and fails if any pallet variant is missing/stale in `SUBTENSOR_VARIANTS`.
- Reworked netuid extraction to avoid tuple false-positives:
  - replaced generic `extract_netuid` with `extract_netuids(pallet, variant, composite)`
  - for unnamed tuple events, uses variant-aware netuid index positions (including multi-netuid events like `StakeMoved` / `StakeTransferred` / `StakeSwapped`).
- Stabilized JSON event schema/type behavior:
  - added canonical top-level `"variant"` key (kept `"event"` for compatibility)
  - added `"pretty_variant"` key
  - normalized integer primitive serialization (`U128`/`I128`) to strings to prevent number-vs-string drift across values.
- Updated in-file tests in `src/events.rs` to match corrected event names and new netuid extraction semantics.

## Measurements
- `cargo check --all-targets (exit code): 0 == 0`
- `cargo build --bin agcli (exit code): 0 == 0`
- `Subtensor event variants in pallet vs pretty-printer coverage: 125 == 125`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- **Handoff Findings (fixes + motivation links):**
  - Fixed stale swap/key/staking/crowdloan filter variant lists, per findings in [`audit-subscribe.md`](.orchestrate/agcli-audit/handoffs/audit-subscribe.md) (items 1–4).
  - Fixed netuid filtering for unnamed tuple events by using variant-aware tuple positions, per [`audit-subscribe.md`](.orchestrate/agcli-audit/handoffs/audit-subscribe.md) item 5.
  - Added canonical `"variant"` JSON field and retained compatibility `"event"` key, addressing schema inconsistency called out in [`audit-subscribe.md`](.orchestrate/agcli-audit/handoffs/audit-subscribe.md) item 7.
  - Implemented exhaustive subtensor event pretty coverage with a source-validated guard test to satisfy the cross-reference requirement against `subtensor/pallets/subtensor/src/macros/events.rs`.
  - Normalized integer JSON encoding to string for stable type shape across event values.
- Environment deviation: initial `cargo check` failed because the VM had Cargo 1.83 (edition2024 unsupported); switched toolchain to stable Rust 1.95 in-session, then all required commands passed.
- No PR was opened (per instruction). Branch pushed to `origin/orch/agcli-audit/fix-events-pretty`.

## Suggested follow-ups
- Add a focused integration test for `subscribe events --netuid` against localnet that asserts tuple-encoded events like `WeightsSet` are not dropped.
- If desired, remove legacy `"event"` key in a future breaking change after downstream consumers migrate to `"variant"`.
- Consider moving filter variant tables to a generated source path from pallet metadata to reduce future drift.