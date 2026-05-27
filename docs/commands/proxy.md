# proxy — Proxy Account Management

Delegate signing authority to another account. Proxy accounts can sign transactions on behalf of the delegator, optionally filtered by operation type and/or restricted to execute only after a configurable block delay.

## Subcommands

### proxy add
Register a proxy delegate for the caller's coldkey.

```bash
agcli proxy add --delegate <SS58> [--proxy-type <type>] [--delay <blocks>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--delegate` | SS58 string | **required** | Account to grant proxy authority to |
| `--proxy-type` | string | `any` | Filter category (see Proxy Types table) |
| `--delay` | u32 | `0` | Minimum blocks before the proxy can execute (0 = immediate) |

**Pallet call:** `Proxy::add_proxy(delegate: MultiAddress, proxy_type: ProxyType, delay: BlockNumber)`
**Storage written:** `Proxy.Proxies[real_account]`
**Events emitted:** `proxy.ProxyAdded { delegator, delegatee, proxy_type, delay }`
**Exit codes:** 0 success · 12 validation error (bad SS58, unknown proxy type) · 13 chain error (e.g. `Proxy::TooMany`, `Proxy::Duplicate`) · 10 network · 11 auth (wallet locked)
**Output (text):** human confirmation + tx hash. No JSON output for write operations (see Findings).

### proxy remove
Unregister a proxy delegate.

```bash
agcli proxy remove --delegate <SS58> [--proxy-type <type>] [--delay <blocks>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--delegate` | SS58 string | **required** | Proxy account to revoke |
| `--proxy-type` | string | `any` | Must match the type used when the proxy was added |
| `--delay` | u32 | `0` | Must match the delay used when the proxy was added |

**Pallet call:** `Proxy::remove_proxy(delegate: MultiAddress, proxy_type: ProxyType, delay: BlockNumber)`
**Storage written:** `Proxy.Proxies[real_account]` (entry removed)
**Events emitted:** `proxy.ProxyRemoved { delegator, delegatee, proxy_type, delay }`
**Exit codes:** 0 success · 12 validation · 13 chain (`Proxy::NotFound`) · 10 network · 11 auth

### proxy remove-all
Revoke **all** proxy delegations for the caller's coldkey in a single call. Prompts for confirmation interactively unless `--yes` is passed.

```bash
agcli proxy remove-all
```

No additional flags. Requires wallet unlock.

**Pallet call:** `Proxy::remove_proxies()` (no arguments)
**Storage written:** `Proxy.Proxies[real_account]` (all entries cleared; deposit returned)
**Events emitted:** none (deposit return event from Balances may fire)
**Exit codes:** 0 success · 11 auth · 10 network · 13 chain

### proxy create-pure
Spawn a fresh pure (anonymous) proxy account. The new account is controlled entirely through the spawner's proxy relationship; it has no private key.

```bash
agcli proxy create-pure [--proxy-type <type>] [--delay <blocks>] [--index <n>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--proxy-type` | string | `any` | Proxy type the spawner will hold over the pure account |
| `--delay` | u32 | `0` | Block delay for the pure proxy relationship |
| `--index` | u16 | `0` | Disambiguation index — allows creating multiple pure proxies with the same type |

After creation, run `agcli proxy list` to find the new pure proxy address.

**Pallet call:** `Proxy::create_pure(proxy_type: ProxyType, delay: BlockNumber, index: u16)`
**Storage written:** `Proxy.Proxies[new_pure_account]`
**Events emitted:** `proxy.PureCreated { pure, who, proxy_type, disambiguation_index }`
**Exit codes:** 0 success · 12 validation · 13 chain · 10 network · 11 auth

### proxy kill-pure
Destroy a pure proxy account. **All funds held by the pure proxy become permanently inaccessible.** Prompts for confirmation.

```bash
agcli proxy kill-pure \
  --spawner <SS58> \
  --height <block_number> \
  --ext-index <extrinsic_index> \
  [--proxy-type <type>] \
  [--index <n>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--spawner` | SS58 string | **required** | Account that originally created the pure proxy |
| `--height` | u32 | **required** | Block height at which the pure proxy was created |
| `--ext-index` | u32 | **required** | Extrinsic index within that block |
| `--proxy-type` | string | `any` | Proxy type used at creation |
| `--index` | u16 | `0` | Disambiguation index used at creation |

**Pallet call:** `Proxy::kill_pure(spawner: MultiAddress, proxy_type: ProxyType, index: u16, height: compact<BlockNumber>, ext_index: compact<u32>)`
**Storage written:** `Proxy.Proxies[pure_account]` (removed)
**Events emitted:** none (deposit returned to spawner via Balances transfer)
**Exit codes:** 0 success · 12 validation · 13 chain (`Proxy::NotFound`) · 10 network · 11 auth

