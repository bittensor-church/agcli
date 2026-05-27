# delegate — Delegation Take Management

`agcli delegate` manages validator take percentages. Delegates are hotkeys registered on the network; their take percentage determines the fraction of nominator emissions they retain. Nominators (coldkeys staking behind a hotkey) pay this fee proportionally.

## Subcommands

### `delegate list`

List the top 50 delegates ranked by total stake.

```
agcli delegate list [--output json|table|csv]
```

**No required flags.**

**Output columns (table / CSV):**

| Field | Description |
|---|---|
| `hotkey` | Delegate SS58 address |
| `owner` | Owning coldkey SS58 |
| `take_pct` | Take as a percentage (float, 2 d.p.) |
| `total_stake_rao` | Total staked behind this delegate in RAO |
| `nominators` | Number of distinct nominators |

**JSON key names** (machine-readable, `--output json`):

```json
[
  {
    "hotkey": "5G...",
    "owner": "5F...",
    "take_pct": 18.0,
    "total_stake_rao": 1000000000,
    "nominators": 3
  }
]
```

**Caveats:**
- Hard-capped at **50 delegates** regardless of actual delegate count. No `--limit` flag is provided.
- Sourced from the `DelegateInfo` runtime API (`delegate_info_runtime_api().get_delegates()`), which aggregates on-chain `Delegates` storage map + staking info.

**Exit codes:** `0` success · `10` network · `13` chain/RPC error.

**Pallet ref:** `SubtensorModule` · storage: `Delegates` (map `AccountId → u16`); runtime API: `DelegateInfoRuntimeApi::get_delegates`.

---

### `delegate show`

Show full info for a single delegate: take, total stake, nominators, subnet registrations, and validator-permit subnets.

```
agcli delegate show [--hotkey-address SS58]
```

| Flag | Type | Required | Description |
|---|---|---|---|
| `--hotkey-address` | SS58 string | no | Delegate hotkey. Defaults to the wallet's configured hotkey. |

**Output:** Human-readable lines printed to stdout. The `--output` format flag is **not** respected by this command — output is always plain text regardless of `--output json`.

Example:
```
Delegate: 5G...
  Owner:       5F...
  Take:        18.00%
  Total stake: 1000.00 TAO
  Nominators:  12
  Registrations: [1, 3]
  VP subnets:    [1]
  Top nominators:
    5D...abc — 500.00 TAO
```

**Exit codes:** `0` success · `11` auth/wallet error · `12` validation (bad SS58) · `10` network · `13` chain.

**Pallet ref:** `SubtensorModule` · runtime API: `DelegateInfoRuntimeApi::get_delegate(AccountId)`.

---

### `delegate decrease-take`

Decrease the take percentage for a hotkey you own. Effective immediately (no rate limit). The new value must be strictly lower than the current take and at or above the chain's `MinDelegateTake` storage value (default 9%, stored as `u16`).

```
agcli delegate decrease-take --take <PERCENT> [--hotkey-address SS58]
```

| Flag | Type | Required | Description |
|---|---|---|---|
| `--take` | `f64` (percentage) | **yes** | New take percentage, in the range `[0, 18]`. E.g. `10.5` means 10.5%. |
| `--hotkey-address` | SS58 string | no | Hotkey to update. Defaults to wallet hotkey. |

**SCALE encoding:** `take` is converted to `u16` as `(pct / 100.0 * 65535.0).round() as u16`. This matches the pallet's expectation (full-range linear, not per-mill).

**On-chain dispatchable:**
```
SubtensorModule::decrease_take(origin: coldkey, hotkey: AccountId, take: u16)
```
- `origin` must be the coldkey that owns `hotkey`.
- Call index: **65** (from `macros/dispatches.rs`).
- Weight class: `Normal`, `Pays::No`.

**Events emitted on success:**
- `SubtensorModule::TakeDecreased(coldkey: AccountId, hotkey: AccountId, take: u16)`

**Errors (pallet-level → exit code 13):**

| Error | Meaning |
|---|---|
| `DelegateTakeTooLow` | New take ≥ current take, or below `MinDelegateTake`. |
| `NotRegistered` / `HotKeyNotRegisteredInNetwork` | Hotkey not registered. |
| `NonAssociatedColdKey` | Caller's coldkey doesn't own the hotkey. |

**Exit codes:** `0` · `11` auth · `12` validation (bad SS58, out-of-range take) · `13` chain · `10` network.

**Pallet ref:** `subtensor/pallets/subtensor/src/staking/decrease_take.rs`.

---

### `delegate increase-take`

