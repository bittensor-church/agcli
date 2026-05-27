# subscribe — Real-Time Block & Event Streaming

Watch finalized blocks or decode and print Subtensor events as they finalize. Useful for monitoring, alerting, and integration pipelines. **No wallet** is required — only a WebSocket-capable RPC endpoint.

**Discoverability:** `agcli subscribe --help` / `agcli subscribe blocks --help` / `agcli subscribe events --help`. See also `docs/llm.txt` (Subscribe row).

## Subcommands

### subscribe blocks

Stream each **finalized** block (number, hash, extrinsic count). Runs until **Ctrl+C**; reconnects automatically if the WebSocket drops (exponential backoff, up to 5 attempts per failure before exiting with an error).

```bash
agcli subscribe blocks
agcli subscribe blocks --output json
```

**Clap flags / types**

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| *(none specific to this subcommand)* | | | |
| `--output` (global) | `table` \| `json` | `table` | Output format |
| `--network` (global) | `String` | `finney` | Network alias or `--endpoint` URL |

**Read path** — `src/cli/network_cmds.rs` `handle_subscribe` → `src/events.rs` `subscribe_blocks` → `subscribe_blocks_inner` → `client.blocks().subscribe_finalized()`. Each block: `block.number()`, `block.hash()`, `block.extrinsics().await` for count.

**Human output (default)**
```
Subscribed to finalized blocks. Ctrl+C to stop.

Block #4321001 hash=0xabc…def extrinsics=3
Block #4321002 hash=0x123…456 extrinsics=1
```
Gap warnings: `Warning: N block(s) missed (#X to #Y) — events in those blocks were not captured`

**JSON output** (`--output json`) — one object per line per block:
```json
{"block": 4321001, "hash": "0xabc…def", "extrinsics": 3}
```
Gap warning object:
```json
{"warning": "gap_detected", "missed_from": 4321001, "missed_to": 4321003, "missed_count": 2}
```

**Exit codes** (from `src/error.rs`)

| Code | Meaning |
|------|---------|
| 0 | Clean exit (Ctrl+C) |
| 1 | Generic/uncategorized error |
| 10 | Network error — persistent WebSocket failure after max reconnect attempts |
| 15 | Timeout |
| 2 | clap parse error (invalid global flags) |

**Pallet reference** — this subcommand does not invoke any pallet dispatchable; it calls the `chain_subscribeAllHeads` / `chain_subscribeFinalizedHeads` RPC path exposed by the subxt `blocks().subscribe_finalized()` API.

**On-chain events** — none emitted (read-only subscription); `subscribe blocks` counts but does not decode pallet events.

---

### subscribe events

Stream **decoded** runtime events from finalized blocks, with optional category, `--netuid`, and `--account` filters. Same long-running / Ctrl+C / reconnect behavior as **`subscribe blocks`**.

```bash
agcli subscribe events
agcli subscribe events --filter staking
agcli subscribe events --filter all --netuid 1
agcli subscribe events --filter transfer --account 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
agcli subscribe events --output json --filter weights
```

**Clap flags / types**

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--filter` | `String` | `"all"` | Event category — see Filter table below |
| `--netuid` | `Option<u16>` | *(absent)* | Only show events whose decoded fields include this netuid |
| `--account` | `Option<String>` | *(absent)* | Only show events that mention this SS58 address |
| `--output` (global) | `table` \| `json` | `table` | Output format |

**Validation** (before subscribing — `src/cli/network_cmds.rs` `handle_subscribe`):
- `validate_event_filter(filter)` — rejects unknown strings; exits **12**
- `validate_ss58(account)` — rejects invalid SS58; exits **12**

**Read path** — `handle_subscribe` → `events::subscribe_events_filtered` → `subscribe_events_inner` → `client.blocks().subscribe_finalized()` → per block `block.events().await` → iterate events → `EventFilter::matches(pallet, variant)` → optional `extract_netuid` / `extract_accounts` field filtering → print.

**Human output (default)**
```
Subscribed to finalized blocks (filter: Staking). Ctrl+C to stop.