### proxy list
List all proxy delegates for an account.

```bash
agcli proxy list [--address <SS58>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--address` | SS58 string | wallet coldkey | Account whose proxies to query |

**JSON output schema** (`--output json`):
```json
[
  {
    "delegate": "5GrwvaEF5...",
    "proxy_type": "Staking",
    "delay": 0
  }
]
```

**Storage read:** `Proxy.Proxies[address]`
**Exit codes:** 0 success (empty list is also 0) · 12 validation · 10 network

### proxy announce
Announce the hash of a proxy call you intend to execute after a time delay. Must precede `proxy-announced` when a delay > 0 was set on the proxy relationship.

```bash
agcli proxy announce --real <SS58> --call-hash <0xHEX>
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--real` | SS58 string | **required** | The account being proxied (real account, not the delegate) |
| `--call-hash` | 0x-hex (32 bytes) | **required** | `blake2_256` of the SCALE-encoded call you will execute |

**Pallet call:** `Proxy::announce(real: MultiAddress, call_hash: [u8; 32])`
**Storage written:** `Proxy.Announcements[delegate_account]`
**Events emitted:** `proxy.Announced { real, proxy, call_hash }`
**Exit codes:** 0 success · 12 validation (bad SS58, bad call hash) · 13 chain (`Proxy::TooMany`) · 10 network · 11 auth

> **Note:** There is a known encoding inconsistency (see Findings): `proxy_announce` in `src/chain/extrinsics.rs` passes the `real` account as raw bytes instead of a `MultiAddress::Id` wrapper. This may cause submission failures against the live chain.

### proxy proxy-announced
Execute a previously announced proxy call after the delay period has elapsed. The announcement is consumed on success.

```bash
agcli proxy proxy-announced \
  --delegate <SS58> \
  --real <SS58> \
  --pallet <name> \
  --call <name> \
  [--proxy-type <type>] \
  [--args <JSON_array>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--delegate` | SS58 string | **required** | Account that made the announcement |
| `--real` | SS58 string | **required** | Account being proxied |
| `--pallet` | string | **required** | Pallet name of the inner call |
| `--call` | string | **required** | Dispatchable name of the inner call |
| `--proxy-type` | string | optional | Force a specific proxy type filter |
| `--args` | JSON array string | optional | Positional arguments for the inner call as JSON |

**Pallet call:** `Proxy::proxy_announced(delegate: MultiAddress, real: MultiAddress, force_proxy_type: Option<ProxyType>, call: RuntimeCall)`
**Storage written:** `Proxy.Announcements[delegate]` (entry removed on success)
**Events emitted:** `proxy.ProxyExecuted { result }` + events from the inner call
**Exit codes:** 0 success · 12 validation · 13 chain (`Proxy::Unannounced`, `Proxy::NoPermission`) · 10 network · 11 auth

> **Note:** Same encoding issue as `announce`: `delegate` and `real` are passed as raw bytes without `MultiAddress::Id` wrapping (see Findings).

### proxy reject-announcement
Reject an announced proxy call. The real account calls this to cancel a delegate's pending announcement.

```bash
agcli proxy reject-announcement --delegate <SS58> --call-hash <0xHEX>
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--delegate` | SS58 string | **required** | Account whose announcement to reject |
| `--call-hash` | 0x-hex (32 bytes) | **required** | Hash of the announced call |

**Pallet call:** `Proxy::reject_announcement(delegate: MultiAddress, call_hash: [u8; 32])`
**Storage written:** `Proxy.Announcements[delegate]` (entry removed)
**Events emitted:** none
**Exit codes:** 0 success · 12 validation · 13 chain · 10 network · 11 auth

> **Note:** Same `MultiAddress::Id` encoding issue applies (see Findings).

### proxy list-announcements
List pending proxy announcements for an account.

```bash
agcli proxy list-announcements [--address <SS58>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--address` | SS58 string | wallet coldkey | Account whose announcements to query |

**JSON output schema** (`--output json`):
```json
[
  {
    "real": "5GrwvaEF5...",
    "call_hash": "0xabcdef...",
    "height": 1000
  }
]
```

**Storage read:** `Proxy.Announcements[address]`
**Exit codes:** 0 success · 12 validation · 10 network

### proxy remove-announcement
Remove a pending announcement that the delegate previously submitted (self-cancellation).

```bash
agcli proxy remove-announcement --real <SS58> --call-hash <0xHEX>
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--real` | SS58 string | **required** | Real account the announcement was targeted at |
| `--call-hash` | 0x-hex (32 bytes) | **required** | Hash of the call to remove |

