# swap + liquidity — Swap-pallet Operations

This page covers the `agcli liquidity ...` command group, which maps to the Subtensor `Swap` pallet AMM/liquidity dispatchables.

> Note: `agcli swap ...` (hotkey/coldkey/evm-key) targets `SubtensorModule` account-swap operations, not the `Swap` pallet.

## Liquidity subcommands (`LiquidityCommands`)

Handler: `src/cli/network_cmds.rs::handle_liquidity`  
Clap enum: `src/cli/mod.rs::LiquidityCommands`

### Common exit codes (from `src/error.rs`)

All liquidity subcommands can return:

- `0`: success
- `11` (`AUTH`): wallet unlock / missing key issues
- `12` (`VALIDATION`): bad input (invalid SS58/netuid/price/range/zero amount)
- `13` (`CHAIN`): on-chain dispatch failure
- `10` (`NETWORK`): endpoint/network errors
- `15` (`TIMEOUT`): operation timeout
- `14` (`IO`): filesystem/keyfile errors
- `1` (`GENERIC`): uncategorized failures

---

### `agcli liquidity add`

```bash
agcli liquidity add \
  --netuid <u16> \
  --price-low <f64> \
  --price-high <f64> \
  --amount <u64> \
  [--hotkey-address <SS58>]
```

**Clap flags + types**

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | subnet id |
| `--price-low` | `f64` | yes | TAO per alpha |
| `--price-high` | `f64` | yes | must be `> price-low` |
| `--amount` | `u64` | yes | liquidity in RAO, must be non-zero |
| `--hotkey-address` | `String` | no | defaults to wallet hotkey |

**Dispatch + SCALE arg mapping**

- agcli call: `Client::add_liquidity(pair, hotkey_ss58, NetUid, tick_low, tick_high, liquidity)`
- dynamic subxt dispatch: `Swap::add_liquidity`
- arg order:
  1. `hotkey: AccountId` (`Value::from_bytes(AccountId32)`)
  2. `netuid: NetUid(u16)` (`Value::u128` coercion)
  3. `tick_low: TickIndex(i32)` (`Value::i128` coercion)
  4. `tick_high: TickIndex(i32)` (`Value::i128` coercion)
  5. `liquidity: u64` (`Value::u128` coercion)

**Pallet/storage reference**

- Pallet call: `subtensor/pallets/swap/src/pallet/mod.rs::add_liquidity`
- Storage keys in scope: `Swap::Positions`, `Swap::CurrentLiquidity` (intended path)
- Current runtime behavior: call currently returns `Swap::UserLiquidityDisabled` directly and does not mutate storage.

**On-chain events emitted**

- Intended: `Swap::LiquidityAdded`
- Current runtime path: no event emitted because dispatch returns error immediately.

**Output JSON schema**

- Current behavior (including `--output json`): no JSON object; handler prints plain text status lines.
- Expected normalized tx schema used elsewhere in agcli:

```json
{
  "tx_hash": "0x..."
}
```

---

### `agcli liquidity remove`

```bash
agcli liquidity remove \
  --netuid <u16> \
  --position-id <u128> \
  [--hotkey-address <SS58>]
```

**Clap flags + types**

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | subnet id |
| `--position-id` | `u128` | yes | position identifier |
| `--hotkey-address` | `String` | no | defaults to wallet hotkey |

**Dispatch + SCALE arg mapping**

- agcli call: `Client::remove_liquidity(pair, hotkey_ss58, NetUid, position_id)`
- dynamic subxt dispatch: `Swap::remove_liquidity`
- arg order:
  1. `hotkey: AccountId` (`Value::from_bytes(AccountId32)`)
  2. `netuid: NetUid(u16)` (`Value::u128` coercion)
  3. `position_id: PositionId(u128)` (`Value::u128`)

**Pallet/storage reference**

- Pallet call: `subtensor/pallets/swap/src/pallet/mod.rs::remove_liquidity`
- Storage keys touched: `Swap::Positions`, `Swap::CurrentLiquidity`, fee globals (`Swap::FeeGlobalTao`, `Swap::FeeGlobalAlpha`) in fee-claim path

