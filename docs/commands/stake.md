# stake — Staking Operations

Lock TAO on subnets behind hotkeys to earn emission rewards. Staking converts TAO into subnet-specific alpha tokens via the AMM pool.

**22 subcommands** (alphabetically in this file; grouped by function below):

| Subcommand | Write? | Pallet dispatchable |
|---|---|---|
| `stake list` | No | — (runtime API `get_stakes_info_for_coldkey`) |
| `stake add` | Yes | `SubtensorModule::add_stake` |
| `stake remove` | Yes | `SubtensorModule::remove_stake` |
| `stake move` | Yes | `SubtensorModule::move_stake` |
| `stake swap` | Yes | `SubtensorModule::swap_stake` |
| `stake unstake-all` | Yes | `SubtensorModule::unstake_all` |
| `stake unstake-all-alpha` | Yes | `SubtensorModule::unstake_all_alpha` |
| `stake add-limit` | Yes | `SubtensorModule::add_stake_limit` |
| `stake remove-limit` | Yes | `SubtensorModule::remove_stake_limit` |
| `stake remove-full-limit` | Yes | `SubtensorModule::remove_stake_full_limit` (raw call) |
| `stake swap-limit` | Yes | `SubtensorModule::swap_stake_limit` |
| `stake recycle-alpha` | Yes | `SubtensorModule::recycle_alpha` |
| `stake burn-alpha` | Yes | `SubtensorModule::burn_alpha` |
| `stake claim-root` | Yes | `SubtensorModule::claim_root` |
| `stake process-claim` | Yes | `SubtensorModule::claim_root_dividends` (raw call) |
| `stake childkey-take` | Yes | `SubtensorModule::set_childkey_take` |
| `stake set-children` | Yes | `SubtensorModule::set_children` |
| `stake set-auto` | Yes | `SubtensorModule::set_coldkey_auto_stake_hotkey` (raw call) |
| `stake show-auto` | No | storage `ColdkeyAutoStakeHotkey(coldkey, netuid)` |
| `stake set-claim` | Yes | `SubtensorModule::set_root_claim_type` (raw call) |
| `stake transfer-stake` | Yes | `SubtensorModule::transfer_stake` |
| `stake wizard` | Yes | `SubtensorModule::add_stake` (interactive wrapper) |

---

## stake list — Positions per coldkey (read-only)

List **alpha stake positions** for a coldkey (default wallet coldkey or `--address`). Optional **historical** snapshot at a block height. No extrinsic; no wallet unlock unless the default coldkey must be read from disk.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--address` | `String` (SS58) | No | Coldkey address. Defaults to wallet coldkey. |
| `--at-block` | `u32` | No | Query stake at a specific block number (historical). |

### Examples

```bash
agcli stake list
agcli stake list --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --output json stake list --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --output csv stake list
agcli stake list --at-block 4000000
agcli --network archive stake list --at-block 3500000 --address 5Gr...
```

### Output JSON schema

With `--output json`:
```json
[
  {
    "netuid": 1,
    "hotkey": "5FHne...",
    "stake_rao": 1000000000,
    "alpha_raw": 987654321
  }
]
```

With `--output csv`, header: `netuid,hotkey,stake_rao,alpha_raw`.

Empty portfolio → empty array (JSON) / empty body (CSV) / `No stakes found for …` (human).

### Exit codes

| Code | When |
|------|------|
| **0** | Success (including empty stake list). |
| **2** | Clap / invalid global flags. |
| **10** | Network / WebSocket failure on `connect` or hard RPC errors. |
| **12** | Validation: invalid `--address` (SS58), message contains `stake list --address`. |
| **15** | Timeout. |
| **1** | `Block N not found` for `--at-block`; could not resolve coldkey. |

### Pallet ref

Storage: `SubtensorModule::Alpha(hotkey, coldkey, netuid)` (individual), or via runtime API `get_stakes_info_for_coldkey(coldkey)`.

---

## stake add — Stake TAO on a subnet

Convert **free TAO** from the coldkey into **alpha** on a subnet for a hotkey. Uses AMM `swap_tao_for_alpha`.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` (TAO) | **Yes** | Amount of TAO to stake. |
| `--netuid` | `u16` | **Yes** | Target subnet UID (≥1). |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |
| `--max-slippage` | `f64` (%) | No | Abort if slippage exceeds this %. |

### Examples

