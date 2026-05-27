# balance / transfer / transfer-all / transfer-keep-alive

Move free TAO between accounts, or query an account's spendable balance. All write commands require wallet unlock (`--password` / `AGCLI_PASSWORD`) unless `--dry-run` is used.

**Discoverability:**
- `agcli balance --help`
- `agcli transfer --help`
- `agcli transfer-all --help`
- `agcli transfer-keep-alive --help`
- Tier 1 in [`docs/llm.txt`](../llm.txt)

---

## `agcli balance`

Query the **free** TAO balance of an account. Uses `System::Account` storage (not a `Balances` storage key); the `free` field of `AccountData` is returned. Reserved / frozen balance is excluded.

### Flags

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--address` | `String` (SS58) | wallet coldkey | Account to query. Defaults to the wallet coldkey. |
| `--watch [N]` | `Option<u64>` | — | Poll balance every N seconds (default 60 if flag present without value). Loops until Ctrl+C. |
| `--threshold <TAO>` | `f64` | — | Alert level: prints `*** BELOW THRESHOLD ***` when balance drops below this amount in TAO. Requires `--watch` to be useful; accepted silently without `--watch` but never triggers. |
| `--at-block <N>` | `u32` | — | Historical query at block N. Requires an archive node (`--network archive`). When `--at-block` and `--watch` are both set, `--at-block` wins and `--watch` is silently ignored. |

### JSON output

One-shot query:
```json
{"address": "5Grw...", "balance_rao": 1000000000, "balance_tao": 1.0}
```

At-block query:
```json
{"address": "5Grw...", "block": 1000, "block_hash": "0x...", "balance_rao": 1000000000, "balance_tao": 1.0}
```

Watch mode (each tick):
```json
{"address": "5Grw...", "balance_rao": 1000000000, "balance_tao": 1.0, "below_threshold": false, "timestamp": "2026-05-27T12:00:00Z"}
```

**Note:** `balance_tao` is `f64`. For amounts below ~1 nanoTAO precision degrades; `balance_rao` (u64) is the authoritative value.

### Pallet ref

Storage: `System::Account(AccountId)` → `AccountInfo { data: AccountData { free, reserved, frozen, flags } }`.  
Only `data.free` is returned. This is the transferable balance.

### Examples

```bash
agcli balance
agcli balance --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB
agcli --output json balance
agcli balance --watch            # poll every 60s
agcli balance --watch 10         # poll every 10s
agcli balance --watch 30 --threshold 5.0
agcli balance --at-block 100000  # requires archive node
```

### Exit codes

| Code | Condition |
|------|-----------|
| **0** | Balance displayed or watch loop exited cleanly (Ctrl+C). |
| **10** | Network / WebSocket failure. |
| **11** | Wallet / unlock failure (only when `--address` is omitted and wallet resolution fails). |
| **12** | Validation: bad `--address` (SS58), negative `--threshold`, non-finite `--threshold`. |
| **15** | Timeout. |
| **1** | Uncategorized. |

---

## `agcli transfer`

Send a specific TAO amount from the wallet coldkey.

**On-chain:** `Balances::transfer_allow_death(dest, value)`.  
The sender account **may be reaped** if the resulting free balance drops below the chain's existential deposit. Use `transfer-keep-alive` to guarantee the sender stays alive.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dest` | `String` (SS58) | Yes | Destination address. |
| `--amount` | `f64` (TAO) | Yes | Amount to send. Must be > 0 and finite. |

Global flags that affect this command: `--yes`, `--dry-run`, `--output`, `--password`, `--wallet`, `--wallet-dir`.

### Behavior

1. `validate_ss58(dest)` — exit 12 on bad address.
2. `validate_amount(amount)` — exit 12 on negative, zero, or non-finite value.
3. Connect to chain.
4. Open and unlock wallet coldkey.
5. **Preflight balance check**: if wallet coldkey is readable and `free < amount`, bail with "Insufficient balance: you have X but trying to transfer Y." (exit 13).
6. Interactive confirm unless `--yes` or batch mode. User declining returns exit 0 ("Cancelled.").
7. Submit `Balances::transfer_allow_death(dest, amount_rao)`.

**Note:** `amount` is in TAO, internally converted to RAO via `Balance::from_tao(amount).rao()` (u128). The conversion multiplies by 1,000,000,000.

### JSON output

```json
{"tx_hash": "0x..."}
```

Dry-run outputs the encoded call data preview to stdout and `{"tx_hash": "dry-run"}`.

### Pallet ref

Pallet: `Balances` (FRAME `pallet_balances`, dispatchable `transfer_allow_death`).  
SCALE encoding: `(MultiAddress::Id(AccountId32), Compact<u128>)`.