**On-chain events emitted**

- `Swap::LiquidityRemoved`

**Output JSON schema**

- Current behavior (including `--output json`): plain text only.
- Expected normalized schema:

```json
{
  "tx_hash": "0x..."
}
```

---

### `agcli liquidity modify`

```bash
agcli liquidity modify \
  --netuid <u16> \
  --position-id <u128> \
  --delta <i64> \
  [--hotkey-address <SS58>]
```

**Clap flags + types**

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | subnet id |
| `--position-id` | `u128` | yes | position identifier |
| `--delta` | `i64` | yes | positive=add, negative=remove, zero rejected |
| `--hotkey-address` | `String` | no | defaults to wallet hotkey |

**Dispatch + SCALE arg mapping**

- agcli call: `Client::modify_liquidity(pair, hotkey_ss58, NetUid, position_id, liquidity_delta)`
- dynamic subxt dispatch: `Swap::modify_position`
- arg order:
  1. `hotkey: AccountId` (`Value::from_bytes(AccountId32)`)
  2. `netuid: NetUid(u16)` (`Value::u128` coercion)
  3. `position_id: PositionId(u128)` (`Value::u128`)
  4. `liquidity_delta: i64` (`Value::i128` coercion)

**Pallet/storage reference**

- Pallet call: `subtensor/pallets/swap/src/pallet/mod.rs::modify_position`
- Storage keys touched: `Swap::Positions`, `Swap::CurrentLiquidity`, fee globals (`Swap::FeeGlobalTao`, `Swap::FeeGlobalAlpha`)

**On-chain events emitted**

- `Swap::LiquidityModified` (partial modifications)
- `Swap::LiquidityRemoved` (when position is fully removed by negative delta path)

**Output JSON schema**

- Current behavior (including `--output json`): plain text only.
- Expected normalized schema:

```json
{
  "tx_hash": "0x..."
}
```

---

### `agcli liquidity toggle`

```bash
agcli liquidity toggle --netuid <u16> [--enable]
```

`--enable` is a bool switch (`false` when omitted, `true` when present).

**Clap flags + types**

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | subnet id |
| `--enable` | `bool` | no | absent=`false`, present=`true` |

**Dispatch + SCALE arg mapping**

- agcli call: `Client::toggle_user_liquidity(pair, NetUid, enable)`
- dynamic subxt dispatch: `Swap::toggle_user_liquidity`
- arg order:
  1. `netuid: NetUid(u16)` (`Value::u128` coercion)
  2. `enable: bool` (`Value::bool`)

**Pallet/storage reference**

- Pallet call: `subtensor/pallets/swap/src/pallet/mod.rs::toggle_user_liquidity`
- Storage key in scope: `Swap::EnabledUserLiquidity(netuid)`
- Current runtime behavior: storage write/event lines are commented in `toggle_user_liquidity`; dispatch currently performs origin + subnet checks but no state mutation.

**On-chain events emitted**

- Intended by pallet docs: `Swap::UserLiquidityToggled`
- Current runtime path: none from `toggle_user_liquidity` itself (event emission is commented out).
- Related dispatchable: root-only `Swap::disable_lp` emits `UserLiquidityToggled(enable=false)` when disabling globally.

**Output JSON schema**

- Current behavior (including `--output json`): plain text only.
- Expected normalized schema:

```json
{
  "tx_hash": "0x..."
}
```

## Swap-pallet coverage snapshot (dispatchables in scope)

`subtensor/pallets/swap/src/pallet/mod.rs` exposes:

- `set_fee_rate(netuid, rate)` — no `agcli liquidity` surface
- `toggle_user_liquidity(netuid, enable)` — mapped to `agcli liquidity toggle`
- `add_liquidity(hotkey, netuid, tick_low, tick_high, liquidity)` — mapped to `agcli liquidity add`
- `remove_liquidity(hotkey, netuid, position_id)` — mapped to `agcli liquidity remove`
- `modify_position(hotkey, netuid, position_id, liquidity_delta)` — mapped to `agcli liquidity modify`
- `disable_lp()` — no `agcli liquidity` surface