#4321001 SubtensorModule::StakeAdded Named([(coldkey, …), (hotkey, …), (amount, …)])
```

**JSON output** (`--output json`) — one object per line per matching event:
```json
{
  "block": 4321001,
  "hash": "0xabc…def",
  "pallet": "SubtensorModule",
  "event": "StakeAdded",
  "fields": {"coldkey": "0x…", "hotkey": "0x…", "amount": 1000000000}
}
```
Note: the JSON key is `"event"`, not `"variant"` (the field name on the `ChainEvent` struct).

**Exit codes** (from `src/error.rs`)

| Code | Meaning |
|------|---------|
| 0 | Clean exit (Ctrl+C) |
| 1 | Generic/uncategorized error |
| 10 | Network error — persistent subscription failure after max reconnect |
| 12 | Validation error — unknown `--filter` value or invalid `--account` SS58 |
| 15 | Timeout |
| 2 | clap parse error (invalid global flags) |

**Pallet reference** — read-only. Events come from every pallet registered in the Subtensor runtime. The primary source is `SubtensorModule` (`subtensor/pallets/subtensor/src/macros/events.rs`), but `Balances`, `Crowdlan`, `Swap`, `SafeMode`, `Sudo`, `Scheduler`, `Proxy`, and `Multisig` events are also visible when using `--filter all` or the matching category filter.

---

## Filter categories (`--filter`)

Values are **case-insensitive**. Aliases match `EventFilter::FromStr` in `src/events.rs` and `validate_event_filter` in `src/cli/helpers.rs`.

| Filter (aliases) | Pallet matched | Known event variants |
|------------------|----------------|----------------------|
| `all` | *(all pallets)* | Every decoded event |
| `staking` (`stake`) | `SubtensorModule` | `StakeAdded`, `StakeRemoved`, `StakeMoved`, `StakeSwapped`, `StakeTransferred`, `AlphaRecycled`, `AlphaBurned`, `RootClaimed`, `AutoStakeAdded`, `AutoStakeDestinationSet` |
| `registration` (`register`, `reg`) | `SubtensorModule` | `NeuronRegistered`, `BurnedRegister`, `SubnetRegistered`, `PowRegistered`, `BulkNeuronsRegistered` |
| `transfer` (`transfers`) | `Balances` | All `Balances` pallet events (Transfer, Deposit, Withdraw, Endowed, …) |
| `weights` (`weight`) | `SubtensorModule` | `WeightsSet`, `WeightsCommitted`, `WeightsRevealed`, `WeightsBatchRevealed`, `CRV3WeightsCommitted`, `CRV3WeightsRevealed`, `TimelockedWeightsCommitted`, `TimelockedWeightsRevealed`, `BatchWeightsCompleted`, `BatchCompletedWithErrors`, `BatchWeightItemFailed`, `CommitRevealEnabled`, `CommitRevealPeriodsSet` |
| `subnet` (`subnets`) | `SubtensorModule` | `SubnetHyperparamsSet`, `SubnetIdentitySet`, `SubnetIdentityRemoved`, `NetworkAdded`, `NetworkRemoved`, `TempoSet`, `DissolveNetworkScheduled`, `SubnetLeaseCreated`, `SubnetLeaseTerminated`, `SubnetLeaseDividendsDistributed`, `SymbolUpdated`, `FirstEmissionBlockNumberSet`, `TransferToggle`, `SubnetOwnerHotkeySet` |
| `delegation` (`delegate`, `delegates`) | `SubtensorModule` | `DelegateAdded`, `TakeDecreased`, `TakeIncreased`, `ChildKeyTakeSet`, `SetChildren`, `SetChildrenScheduled` |
| `keys` (`key`) | `SubtensorModule` | `HotkeySwapped`, `HotkeySwappedOnSubnet`, `ColdkeySwapped`, `ColdkeySwapScheduled`†, `EvmKeyAssociated`, `ChainIdentitySet` |
| `swap` (`dex`, `liquidity`) | `Swap` | `SwapExecuted`†, `LiquidityAdded`, `LiquidityRemoved`, `PositionCreated`†, `PositionClosed`†, `FeesCollected`† |
| `governance` (`gov`, `sudo`, `safemode`) | `SafeMode`, `Sudo`, `Scheduler`, `Proxy`, `Multisig` | Entered/Exited/DepositPlaced/DepositReleased, Sudid/KeyChanged/KeyRotated/SudoAsDone, Scheduled/Canceled/Dispatched, ProxyExecuted/PureCreated/Announced/ProxyAdded/ProxyRemoved, NewMultisig/MultisigApproval/MultisigExecuted/MultisigCancelled |
| `crowdloan` (`crowdloans`, `fund`) | `Crowdloan` | `Created`, `Contributed`, `Withdrew`, `PartiallyRefunded`, `AllRefunded`, `Dissolved`, `Edited`† |

†  **Phantom/stale variants** — see [Audit Notes](#audit-notes--known-drift) below.

**`--netuid N`** — keeps only events whose decoded fields include a named `netuid` field equal to `N`. Events whose `netuid` is encoded as an **unnamed** positional tuple element (e.g. `WeightsSet(NetUidStorageIndex, u16)`) may not be matched by this filter even when the netuid matches. See audit note 4.

**`--account SS58`** — keeps only events that mention the given SS58 address in a composite field. Matching is exact string equality (case-sensitive, 32-byte AccountId decoded to SS58 with prefix 42).

---

## Source code

- **CLI handler:** `src/cli/network_cmds.rs` — `handle_subscribe()` (~L366)
- **Streaming logic:** `src/events.rs` — `subscribe_blocks`, `subscribe_events_filtered`, filter taxonomy, gap detection
- **Filter validation:** `src/cli/helpers.rs` — `validate_event_filter`
- **Pallet events reference:** `subtensor/pallets/subtensor/src/macros/events.rs`

---

## Related commands

- `agcli subnet monitor` — Higher-level subnet monitoring with summaries
- `agcli subnet watch` — Tempo / weight-window focused TUI-style watch
- `agcli block latest` — One-shot head snapshot (no WebSocket stream)

---

## Audit notes / Known drift

The following issues were identified in the `audit-subscribe` audit pass. They are documented here for agent awareness; source-level fixes are tracked separately.

**1. `SWAP_VARIANTS` has 4 phantom event names and 3 missing real ones.**
The `Swap` pallet (`subtensor/pallets/swap/src/pallet/mod.rs`) emits: `FeeRateSet`, `UserLiquidityToggled`, `LiquidityAdded`, `LiquidityRemoved`, `LiquidityModified`. The `SWAP_VARIANTS` constant in `src/events.rs` lists: `SwapExecuted`, `LiquidityAdded`, `LiquidityRemoved`, `PositionCreated`, `PositionClosed`, `FeesCollected`. The variants `SwapExecuted`, `PositionCreated`, `PositionClosed`, and `FeesCollected` do not exist on-chain and will never match. The real events `FeeRateSet`, `UserLiquidityToggled`, and `LiquidityModified` are missing. With the current code, `--filter swap` only captures `LiquidityAdded` and `LiquidityRemoved`.

**2. `KEY_VARIANTS` includes `ColdkeySwapScheduled` which is not a pallet event.**
`ColdkeySwapScheduled` is a deprecated storage map that was migrated (`migrate_coldkey_swap_scheduled_to_announcements.rs`), not an event. The pallet actually emits `ColdkeySwapAnnounced`, `ColdkeySwapReset`, `ColdkeySwapDisputed`, `AllBalanceUnstakedAndTransferredToNewColdkey`, and `ArbitrationPeriodExtended` for the coldkey lifecycle. None of these are in `KEY_VARIANTS`. The `--filter keys` will miss all real coldkey-swap lifecycle events.

**3. `STAKING_VARIANTS` includes `AllStakeRemoved` which does not exist on-chain.**
The subtensor pallet's event enum in `events.rs` has no `AllStakeRemoved` variant. This is a phantom that will never match. The staking filter otherwise captures real events correctly.

**4. `CROWDLOAN_VARIANTS` has `Edited` which does not exist; `Finalized` is missing.**
The crowdloan pallet emits `Finalized` when a crowdloan is successfully finalized and several update events (`MinContributionUpdated`, `EndUpdated`, `CapUpdated`). It has no `Edited` event. The `Edited` entry will never match. `Finalized` events will be invisible under `--filter crowdloan`.

**5. `--netuid` filtering only works for named composite fields.**
`extract_netuid` only checks `Composite::Named` fields for a `"netuid"` key. Subtensor events that encode netuid as an unnamed positional tuple (e.g. `WeightsSet(NetUidStorageIndex, u16)`) will return `None` from `extract_netuid` and be dropped when `--netuid` is active, even if the encoded value matches the filter. This means `--filter weights --netuid N` will silently drop `WeightsSet` events for subnet N.

**6. `EventFilter::from_str` has `Infallible` error — silently falls back to `All`.**
`EventFilter::from_str` returns `Ok(All)` for any string not in the match arms, including typos. The `validate_event_filter` call in `handle_subscribe` prevents invalid strings from reaching the `FromStr` parse, so in normal usage this is harmless. However the `.map_err(...)` call wrapping `filter.parse()` in `handle_subscribe` is dead code — it can never be triggered because `Infallible` is the error type.

**7. JSON `"event"` key vs `ChainEvent.variant` field name inconsistency.**
The `ChainEvent` struct has a field named `variant`, and the human-readable output uses `pallet::variant`. But the JSON output emits the key as `"event"` (not `"variant"`). Agents parsing JSON output by field name should use `"event"`, not `"variant"`.

**8. No `subscribe transactions` subcommand.**
There is no way to subscribe to pending extrinsics or to decode extrinsic contents from included blocks. The `subscribe_blocks_inner` function calls `block.extrinsics().await` only for the count, without exposing individual extrinsics. The `author_pendingExtrinsics` RPC method is not surfaced.