**Pallet call:** `Proxy::remove_announcement(real: MultiAddress, call_hash: [u8; 32])`
**Storage written:** `Proxy.Announcements[caller]` (entry removed; deposit returned)
**Events emitted:** none
**Exit codes:** 0 success · 12 validation · 13 chain · 10 network · 11 auth

## Proxy Types

All type strings are case-insensitive; snake_case and PascalCase are accepted.

| CLI value | On-chain variant | Description |
|-----------|-----------------|-------------|
| `any` | `Any` | All operations permitted |
| `owner` | `Owner` | Subnet owner operations |
| `staking` | `Staking` | Stake add/remove/move only |
| `non_transfer` | `NonTransfer` | Everything except balance transfers |
| `non_critical` | `NonCritical` | Non-critical operations only |
| `governance` | `Governance` | Governance voting |
| `senate` | `Senate` | Senate operations |
| `registration` | `Registration` | Neuron registration |
| `transfer` | `Transfer` | Balance transfers only |
| `small_transfer` | `SmallTransfer` | Transfers below threshold |
| `root_weights` | `RootWeights` | Root network weight setting |
| `child_keys` | `ChildKeys` | Child key operations |
| `swap_hotkey` | `SwapHotkey` | Hotkey swap operations |
| `subnet_lease_beneficiary` | `SubnetLeaseBeneficiary` | Subnet lease beneficiary operations |
| `root_claim` | `RootClaim` | Root claim operations |
| `triumvirate` | `Triumvirate` | Triumvirate governance operations |
| `non_fungible` | `NonFungible` | Non-fungible token operations |
| `sudo_unchecked_set_code` | `SudoUncheckedSetCode` | Sudo set-code (privileged) |

## Exit Codes

| Code | Constant | Meaning |
|------|----------|---------|
| 0 | — | Success |
| 1 | `GENERIC` | Unexpected / uncategorised error |
| 10 | `NETWORK` | RPC connection or WebSocket failure |
| 11 | `AUTH` | Wallet locked, wrong password, missing key |
| 12 | `VALIDATION` | Bad SS58 address, unknown proxy type, malformed call hash |
| 13 | `CHAIN` | On-chain dispatch error (see below) |
| 14 | `IO` | File I/O failure |
| 15 | `TIMEOUT` | Operation exceeded configured timeout |

### Chain errors (exit 13) for proxy operations

| Pallet error | When triggered |
|--------------|----------------|
| `Proxy::TooMany` | Exceeded max proxies or announcements per account |
| `Proxy::NotFound` | Trying to remove/execute a proxy that doesn't exist |
| `Proxy::Duplicate` | Adding a proxy that already exists (same delegate + type + delay) |
| `Proxy::NoPermission` | Proxy type does not cover the inner call |
| `Proxy::Unannounced` | `proxy_announced` called without a prior `announce` |
| `Proxy::Unproxyable` | The inner call cannot be executed via proxy |

## Time-Delayed Proxy Workflow

```
1. proxy add --delegate D --delay 100          # set up proxy with 100-block delay
2. proxy announce --real R --call-hash 0x...   # D announces intent
3. <wait 100 blocks>
4. proxy proxy-announced --delegate D --real R \
     --pallet SubtensorModule --call add_stake  # D executes after delay
```

## On-chain Pallet Reference

Pallet: `Proxy` (FRAME `pallet-proxy`)

Storage keys:
- `Proxy.Proxies` — map `AccountId → (Vec<ProxyDefinition>, BalanceOf)` — all proxy entries per real account
- `Proxy.Announcements` — map `AccountId → (Vec<Announcement>, BalanceOf)` — pending announcements per delegate

Dispatchables mapped to CLI:

| Pallet dispatchable | CLI subcommand |
|--------------------|----------------|
| `Proxy::add_proxy` | `proxy add` |
| `Proxy::remove_proxy` | `proxy remove` |
| `Proxy::remove_proxies` | `proxy remove-all` |
| `Proxy::create_pure` | `proxy create-pure` |
| `Proxy::kill_pure` | `proxy kill-pure` |
| `Proxy::announce` | `proxy announce` |
| `Proxy::proxy_announced` | `proxy proxy-announced` |
| `Proxy::reject_announcement` | `proxy reject-announcement` |
| `Proxy::remove_announcement` | `proxy remove-announcement` |
| `Proxy::proxy` | *(no CLI surface — see Findings)* |
| `Proxy::poke_deposit` | *(no CLI surface — see Findings)* |

## Related Commands

- `agcli multisig` — Multi-party approval (vs single-signer delegation)
- `agcli wallet list` — List wallet coldkeys for use as `--address`
