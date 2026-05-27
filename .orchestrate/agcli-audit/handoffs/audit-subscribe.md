<!-- orchestrate handoff
task: audit-subscribe
branch: orch/agcli-audit/audit-subscribe
agentId: bc-1577760e-8218-4d2e-b475-6cc7ef81de60
runId: run-ea9fade0-71cd-4ba8-b12d-f1c39d67f080
resultStatus: finished
finishedAt: 2026-05-27T13:19:15.875Z
-->

No PR template in the main repo. Per the task instructions, I should NOT open a PR (the planner owns integration). The task says "Do NOT merge, rebase, or open a PR." Let me complete the handoff.

---

## Status
success

## Branch
`orch/agcli-audit/audit-subscribe`

## What I did
- Read `.orchestrate/agcli-audit/discovery.md` for environment constraints, pallet mapping, and the subtensor submodule reference.
- Enumerated all 2 `SubscribeCommands` variants (`Blocks`, `Events`) in `src/cli/mod.rs` (L442–L457) and traced each through `handle_subscribe` in `src/cli/network_cmds.rs` (L366–L404) to `src/events.rs` (`subscribe_blocks`, `subscribe_events_filtered`).
- Cross-referenced `EventFilter` variants and their `STAKING_VARIANTS`, `SWAP_VARIANTS`, `KEY_VARIANTS`, `CROWDLOAN_VARIANTS` filter lists against the actual pallet event enums in `subtensor/pallets/subtensor/src/macros/events.rs`, `subtensor/pallets/swap/src/pallet/mod.rs`, and `subtensor/pallets/crowdloan/src/lib.rs`.
- **Created `tests/audit_subscribe.rs`**: 41 non-ignored parse-surface tests (all pass) + 1 `#[ignore]` localnet integration test. Tests cover both `Blocks` and `Events` subcommands, all 27 valid filter aliases, `--netuid`, `--account`, `validate_event_filter`, and `EventFilter::from_str`.
- **Rewrote `docs/commands/subscribe.md`**: every subcommand documented with clap flags+types, exit codes (from `src/error.rs`), JSON output schema, pallet ref, and an Audit Notes section listing 8 concrete drift items.
- Ran `cargo check --all-targets` (clean) and `cargo test --test audit_subscribe` (41 passed, 1 ignored).

## Measurements
- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --test audit_subscribe`: 41 passed; 0 failed; 1 ignored
- `tests/audit_subscribe.rs`: 0 LOC → 268 LOC
- `docs/commands/subscribe.md`: 63 LOC → 209 LOC

## Verification
`unit-test-verified` — 41 parse-surface tests cover every `SubscribeCommands` variant and all filter aliases via `Cli::try_parse_from`; `cargo test --test audit_subscribe` passes in this VM. The `#[ignore]`d localnet test is not verified (Docker unavailable in cloud-agent VM per discovery.md).

## Findings

1. **`SWAP_VARIANTS` has 4 phantom events and is missing 3 real ones.** The `Swap` pallet emits: `FeeRateSet`, `UserLiquidityToggled`, `LiquidityAdded`, `LiquidityRemoved`, `LiquidityModified`. But `SWAP_VARIANTS` in `src/events.rs` lists `SwapExecuted`, `LiquidityAdded`, `LiquidityRemoved`, `PositionCreated`, `PositionClosed`, `FeesCollected`. The four variants `SwapExecuted`, `PositionCreated`, `PositionClosed`, and `FeesCollected` do not exist in the pallet and will never match. The real events `FeeRateSet`, `UserLiquidityToggled`, and `LiquidityModified` are entirely absent. Result: `--filter swap` only captures 2 of 5 real swap events.