```bash
agcli stake add --amount 10.0 --netuid 1 --password p --yes
agcli stake add --amount 1.0 --netuid 1 --hotkey-address 5FHne... --password p --yes
agcli stake add --amount 5.0 --netuid 1 --max-slippage 2.0 --password p --yes
```

### Validation sequence

1. `validate_netuid(netuid)` — rejects netuid 0 (root network reserved)
2. `validate_amount(amount, "stake amount")` — positive, finite
3. `check_spending_limit(netuid, amount)` — optional config cap
4. `unlock_and_resolve` — wallet unlock
5. Balance preflight: `get_balance(&coldkey_pub)` — bails with "Insufficient balance" if free TAO < amount
6. If `--max-slippage`: `check_slippage` (buy path) — `try_join!(current_alpha_price, sim_swap_tao_for_alpha)`, aborts if slippage exceeds cap, warns if slippage > 2%
7. `add_stake_mev(&pair, &hk, NetUid(netuid), Balance::from_tao(amount), mev)`

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **2** | Clap / invalid global flags. |
| **10** | Network failure. |
| **11** | Auth: wallet / password / hotkey resolution. |
| **12** | Validation: netuid 0, negative amount (`stake amount` label), spending limit exceeded. |
| **13** | Chain: insufficient balance; slippage exceeded (`maximum allowed`); `NotEnoughBalanceToStake`, `HotKeyAccountNotExists`, `StakingRateLimitExceeded`. |
| **15** | Timeout. |
| **1** | Uncategorized. |

### Pallet ref

`SubtensorModule::add_stake(origin, hotkey, netuid, amount_staked)` →
`stake_into_subnet()` → AMM `swap_tao_for_alpha()` → `Alpha(hotkey, coldkey, netuid)`.

**Events emitted**: `StakeAdded(coldkey, hotkey, netuid, amount, alpha)`.

---

## stake remove — Unstake alpha to free TAO

Burn **alpha** on a subnet for a hotkey, receiving **free TAO** on the coldkey via AMM `swap_alpha_for_tao`. No client-side balance preflight (unlike `stake add`).

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` (TAO-scale) | **Yes** | Amount to unstake. |
| `--netuid` | `u16` | **Yes** | Source subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |
| `--max-slippage` | `f64` (%) | No | Abort if slippage exceeds this %. |

### Examples

```bash
agcli stake remove --amount 1.0 --netuid 1 --password p --yes
agcli stake remove --amount 2.0 --netuid 1 --max-slippage 2.0 --password p --yes
```

### Validation sequence

1. `validate_netuid(netuid)` — rejects netuid 0
2. `validate_amount(amount, "unstake amount")` — positive, finite
3. `unlock_and_resolve` — wallet unlock
4. If `--max-slippage`: `check_slippage(..., is_buy=false)` using `sim_swap_alpha_for_tao`
5. `remove_stake_mev(&pair, &hk, NetUid(netuid), Balance::from_tao(amount), mev)`

**No `check_spending_limit`** — unstaking returns funds.

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **2** | Clap / invalid global flags. |
| **10** | Network failure. |
| **11** | Auth. |
| **12** | Validation: invalid netuid, negative amount (`unstake amount` label). |
| **13** | Chain: slippage exceeded; `NotEnoughStakeToWithdraw`, `StakingRateLimitExceeded`. |
| **15** | Timeout. |
| **1** | Uncategorized. |

### Pallet ref

`SubtensorModule::remove_stake(origin, hotkey, netuid, amount_unstaked)` → AMM `swap_alpha_for_tao()`.

**Events emitted**: `StakeRemoved(coldkey, hotkey, netuid, alpha, tao)`.

---

## stake move — Move alpha between subnets (same hotkey, same coldkey)

Move alpha from one subnet to another for the **same hotkey** via `move_stake`. Internally: alpha out of source pool, TAO through coldkey, alpha into destination pool. No slippage guard.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount to move (TAO-scale; actually alpha amount). |
| `--from` | `u16` | **Yes** | Source subnet UID. |
| `--to` | `u16` | **Yes** | Destination subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake move --amount 1.0 --from 1 --to 2 --password p --yes
agcli stake move --amount 0.5 --from 1 --to 2 --hotkey-address 5FHne... --password p --yes
```

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **2** | Clap / invalid global flags. |
| **10** | Network failure. |
| **11** | Auth. |
| **12** | Validation: invalid `--from`/`--to` (SN0), negative amount (`move amount`), spending limit. |
| **13** | Chain: `NotEnoughStakeToWithdraw`, `SubnetNotExists`, etc. |
| **1** | `--from == --to` (same-subnet bail). |