### Events emitted

| Event | Pallet | Condition |
|-------|--------|-----------|
| `Transfer { from, to, amount }` | `Balances` | Always on success. |
| `Endowed { account, free_balance }` | `Balances` | If destination account is new (below ED). |
| `KilledAccount { account }` | `System` | If sender balance reaches zero (account reaped). |

### Examples

```bash
agcli transfer --dest 5FHn... --amount 1.0 --password p --yes
agcli --output json transfer --dest 5FHn... --amount 0.001 --password p --yes
agcli --dry-run transfer --dest 5FHn... --amount 1.0 --password p --yes
```

### Exit codes

| Code | Condition |
|------|-----------|
| **0** | Transfer submitted; `--dry-run` preview OK; user declined ("Cancelled."). |
| **10** | Network / WebSocket failure. |
| **11** | Wallet unlock / auth failure. |
| **12** | Validation: bad `--dest` (SS58), invalid `--amount` (negative, zero, non-finite). |
| **13** | Chain error (dispatch failure, client-side insufficient balance check). |
| **15** | Finalization timeout. |
| **1** | Uncategorized. |

---

## `agcli transfer-all`

Transfer the **entire free balance** to another account (minus transaction fees).

**On-chain:** `Balances::transfer_all(dest, keep_alive)`.  
The chain computes the transferable amount; no client-side preflight balance check is performed (unlike `transfer` and `transfer-keep-alive`).

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dest` | `String` (SS58) | Yes | Destination address. |
| `--keep-alive` | `bool` (flag) | No | When set, leaves the existential deposit in the sender account. Default: false (account may be reaped). |

### Behavior

1. `validate_ss58(dest)` — exit 12 on bad address.
2. Connect to chain.
3. Open and unlock wallet coldkey.
4. Interactive confirm with "Transfer ALL funds? This will empty your account." prompt unless `--yes`.
5. Submit `Balances::transfer_all(dest, keep_alive)`.

**No preflight balance check** — the chain determines what "all" means at execution time. This is intentional: the entire free balance at execution time is transferred.

### JSON output

```json
{"tx_hash": "0x..."}
```

### Pallet ref

Pallet: `Balances`, dispatchable `transfer_all`.  
SCALE encoding: `(MultiAddress::Id(AccountId32), bool)`.

### Events emitted

| Event | Pallet | Condition |
|-------|--------|-----------|
| `Transfer { from, to, amount }` | `Balances` | Always on success. |
| `Endowed { account, free_balance }` | `Balances` | If destination is new. |
| `KilledAccount { account }` | `System` | If `keep_alive=false` and sender balance reaches zero. |

### Examples

```bash
agcli transfer-all --dest 5FHn... --password p --yes
agcli transfer-all --dest 5FHn... --keep-alive --password p --yes
agcli --output json transfer-all --dest 5FHn... --password p --yes
```

### Exit codes

| Code | Condition |
|------|-----------|
| **0** | Transfer submitted; user declined ("Cancelled."). |
| **10** | Network failure. |
| **11** | Wallet unlock failure. |
| **12** | Validation: bad `--dest` (SS58). |
| **13** | Chain dispatch error. |
| **15** | Finalization timeout. |
| **1** | Uncategorized. |

---

## `agcli transfer-keep-alive`

Send a specific TAO amount while **guaranteeing the sender account stays above the existential deposit**.

**On-chain:** `Balances::transfer_keep_alive(dest, value)`.  
The runtime will reject the extrinsic if the transfer would bring the sender below the existential deposit.

### Flags

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dest` | `String` (SS58) | Yes | Destination address. |
| `--amount` | `f64` (TAO) | Yes | Amount to send. Must be > 0 and finite. |

### Behavior

Identical to `transfer` except:
- Calls `Balances::transfer_keep_alive` instead of `Balances::transfer_allow_death`.
- The chain enforces that the sender survives (does not reap the account).

Includes client-side preflight balance check (same as `transfer`).

### JSON output

```json
{"tx_hash": "0x..."}
```

### Pallet ref

Pallet: `Balances`, dispatchable `transfer_keep_alive`.  
SCALE encoding: `(MultiAddress::Id(AccountId32), Compact<u128>)`.

### Events emitted

| Event | Pallet | Condition |
|-------|--------|-----------|
| `Transfer { from, to, amount }` | `Balances` | Always on success. |
| `Endowed { account, free_balance }` | `Balances` | If destination is new. |

`KilledAccount` is **never** emitted because the sender is kept alive.

### Examples