2. **`KEY_VARIANTS` includes `ColdkeySwapScheduled` which is a deprecated storage map, not a pallet event.** The real coldkey lifecycle events (`ColdkeySwapAnnounced`, `ColdkeySwapReset`, `ColdkeySwapDisputed`, `AllBalanceUnstakedAndTransferredToNewColdkey`, `ArbitrationPeriodExtended`) are entirely absent from `KEY_VARIANTS`. Result: `--filter keys` is completely blind to the entire coldkey swap announcement/reset/dispute lifecycle.

3. **`STAKING_VARIANTS` includes `AllStakeRemoved` which does not exist on-chain.** The subtensor `events.rs` has no `AllStakeRemoved` variant. This is a phantom that consumes a slot in the filter list but never matches any real event.

4. **`CROWDLOAN_VARIANTS` has `Edited` (does not exist) and is missing `Finalized`.** The crowdloan pallet emits `Finalized` when a crowdloan is successfully finalized. It has no `Edited` event — update events are named `MinContributionUpdated`, `EndUpdated`, `CapUpdated`. `--filter crowdloan` will miss all `Finalized` events.

5. **`--netuid` filter silently drops events whose netuid is encoded as an unnamed positional tuple.** `extract_netuid` in `src/events.rs` only handles `Composite::Named` fields. Many subtensor events encode netuid as an unnamed positional element (e.g. `WeightsSet(NetUidStorageIndex, u16)`). For these, `extract_netuid` returns `None` and the event is dropped. `--filter weights --netuid N` will silently drop `WeightsSet` events for subnet N.

6. **`EventFilter::from_str` has `Infallible` error — the `map_err` in `handle_subscribe` is dead code.** `filter.parse().map_err(|e| anyhow::anyhow!("Invalid event filter '{}': {}", filter, e))` in `network_cmds.rs:394` can never trigger because `EventFilter::FromStr::Err = std::convert::Infallible`. The validation is correctly done before this point by `validate_event_filter`, but the error message in the `.map_err` closure is unreachable.

7. **JSON key `"event"` is inconsistent with the `ChainEvent.variant` struct field name.** The JSON output (`src/events.rs:617–624`) emits `"event": variant` but the `ChainEvent` struct has a field named `variant` and human-readable output uses `pallet::variant`. Agents parsing event JSON by field name must use `"event"`, not `"variant"`.

8. **No `subscribe transactions` subcommand.** There is no way to subscribe to pending or included extrinsics. `subscribe_blocks_inner` calls `block.extrinsics().await` only for the count; individual extrinsic contents are not decoded or emitted.

## Suggested follow-ups
- Fix `SWAP_VARIANTS` to match the actual Swap pallet event enum: remove `SwapExecuted`, `PositionCreated`, `PositionClosed`, `FeesCollected`; add `FeeRateSet`, `UserLiquidityToggled`, `LiquidityModified`.
- Fix `KEY_VARIANTS`: remove `ColdkeySwapScheduled`; add `ColdkeySwapAnnounced`, `ColdkeySwapReset`, `ColdkeySwapDisputed`, `AllBalanceUnstakedAndTransferredToNewColdkey`, `ArbitrationPeriodExtended`.
- Fix `STAKING_VARIANTS`: remove `AllStakeRemoved`.
- Fix `CROWDLOAN_VARIANTS`: remove `Edited`; add `Finalized`, `MinContributionUpdated`, `EndUpdated`, `CapUpdated`.
- Fix `extract_netuid` in `src/events.rs` to also handle `Composite::Unnamed` by checking if any unnamed `u128` value is in the u16 range and treating the first such value as the netuid (or better, document a known-netuid-position-per-event approach).
- Remove the dead `map_err` in `handle_subscribe` (`network_cmds.rs:392–394`) — or replace it with a `let f: EventFilter = filter.parse();` (infallible) and document the rely on prior `validate_event_filter`.
- Rename JSON key `"event"` → `"variant"` in `subscribe_events_inner` output (or vice versa — pick one and update docs); breaking change requiring a version bump.
- Add `agcli subscribe transactions` subcommand using subxt's extrinsic decoding to stream included extrinsics with pallet/call name + signer.