### Pallet ref

`SubtensorModule::move_stake(origin, origin_hotkey, destination_hotkey, origin_netuid, dest_netuid, alpha_amount)`.

**Note (Finding #1)**: The CLI passes the same hotkey for both `origin_hotkey` and `destination_hotkey`. There is no `--dest-hotkey` flag. Cross-hotkey moves require `stake transfer-stake`.

**Events emitted**: `StakeMoved(coldkey, origin_hotkey, origin_netuid, dest_hotkey, dest_netuid, tao_equivalent)`.

---

## stake swap — Swap alpha between subnets (same hotkey)

Same-hotkey cross-subnet rebalance via `swap_stake`. Semantically similar to `stake move`; uses a different on-chain path that may differ in liquidity treatment.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount (TAO-scale). |
| `--from` | `u16` | **Yes** | Source subnet UID. |
| `--to` | `u16` | **Yes** | Destination subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake swap --amount 1.0 --from 1 --to 2 --password p --yes
```

### Exit codes

Same as `stake move`.

### Pallet ref

`SubtensorModule::swap_stake(origin, hotkey, origin_netuid, dest_netuid, alpha_amount)`.

**Events emitted**: `StakeSwapped(coldkey, hotkey, from_netuid, to_netuid, alpha, tao)` (may vary by runtime version).

---

## stake unstake-all — Unstake all alpha for one hotkey

Unwind **all alpha positions** across **all subnets** for a hotkey in a single `unstake_all` extrinsic. No `--netuid` or `--amount` flags.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake unstake-all --password p --yes
agcli stake unstake-all --hotkey-address 5FHne... --password p --yes
```

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic submitted (including "no stake" outcome). |
| **2** | Clap / invalid global flags. |
| **10** | Network failure. |
| **11** | Auth. |
| **12** | Validation: invalid `--hotkey-address`. |
| **13** | Chain: rate limits, etc. |
| **1** | Uncategorized. |

### Pallet ref

`SubtensorModule::unstake_all(origin, hotkey)` — coldkey origin; unwinds all alpha positions.

---

## stake unstake-all-alpha — Unstake all alpha across all subnets

Similar to `stake unstake-all` but uses `unstake_all_alpha` — a separate on-chain entrypoint that may differ in fee/behavior from `unstake_all`.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake unstake-all-alpha --password p --yes
agcli stake unstake-all-alpha --hotkey-address 5FHne... --password p --yes
```

### Exit codes

Same as `stake unstake-all`.

### Pallet ref

`SubtensorModule::unstake_all_alpha(origin, hotkey)`.

---

## stake add-limit — Stake with a limit price

Place a **limit order** to stake TAO into alpha when the AMM price reaches a target. Unlike `stake add`, the extrinsic is accepted immediately but executes when conditions are met.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` (TAO) | **Yes** | Amount of TAO to stake. |
| `--netuid` | `u16` | **Yes** | Target subnet UID. |
| `--price` | `f64` | **Yes** | Limit price (TAO per alpha). Must be positive and finite. |
| `--partial` | `bool` (flag) | No | Allow partial fills. Default: false. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake add-limit --amount 10.0 --netuid 1 --price 0.5 --password p --yes
agcli stake add-limit --amount 10.0 --netuid 1 --price 0.5 --partial --password p --yes
```

### Validation

1. `validate_netuid(netuid)`, `validate_amount(amount, "limit stake amount")`
2. `validate_amount(price, "limit price")` + `validate_limit_price(price, "limit price")` — rejects zero/negative, rejects values that overflow u64 at 1e9 scale
3. `check_spending_limit(netuid, amount)`
4. Amount encoded as `Balance::from_tao(amount)` → `.rao()` (u64)
5. Price encoded as `safe_rao(price)` = `Balance::from_tao(price).rao()` (u64)

### Exit codes

| Code | When |
|------|------|
| **0** | Limit order submitted. |
| **11** | Auth. |
| **12** | Invalid netuid, invalid amount, invalid price, spending limit. |
| **13** | Chain errors. |

### Pallet ref

`SubtensorModule::add_stake_limit(origin, hotkey, netuid, amount_staked, limit_price, allow_partial)`.

---

## stake remove-limit — Unstake with a limit price

Remove stake with a minimum price constraint. Executes when the AMM price exceeds the limit.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount of alpha to remove. |
| `--netuid` | `u16` | **Yes** | Source subnet UID. |
| `--price` | `f64` | **Yes** | Limit price (minimum TAO per alpha). Must be positive. |
| `--partial` | `bool` (flag) | No | Allow partial fills. Default: false. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake remove-limit --amount 5.0 --netuid 1 --price 0.8 --password p --yes
agcli stake remove-limit --amount 5.0 --netuid 1 --price 0.8 --partial --password p --yes
```

### Notes

Amount is encoded via `safe_rao(amount)` = `Balance::from_tao(amount).rao()`. This converts the f64 as if it were TAO, not raw alpha. See **Finding #2** for implications.

### Pallet ref

`SubtensorModule::remove_stake_limit(origin, hotkey, netuid, amount_unstaked, limit_price, allow_partial)`.

---

## stake remove-full-limit — Remove all stake with a price limit

Unstake the **entire position** for a hotkey on a subnet, with an optional minimum price guard. Passes `amount=0` to the chain (interpreted as "full removal").

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | **Yes** | Source subnet UID. |
| `--price` | `f64` | **Yes** | Minimum TAO per alpha. Converted via `Balance::from_tao(price).rao()`. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake remove-full-limit --netuid 1 --price 0.001 --password p --yes
```

### Pallet ref

`SubtensorModule::remove_stake_full_limit(origin, hotkey, netuid, amount, limit_price)` (raw call via `submit_raw_call`). Amount is always 0 (full removal).

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic submitted. |
| **11** | Auth. |
| **13** | Chain: price limit not reached, insufficient stake. |

---

## stake swap-limit — Swap alpha with a limit price

Swap stake cross-subnet with a minimum price guarantee. Like `stake swap` but conditional on reaching the limit price.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount of alpha to swap. |
| `--from` | `u16` | **Yes** | Source subnet UID. |
| `--to` | `u16` | **Yes** | Destination subnet UID. |
| `--price` | `f64` | **Yes** | Limit price. Must be positive. |
| `--partial` | `bool` (flag) | No | Allow partial fills. Default: false. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake swap-limit --amount 5.0 --from 1 --to 2 --price 0.5 --password p --yes
agcli stake swap-limit --amount 5.0 --from 1 --to 2 --price 0.5 --partial --password p --yes
```

### Exit codes

Same as `stake move` / `stake swap`.

### Pallet ref

`SubtensorModule::swap_stake_limit(origin, hotkey, from_netuid, to_netuid, alpha_amount, limit_price, allow_partial)`.

---

## stake recycle-alpha — Recycle alpha to TAO

Burn alpha tokens back into the AMM pool, decreasing `SubnetAlphaOut` and increasing the alpha price. The coldkey receives TAO in return.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount of alpha to recycle. |
| `--netuid` | `u16` | **Yes** | Subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake recycle-alpha --amount 100.0 --netuid 1 --password p --yes
```

### Note

Amount encoded as `safe_rao(amount)` (= `Balance::from_tao(amount).rao()`).

### Pallet ref

`SubtensorModule::recycle_alpha(origin, hotkey, amount, netuid)` — decreases `SubnetAlphaOut`, increasing price. Different from `burn_alpha` (which does not affect pool ratio).

---

## stake burn-alpha — Permanently burn alpha

Destroy alpha tokens permanently. **Does not** reduce `SubnetAlphaOut` (unlike `recycle-alpha`), so the pool ratio is unchanged. Useful for deflationary token mechanics.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | **Yes** | Amount of alpha to burn. |
| `--netuid` | `u16` | **Yes** | Subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake burn-alpha --amount 50.0 --netuid 1 --password p --yes
```

### Pallet ref

`SubtensorModule::burn_alpha(origin, hotkey, amount, netuid)` — permanently destroys alpha supply.

**Note (Finding #3)**: The extrinsic arg order in `burn_alpha_mev` is `(pair, hotkey, amount, netuid)`. The pallet dispatch may have different internal ordering; verify against on-chain metadata if submitting raw.

---

## stake claim-root — Claim root dividends for a subnet

Claim root network dividends for a specific subnet. The coldkey is the signer; no hotkey argument is accepted at the CLI level.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | **Yes** | Subnet UID to claim for (must be ≥1; SN0 rejected by `validate_netuid`). |

### Examples

```bash
agcli stake claim-root --netuid 1 --password p --yes
```

### Important

`claim-root` calls the typed `SubtensorModule::claim_root(subnets: Vec<u16>)` extrinsic signed by the coldkey. It does **not** accept a `--hotkey-address`. The single `--netuid` is wrapped in a `[netuid]` slice.

See **Finding #4** for the divergence between `claim-root` and `process-claim`.

### Pallet ref

`SubtensorModule::claim_root(origin, subnets: Vec<u16>)`.

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **11** | Auth (coldkey unlock). |
| **12** | `validate_netuid` fails (netuid 0). |
| **13** | Chain dispatch errors. |

---

## stake process-claim — Batch claim root dividends

Iterate over all subnets where the hotkey has stake and call `claim_root_dividends` (raw call) for each in parallel. Optionally filter to specific subnet IDs.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |
| `--netuids` | `String` | No | Comma-separated subnet UIDs to filter (e.g., `"1,2,3"`). Invalid IDs warn and are skipped. |

### Examples

```bash
agcli stake process-claim --password p --yes
agcli stake process-claim --netuids "1,2,3" --password p --yes
agcli stake process-claim --hotkey-address 5FHne... --netuids "5,10" --password p --yes
```

### Behavior

1. Opens wallet, queries `get_stake_for_coldkey` to enumerate subnets where the hotkey has stake.
2. Filters to `--netuids` if provided; logs warnings for non-u16 IDs.
3. Submits `claim_root_dividends(hotkey_bytes, netuid)` raw calls in parallel via `futures::future::join_all`.
4. Prints per-subnet success/failure, then a totals line.

### Exit codes

| Code | When |
|------|------|
| **0** | All claims submitted (some may fail individually; process exits 0). |
| **11** | Auth. |
| **10** | Network error querying stakes. |

**Note**: Individual per-subnet failures do not affect the overall exit code (currently always 0 on partial failure). See **Finding #5**.

### Pallet ref

`SubtensorModule::claim_root_dividends(origin, hotkey, netuid)` — different from `claim_root` used by `stake claim-root`.

---

## stake childkey-take — Set childkey take percentage

Set the fraction of hotkey emissions taken by a parent key for weight delegation.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--take` | `f64` (%) | **Yes** | Take percentage. Range: 0.0 – 18.0 (runtime enforced; `validate_take_pct` enforces locally). |
| `--netuid` | `u16` | **Yes** | Subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake childkey-take --take 10.0 --netuid 1 --password p --yes
agcli stake childkey-take --take 18.0 --netuid 1 --password p --yes   # maximum
agcli stake childkey-take --take 0.0 --netuid 1 --password p --yes    # reset
```

### Encoding

`take_u16 = (take / 100.0 * 65535.0).round().min(65535.0) as u16`

- 18% → 11796
- 100% → 65535 (hypothetical; clamped by `validate_take_pct` to 18% before conversion)
- 0.01% → 7 (rounds from 6.5535 — `.round()` avoids truncation bias)

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **11** | Auth. |
| **12** | `validate_take_pct` fails (negative, > 18%, non-finite). |
| **13** | Chain: `InvalidChildkeyTake`, `TxChildkeyTakeRateLimitExceeded`. |

### Pallet ref

`SubtensorModule::set_childkey_take(origin, hotkey, netuid, take: u16)`.

**Storage**: `ChildkeyTake(hotkey, netuid) → u16`.

---

## stake set-children — Delegate weight to child hotkeys

Set child hotkeys for a parent hotkey on a subnet. Children are **not applied immediately** — they are scheduled via `PendingChildKeys` with a cooldown period.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | **Yes** | Subnet UID. |
| `--children` | `String` | **Yes** | `"proportion:hotkey_ss58"` pairs, comma-separated. Proportions sum to ≤1.0. |
| `--hotkey-address` | `String` (SS58) | No | Parent hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake set-children --netuid 1 \
  --children "0.5:5FHne...,0.3:5GrwvaEF..." \
  --password p --yes
```

### Notes

- Maximum 5 children per hotkey per subnet (chain enforces `TooManyChildren`).
- Child hotkeys must not form cycles in the parent-child graph (`ChildParentInconsistency`).
- Bipartite separation is enforced on-chain.

### Exit codes

| Code | When |
|------|------|
| **0** | Scheduling extrinsic submitted. |
| **11** | Auth. |
| **12** | Validation: invalid `--netuid`. |
| **13** | Chain: `InvalidChild`, `DuplicateChild`, `ProportionOverflow`, `TooManyChildren`, `ChildParentInconsistency`, `NotEnoughStakeToSetChildkeys`. |

### Pallet ref

`SubtensorModule::set_children(origin, hotkey, netuid, children: Vec<(u64, AccountId)>)` → `do_schedule_children()`.

**Events**: `SetChildrenScheduled(hotkey, netuid, cooldown_block, children)`.

**Storage**: `PendingChildKeys(hotkey, netuid)` → applied at `cooldown_block`.

---

## stake set-auto — Set auto-stake hotkey for a subnet

Configure emissions for a coldkey+netuid pair to automatically compound into a hotkey.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | **Yes** | Subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey to auto-stake to. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake set-auto --netuid 1 --password p --yes
agcli stake set-auto --netuid 1 --hotkey-address 5GrwvaEF... --password p --yes
```

### Pallet ref

`SubtensorModule::set_coldkey_auto_stake_hotkey(origin, netuid, hotkey)` (raw call via `submit_raw_call`).

Arg order in raw call: `[Value::u128(netuid), Value::from_bytes(hotkey_id)]`.

**Storage**: `ColdkeyAutoStakeHotkey(coldkey, netuid) → Option<AccountId>`.

---

## stake show-auto — Show auto-stake destinations

Read-only query: list all subnets where the coldkey has an auto-stake hotkey configured.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--address` | `String` (SS58) | No | Coldkey address. Defaults to wallet coldkey. |

### Examples

```bash
agcli stake show-auto
agcli stake show-auto --address 5GrwvaEF...
```

### Output

Human table (`SN1   → 5FHne...`). No `--output json` support for this command — JSON output flag is silently treated as default (human text). See **Finding #6**.

### Pallet ref

Storage: `SubtensorModule::ColdkeyAutoStakeHotkey(coldkey, netuid) → Option<AccountId>`. Queried per-subnet in parallel via `futures::future::join_all`.

---

## stake set-claim — Set root emission handling mode

Configure how root network emissions are handled for a coldkey: swap to TAO, keep as alpha, or keep for specific subnets.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--claim-type` | `String` | **Yes** | One of `swap`, `keep`, `keep-subnets` (clap `value_parser` enforces). |
| `--subnets` | `String` | No | Comma-separated subnet UIDs (only with `--claim-type keep-subnets`). |

### Examples

```bash
agcli stake set-claim --claim-type swap --password p --yes
agcli stake set-claim --claim-type keep --password p --yes
agcli stake set-claim --claim-type keep-subnets --subnets "1,2,3" --password p --yes
```

### Notes

Invalid `--claim-type` values are rejected at parse time by clap's `value_parser`. Invalid subnet IDs in `--subnets` warn and are skipped (no exit code ≥ 1).

### Pallet ref

`SubtensorModule::set_root_claim_type(origin, claim_type: RootClaimType)` (raw call).

`RootClaimType` variants: `Swap`, `Keep`, `KeepSubnets { subnets: Vec<u16> }`.

---

## stake transfer-stake — Transfer stake to a different coldkey

Move a stake position to a different destination coldkey, optionally changing the subnet.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dest` | `String` (SS58) | **Yes** | Destination coldkey address. |
| `--amount` | `f64` (TAO) | **Yes** | Amount to transfer (`Balance::from_tao`). |
| `--from` | `u16` | **Yes** | Source subnet UID. |
| `--to` | `u16` | **Yes** | Destination subnet UID. |
| `--hotkey-address` | `String` (SS58) | No | Hotkey. Defaults to wallet hotkey. |

### Examples

```bash
agcli stake transfer-stake \
  --dest 5GrwvaEF... --amount 10.0 --from 1 --to 2 \
  --password p --yes
```

### Validation

1. `validate_netuid(from)`, `validate_netuid(to)`
2. `validate_ss58(&dest, "destination")` — errors classify as VALIDATION (12)
3. `validate_amount(amount, "transfer stake amount")`
4. `check_spending_limit(to, amount)`
5. Amount encoded as `Balance::from_tao(amount)`

### Exit codes

| Code | When |
|------|------|
| **0** | Extrinsic finalized. |
| **11** | Auth. |
| **12** | Invalid dest SS58, invalid netuid, invalid amount, spending limit. |
| **13** | Chain: `NotEnoughStakeToWithdraw`, `SubnetNotExists`, etc. |

### Pallet ref

`SubtensorModule::transfer_stake(origin, destination_coldkey, hotkey, origin_netuid, dest_netuid, alpha_amount)`.

All stake transitions funnel through `transition_stake_internal()`.

---

## stake wizard — Interactive or non-interactive staking wizard

Full staking workflow: shows top subnets by pool depth, prompts for netuid and amount (or accepts CLI flags), and calls `add_stake`. Validates stale prices on re-entry after long interactive sessions.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | No | Skip interactive subnet selection. |
| `--amount` | `f64` | No | Skip interactive amount prompt. |
| `--hotkey-address` | `String` (SS58) | No | Skip interactive hotkey selection. |

### Examples

```bash
agcli stake wizard                                   # fully interactive
agcli stake wizard --netuid 1 --amount 5.0 --yes     # non-interactive
agcli stake wizard --netuid 1 --amount 5.0 --hotkey-address 5FHne... --yes
```

### Notes

- Non-interactive mode (`--yes` + all flags) behaves identically to `stake add`.
- Price staleness guard: after interactive prompts, fetches fresh `get_all_dynamic_info()` and warns if price moved > 5% since displayed.
- Confirm dialog is skipped when `--yes` is set.
- Portfolio summary is printed after successful stake.

### Pallet ref

`SubtensorModule::add_stake` (same as `stake add`).

---

## Global flags that affect staking

| Flag | Effect |
|------|--------|
| `--mev` | Encrypt extrinsic via MEV shield (ML-KEM-768). |
| `--dry-run` | Show what would be submitted without broadcasting. |
| `--output json` | JSON output (effective for `stake list`; human-only for write commands). |
| `--output csv` | CSV output (effective for `stake list`). |
| `--batch` / `--yes` | Non-interactive; skip confirmation prompts. |
| `--password` | Coldkey decrypt password. |
| `--wallet` / `--wallet-dir` | Wallet name and directory. |
| `--endpoint` | WebSocket endpoint override. |
| `--network` | Preset network (finney, test, local). |

---

## Common errors

| Error | Code | Cause | Fix |
|-------|------|-------|-----|
| `NotEnoughBalanceToStake` | 13 | Free TAO < stake amount | Check `agcli balance` |
| `StakingRateLimitExceeded` | 13 | Too many stake ops in short window | Wait and retry |
| `NotEnoughStakeToWithdraw` | 13 | Unstake amount > staked | Check `agcli stake list` |
| `HotKeyAccountNotExists` | 13 | Hotkey not registered on chain | Register hotkey first |
| `TooManyChildren` | 13 | > 5 children set | Reduce child count |
| `AmountTooLow` | 13 | Amount below on-chain minimum | Increase amount |
| `InvalidChildkeyTake` | 13 | Take % out of range on-chain | Use 0–18% |
| `TxChildkeyTakeRateLimitExceeded` | 13 | Too many take updates | Wait for cooldown |
| `SubnetNotExists` | 13 | Subnet UID not registered | Verify subnet |
| `InvalidNetuid` / netuid 0 | 12 | SN0 reserved for root; client-side guard | Use netuid ≥ 1 |
| `Insufficient balance` | 13 | Client-side balance preflight (stake add only) | Fund coldkey |
| Slippage `exceeds maximum allowed` | 13 | AMM slippage > `--max-slippage` | Reduce trade size or use limit order |

---

## Audit findings

### Finding #1 — `stake move` hardcodes same hotkey for both source and destination

`move_stake_mev` in `src/chain/extrinsics.rs` calls:
```rust
api::tx().subtensor_module().move_stake(hk.clone(), hk, from.0, to.0, amount.rao())
```
Both `origin_hotkey` and `destination_hotkey` are the same value. The pallet's `move_stake` accepts distinct origin/destination hotkeys, but the CLI provides no `--dest-hotkey` flag. Agents cannot move stake to a different hotkey using `stake move`; they must use `stake transfer-stake` (which changes the coldkey).

### Finding #2 — `stake remove-limit` interprets amount as TAO, not alpha

The `RemoveLimit` handler uses `safe_rao(amount)` = `Balance::from_tao(amount).rao()` to encode the amount for the `remove_stake_limit` extrinsic. The clap help says "Amount of alpha" but the conversion treats it as TAO-scale (×1e9). An agent trying to remove 100 alpha tokens would pass `--amount 100` and submit 100 × 10^9 raw units — likely overshooting the available position. Consistent with `RemoveLimit` docstring but not with `RecycleAlpha` / `BurnAlpha` which have the same issue.

### Finding #3 — `stake claim-root` and `stake process-claim` call different pallet functions

`stake claim-root` uses the typed API call `claim_root(subnets: Vec<u16>)` signed by the coldkey with no hotkey argument. `stake process-claim` uses `submit_raw_call("claim_root_dividends", [hotkey_bytes, netuid])` — a different on-chain function taking a hotkey parameter. These are not the same operation. Agents seeking to claim root dividends for a specific hotkey should use `process-claim`; `claim-root` operates at coldkey+subnet granularity without targeting a specific hotkey.

### Finding #4 — `stake process-claim` exits 0 on partial per-subnet failure

The `ProcessClaim` handler collects results and prints per-subnet success/error but always returns `Ok(())`. If some subnet claims fail (chain error), the overall process exits 0. An agent relying on exit codes for scripted automation will not detect partial failures.

### Finding #5 — Write commands do not emit JSON output; `--output json` is silently ignored

All write commands (`stake add`, `stake remove`, `stake move`, etc.) print human-readable text and do not check `ctx.output` for JSON formatting. The global `--output json` flag has no effect on write-command success output. Only `stake list` and `stake remove-full-limit` (which calls `print_tx_result`) respond to the output flag. Agents expecting JSON from write commands will receive plain text.

### Finding #6 — `stake show-auto` has no JSON output mode

`show-auto` always prints human text. The `output` field from `ctx` is not read. An agent passing `--output json` gets no JSON back.

### Finding #7 — `stake swap` vs `stake move` semantic difference is undocumented

The pallet has both `swap_stake` and `move_stake` as distinct dispatchables. The CLI exposes both but the docs do not clearly differentiate their on-chain behavior. From the extrinsics: `move_stake` takes separate `origin_hotkey` and `dest_hotkey` (though both are hardcoded to the same value — see Finding #1); `swap_stake` takes a single `hotkey`. The actual liquidity mechanics differ at the pallet level. Agents choosing between them lack clear guidance.

### Finding #8 — `stake wizard` uses `dialoguer` and panics without a TTY

The `staking_wizard` function calls `dialoguer::Input::new().interact_text()` and `dialoguer::Confirm::new().interact()`. These panic if stdin is not a TTY (e.g., in a piped agent workflow). Non-interactive use requires all three flags (`--netuid`, `--amount`) and `--yes`. Missing any one triggers the interactive prompt, which panics in non-TTY environments.

---

## Source code references

- Handler: [`src/cli/stake_cmds.rs`](../src/cli/stake_cmds.rs)
- Extrinsics: [`src/chain/extrinsics.rs`](../src/chain/extrinsics.rs)
- Helpers: [`src/cli/helpers.rs`](../src/cli/helpers.rs) — `safe_rao`, `validate_take_pct`, `validate_limit_price`, `check_spending_limit`
- Pallet dispatches: `subtensor/pallets/subtensor/src/macros/dispatches.rs`
- Pallet events: `subtensor/pallets/subtensor/src/macros/events.rs`
- Pallet errors: `subtensor/pallets/subtensor/src/macros/errors.rs`
- Staking files:
  - `subtensor/pallets/subtensor/src/staking/add_stake.rs`
  - `subtensor/pallets/subtensor/src/staking/remove_stake.rs`
  - `subtensor/pallets/subtensor/src/staking/move_stake.rs`
  - `subtensor/pallets/subtensor/src/staking/set_children.rs`
  - `subtensor/pallets/subtensor/src/staking/recycle_alpha.rs`
  - `subtensor/pallets/subtensor/src/staking/claim_root.rs`

## Related commands

- `agcli balance` — free TAO balance before staking
- `agcli view portfolio` — balance + all stake positions + pricing
- `agcli subnet show --netuid N` — AMM pool depth for a subnet
- `agcli view swap-sim --netuid N --tao X` — simulate stake swap before submitting
- `agcli explain --topic stake-weight` — minimum stake for weight setting
- `agcli diff portfolio` — stake map comparison at two blocks