Increase the take percentage for a hotkey you own. **Rate-limited** — only one increase is allowed within the `TxDelegateTakeRateLimit` window (default ~300 blocks). The new value must be strictly higher than the current take and at or below `MaxDelegateTake` (default 18% = 11796/65535).

```
agcli delegate increase-take --take <PERCENT> [--hotkey-address SS58]
```

| Flag | Type | Required | Description |
|---|---|---|---|
| `--take` | `f64` (percentage) | **yes** | New take percentage, in the range `[0, 18]`. |
| `--hotkey-address` | SS58 string | no | Hotkey to update. Defaults to wallet hotkey. |

**SCALE encoding:** Same formula as `decrease-take`: `(pct / 100.0 * 65535.0).round() as u16`.

**On-chain dispatchable:**
```
SubtensorModule::increase_take(origin: coldkey, hotkey: AccountId, take: u16)
```
- Call index: **66** (from `macros/dispatches.rs`).
- Weight class: `Normal`, `Pays::No`.

**Events emitted on success:**
- `SubtensorModule::TakeIncreased(coldkey: AccountId, hotkey: AccountId, take: u16)`

**Errors (pallet-level → exit code 13):**

| Error | Meaning |
|---|---|
| `DelegateTakeTooHigh` | New take > `MaxDelegateTake` (currently 18%). |
| `DelegateTakeTooLow` | New take ≤ current take (must be strictly increasing). |
| `DelegateTxRateLimitExceeded` | Too many `increase_take` calls within the rate-limit window. |
| `NotRegistered` / `HotKeyNotRegisteredInNetwork` | Hotkey not registered. |
| `NonAssociatedColdKey` | Caller's coldkey doesn't own the hotkey. |

**Exit codes:** `0` · `11` auth · `12` validation · `13` chain · `10` network.

**Pallet ref:** `subtensor/pallets/subtensor/src/staking/increase_take.rs`.

---

## Storage keys (cross-reference)

| Storage item | Pallet | Encoding | Description |
|---|---|---|---|
| `Delegates` | `SubtensorModule` | `Blake2_128Concat AccountId → u16` | Take per hotkey. |
| `MaxDelegateTake` | `SubtensorModule` | `StorageValue → u16` | Chain-wide upper bound for take (default 18% = 11796). |
| `MinDelegateTake` | `SubtensorModule` | `StorageValue → u16` | Chain-wide lower bound (default 9%). |
| `TxDelegateTakeRateLimit` | `SubtensorModule` | `StorageValue → u64` | Block window for `increase_take` rate limit. |
| `LastTxBlockDelegateTake` | `SubtensorModule` | `map AccountId → u64` | Last block a hotkey called `increase_take`. |
| `NominatorMinRequiredStake` | `SubtensorModule` | `StorageValue → u64` | Minimum RAO a nominator must stake to remain active. |

---

## Admin-utils dispatchables (no agcli surface — audit finding)

The `admin-utils` pallet exposes `sudo_set_min_delegate_take(take: u16)` to change the on-chain minimum take. There is no `agcli delegate` or `agcli admin` subcommand that wraps this call. It can only be reached via `agcli admin raw --call sudo_set_min_delegate_take --args '[<u16>]'`.

There is no `sudo_set_max_delegate_take` exposed in `admin-utils` (chain uses `InitialDefaultDelegateTake` as the default for `MaxDelegateTake`, which can be mutated via a custom migration/sudo call if needed).

---

## Related commands

- `agcli stake add --hotkey-address SS58 --amount TAO` — Stake behind a delegate.
- `agcli view nominations --hotkey-address SS58` — See who nominates a given hotkey.
- `agcli admin set-nominator-min-stake --stake <u64>` — Set global nominator minimum stake (admin only).
- `agcli explain --topic take` — Conceptual explanation of take percentage.
- `agcli explain --topic delegation` — Delegation mechanics.

---

## Source locations

| Component | Path |
|---|---|
| CLI enum | `src/cli/mod.rs` — `DelegateCommands` |
| Handler | `src/cli/network_cmds.rs` — `handle_delegate()` |
| Extrinsics | `src/chain/extrinsics.rs` — `decrease_take()`, `increase_take()` |
| Queries | `src/chain/queries.rs` — `get_delegates()`, `get_delegate()`, `get_nominator_min_stake()` |
| Take validation | `src/cli/helpers.rs` — `validate_delegate_take()` |
| Pallet (decrease) | `subtensor/pallets/subtensor/src/staking/decrease_take.rs` |
| Pallet (increase) | `subtensor/pallets/subtensor/src/staking/increase_take.rs` |
| Dispatches | `subtensor/pallets/subtensor/src/macros/dispatches.rs` L600, L640 |
| Events | `subtensor/pallets/subtensor/src/macros/events.rs` L159, L161 |