```bash
agcli transfer-keep-alive --dest 5FHn... --amount 1.0 --password p --yes
agcli --output json transfer-keep-alive --dest 5FHn... --amount 0.5 --password p --yes
agcli --dry-run transfer-keep-alive --dest 5FHn... --amount 1.0 --password p --yes
```

### Exit codes

| Code | Condition |
|------|-----------|
| **0** | Transfer submitted; user declined ("Cancelled."). |
| **10** | Network failure. |
| **11** | Wallet unlock failure. |
| **12** | Validation: bad `--dest`, invalid `--amount`. |
| **13** | Chain dispatch error (runtime rejects if sender would go below ED). |
| **15** | Finalization timeout. |
| **1** | Uncategorized. |

---

## Exit code summary

| Code | Name | All four commands |
|------|------|-------------------|
| **0** | Success | OK / dry-run / user cancelled |
| **1** | Generic | Uncategorized |
| **2** | Clap | Invalid global flags (clap itself) |
| **10** | Network | WebSocket / connection failure |
| **11** | Auth | Wallet unlock failure |
| **12** | Validation | Bad address, invalid amount, invalid threshold |
| **13** | Chain | Dispatch failure, insufficient balance (on-chain or client-side check) |
| **14** | IO | File not found, permission denied |
| **15** | Timeout | Finalization timeout |

Source: [`src/error.rs::exit_code`](../../src/error.rs).

---

## Implementation references

- **CLI struct:** `Commands::Balance`, `Commands::Transfer`, `Commands::TransferAll`, `Commands::TransferKeepAlive` in [`src/cli/mod.rs`](../../src/cli/mod.rs) (lines 170–215).
- **Handlers:** `Commands::Balance`, `Commands::Transfer`, `Commands::TransferAll`, `Commands::TransferKeepAlive` in [`src/cli/commands.rs`](../../src/cli/commands.rs) (lines 201–469).
- **Extrinsics:** `Client::transfer`, `Client::transfer_all`, `Client::transfer_keep_alive` in [`src/chain/extrinsics.rs`](../../src/chain/extrinsics.rs).
- **Balance query:** `Client::get_balance`, `Client::get_balance_ss58`, `Client::get_balance_at_block` in [`src/chain/mod.rs`](../../src/chain/mod.rs).
- **Validation helpers:** `validate_ss58`, `validate_amount`, `validate_threshold` in [`src/cli/helpers.rs`](../../src/cli/helpers.rs).

---

## Related commands

- `agcli balance` — check free TAO before sending
- `agcli stake transfer-stake` — move **stake** between coldkeys/subnets (`SubtensorModule::transfer_stake`, different pallet)
- `agcli view history` — recent account activity
- `agcli doctor` — RPC smoke test after install

---

## Known issues / audit findings

1. **`balance --threshold` without `--watch` is silently accepted but has no effect.** The flag is validated (exit 12 on negative/NaN) but the computed value is unused in the one-shot balance path. Agents should always combine `--threshold` with `--watch`.

2. **`balance --at-block` + `--watch` silently ignores `--watch`.** Both flags parse successfully. The handler checks `at_block` first and returns early, so `--watch` is never entered. No warning is printed. Providing both flags gives the at-block result, not a polling loop.

3. **`transfer` maps to `transfer_allow_death`, not `transfer`.** The CLI command is `agcli transfer` but the on-chain dispatchable is `Balances::transfer_allow_death`. The FRAME v2 `transfer` dispatchable was removed; this is the correct encoding for account-reaping transfers, but agents must be aware that the sender account can be reaped.

4. **`get_balance` reads only `data.free`, not total balance.** Reserved, frozen, and locked amounts are excluded. If a user has 1 TAO staked and 0.1 TAO free, `agcli balance` shows 0.1 TAO. The JSON field name `balance_rao` / `balance_tao` does not specify "free" — this is a documentation gap.

5. **`transfer-all` has no client-side preflight balance check.** Unlike `transfer` and `transfer-keep-alive`, there is no `get_balance_ss58` call before submission. The full balance at execution time is sent. This asymmetry is intentional but undocumented.

6. **`balance_tao` in JSON output is `f64`.** For amounts below ~1e-15 TAO, floating-point precision is lost. The canonical representation is `balance_rao` (u64, max ~18.4 × 10^18 RAO ≈ 18.4 × 10^9 TAO — well above supply). Agents doing programmatic comparisons should use `balance_rao`.

7. **`Balances::force_transfer` (sudo) is not exposed.** The FRAME `pallet_balances::force_transfer` dispatchable (transfers from any account without signing) has no `agcli` surface. This is reasonable (sudo-only) but means there is no agcli path for forced balance moves.
