# view — Query & Analytics Commands

Read-only commands for querying chain state, analytics, and account information. No wallet unlock required for any `view` subcommand. All subcommands connect to the chain as specified by the global `--network` / `--endpoint` flags.

## Global flags relevant to `view`

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--output` | `table\|json\|csv` | `table` | Output format. `json` emits a single JSON object to stdout. `csv` emits a header row followed by data rows. |
| `--live [SECS]` | `Option<u64>` | — | Poll in a loop. Interval defaults to 12 s if omitted. Supported by `portfolio`, `dynamic`, `metagraph`. |
| `--network` | `String` | `finney` | Chain network alias or custom endpoint tag. |
| `--endpoint` | `Option<String>` | — | Override `--network` with a raw WebSocket URL. |
| `--pretty` | `bool` | `false` | Pretty-print JSON output. |

## Exit codes

| Code | Meaning |
|------|---------|
| **0** | Success (including empty result sets). |
| **1** | Generic / uncategorized error (block not found, no address resolved, anyhow bail). |
| **10** | Network / WebSocket failure, DNS error, connection refused. |
| **12** | Validation error: invalid `--address` (bad SS58), invalid `--netuid` (root), zero/negative limit. |
| **15** | Timeout. |

Source: `src/error.rs` → `exit_code::*`.  Validation errors from `validate_ss58`, `validate_netuid`, `validate_view_limit` all produce exit **12**.

---

## view portfolio

Aggregates free TAO balance, total staked TAO equivalent, and per-subnet alpha positions for a coldkey address.

### Flags

| Flag | Type | Required | Default | Notes |
|------|------|----------|---------|-------|
| `--address` | `String` (SS58) | No | wallet coldkey from `~/.bittensor/wallets/<wallet>` | Validated with `validate_ss58(..., "portfolio --address")`; exit 12 on bad input. |
| `--at-block` | `u32` | No | — | Historical wayback; requires archive node beyond ~256-block pruning window. |

Live mode: `--live [SECS]` polls `fetch_portfolio` continuously.

### Read path

1. `pin_latest_block` → parallel `try_join!(get_balance_at_hash, get_stake_for_coldkey_pinned, get_all_dynamic_info_at_block)`.
2. `--at-block N`: `get_block_hash(N)` → parallel `try_join!(get_balance_at_block, get_stake_for_coldkey_at_block)` — no dynamic info merge on this path.
3. `--live`: `src/live.rs::live_portfolio` loops over `fetch_portfolio`.

### Pallet storage accessed (read-only)

- `System::Account` — free balance (Balances pallet).
- `SubtensorModule::Stake` — `(hotkey, netuid) → stake_rao`.
- `SubtensorModule::Alpha` — `(hotkey, coldkey, netuid) → alpha_raw`.
- `SubtensorModule::SubnetsMechanism` / `DynamicInfo` runtime API — for subnet name and price (latest path only).

### JSON schema

**Latest path:**

```json
{
  "coldkey_ss58": "5G...",
  "free_balance": ...,
  "total_staked": ...,
  "positions": [
    {
      "netuid": 1,
      "subnet_name": "...",
      "hotkey_ss58": "5H...",
      "alpha_stake": ...,
      "tao_equivalent": ...,
      "price": 0.012345
    }
  ]
}
```

**`--at-block` path:**

```json
{
  "address": "5G...",
  "block": 4000000,
  "free_balance_rao": 1000000000,
  "free_balance_tao": 1.0,
  "total_staked_rao": 2000000000,
  "total_staked_tao": 2.0,
  "stakes": [
    { "hotkey": "5H...", "netuid": 1, "stake_rao": 2000000000, "stake_tao": 2.0 }
  ]
}
```

### Examples

```bash
agcli view portfolio
agcli view portfolio --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --output json view portfolio
agcli --output csv view portfolio
agcli view portfolio --at-block 4000000
agcli --network archive view portfolio --at-block 3500000 --address 5GrwvaEF...
agcli --live 30 view portfolio
```

### Events emitted

None — read-only query.

---

## view network

Network-wide summary: current block, active subnets, total TAO issuance, total staked, emission per block, staking ratio.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--at-block` | `u32` | No | — |

### Read path

- Latest: `get_network_overview()` pins one block → parallel reads of `TotalIssuance`, `TotalStake`, `TotalNetworks`, `BlockEmission`.
- `--at-block N`: `get_block_hash(N)` → parallel `try_join!(get_total_stake_at_block, storage().balances().total_issuance())`.

### Pallet storage accessed

- `Balances::TotalIssuance` — total TAO in existence.
- `SubtensorModule::TotalStake` — sum of all stakes across subnets.
- `SubtensorModule::TotalNetworks` — registered subnet count.
- `SubtensorModule::BlockEmission` — per-block emission rate.

### JSON schema

```json
{
  "block": 5000000,
  "subnets": 64,
  "total_issuance_rao": 21000000000000000,
  "total_issuance_tao": 21000000.0,
  "total_stake_rao": 7000000000000000,
  "total_stake_tao": 7000000.0,
  "emission_per_block_rao": 1000000000,
  "staking_ratio_pct": 33.3
}
```

`--at-block` path omits `subnets` and `emission_per_block_rao`; adds `block_hash` string.

### Examples

```bash
agcli view network
agcli --output json view network
agcli view network --at-block 5000000
```

---

## view dynamic

Dynamic TAO info for every registered subnet: price (τ/α), AMM pool depths, volume, emission, tempo, symbol.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--at-block` | `u32` | No | — |

Live mode supported via `--live`.

### Read path

- Latest: `get_all_dynamic_info()` → runtime API `SubnetDynamicInfo` for each subnet.
- `--at-block N`: `get_block_hash(N)` → `get_all_dynamic_info_at_block(hash)`.
- `--live`: `src/live.rs::live_dynamic`.

### Pallet storage accessed (read-only)

- `SubtensorModule::SubnetDynamicInfo` runtime API — per-subnet: `tao_in`, `alpha_in`, `alpha_out`, `price`, `name`, `symbol`, `tempo`, `subnet_volume`, `emission`.

### JSON schema (CSV header)

CSV header: `netuid,name,symbol,tempo,price,tao_in_rao,alpha_in,alpha_out,emission,volume`

Table columns: `NetUID, Name, Symbol, Price (τ/α), TAO In, Alpha In, Alpha Out, Emission, Tempo`

> **Audit finding**: CSV has 10 fields (including `volume`) but the table renderer only shows 9 columns (no `volume` column). The `volume` field is included in CSV output but silently absent from table view.

### Examples

```bash
agcli view dynamic
agcli --output json view dynamic
agcli --output csv view dynamic
agcli view dynamic --at-block 3500000
agcli --live view dynamic
```

---

## view neuron

Full detail for a single neuron: hotkey, coldkey, stake, rank, trust, consensus, incentive, dividends, emission, validator trust/permit, pruning score, last update, axon endpoint, prometheus endpoint.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |
| `--uid` | `u16` | **Yes** | — |
| `--at-block` | `u32` | No | — |

### Read path

- Latest: `get_neuron(NetUid(netuid), uid)`.
- `--at-block N`: `get_block_hash(N)` → `get_neuron_at_block(netuid, uid, hash)`.

### Pallet storage accessed (read-only)

- `SubtensorModule::NeuronInfo` / `NeuronsLite` runtime API — full neuron record including axon and prometheus info.

### JSON schema

> **Audit finding**: `view neuron` does **not** respect `--output json`. The handler uses `println!` directly and ignores the `OutputFormat`. Running `agcli --output json view neuron --netuid 1 --uid 0` produces human-readable text, not JSON.

Output is human-readable table only (no JSON/CSV path implemented):

```
Neuron UID 0 on SN1
  Hotkey:          5H...
  Coldkey:         5G...
  Active:          true
  Stake:           1.2345 τ
  Rank:            0.123456
  Trust:           0.999000
  Consensus:       0.998000
  Incentive:       0.001000
  Dividends:       0.012000
  Emission:        0.0012 τ
  Val. Trust:      0.998000
  Val. Permit:     true
  Pruning Score:   0.000000
  Last Update:     5000000
  Axon:            192.168.1.1:8091 (v1, proto 4)
  Prometheus:      192.168.1.1:9090 (v1)
```

### Examples

```bash
agcli view neuron --netuid 1 --uid 0
agcli view neuron --netuid 18 --uid 42 --at-block 4000000
```

---

## view validators

Top validators by stake: without `--netuid` shows cross-subnet delegates ranked by total stake; with `--netuid` shows per-subnet neurons with validator permits.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | No | — (all subnets) |
| `--limit` | `usize` | No | `50` |
| `--at-block` | `u32` | No | — |

### Read path

- Without `--netuid`: `get_delegates()` → sort by `total_stake` descending → truncate to `limit`.
- With `--netuid N`: `get_neurons_lite(N)` → filter `validator_permit == true` → sort by stake → truncate to `limit`.
- `--at-block` variants use `*_at_block` equivalents.

### Pallet storage accessed (read-only)

- `SubtensorModule::Delegates` — take, nominators, registrations per hotkey.
- `SubtensorModule::NeuronsLite` runtime API — lightweight neuron records for per-subnet path.

### JSON schema (CSV header without `--netuid`)

CSV: `rank,hotkey,owner,take_pct,total_stake_rao,nominators,registrations`

CSV with `--netuid`: `uid,hotkey,coldkey,stake_rao,trust,vtrust,dividends,emission`

### Examples

```bash
agcli view validators
agcli view validators --limit 100
agcli view validators --netuid 1
agcli view validators --netuid 1 --limit 20 --at-block 4000000
agcli --output json view validators --netuid 1
```

---

## view history

Recent extrinsics for an account via the Subscan API (`https://bittensor.api.subscan.io/api/v2/scan/extrinsics`).

> **Audit finding**: This command depends on an external third-party API (Subscan), not the chain RPC directly. There is no API key support and no rate-limit handling — when the API returns an error or empty result, the CLI prints a note but exits 0. If Subscan is unavailable, the command silently produces no output.

> **Audit finding**: The CSV header has 6 fields (`block,hash,module,call,success,timestamp`) but the table header has 5 columns (`Block, Module, Call, Success, Hash`) — `timestamp` is present in CSV output but missing from the table, and `hash` appears second in CSV but last in table. The columns are ordered differently between output modes.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--address` | `String` (SS58) | No | wallet coldkey |
| `--limit` | `usize` | No | `20` (max 100; Subscan caps at 100) |

### JSON schema (CSV header)

CSV header: `block,hash,module,call,success,timestamp`

Table columns: `Block, Module, Call, Success, Hash`

### Examples

```bash
agcli view history
agcli view history --address 5GrwvaEF...
agcli view history --limit 50
agcli --output json view history
agcli --output csv view history
```

---

## view account

Comprehensive account explorer: free balance, total staked, identity info, delegate status (take, nominators, registered subnets), stake positions with subnet names and prices.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--address` | `String` (SS58) | No | wallet coldkey |
| `--at-block` | `u32` | No | — |

### Read path

- Latest: `pin_latest_block` → parallel `try_join!(get_balance_at_hash, get_stake_for_coldkey_pinned, get_identity_pinned, get_all_dynamic_info, get_delegate_pinned)`.
- `--at-block N`: `get_block_hash(N)` → parallel `try_join!(get_balance_at_block, get_stake_for_coldkey_at_block, get_identity_at_block)`.

### Pallet storage accessed (read-only)

- `System::Account` (Balances).
- `SubtensorModule::Stake`, `Alpha`.
- `Registry::IdentityOf` — on-chain identity (name, url, discord, github).
- `SubtensorModule::Delegates` — delegate take and nominators.
- `SubtensorModule::DynamicInfo` — subnet names and prices.

### JSON schema

```json
{
  "address": "5G...",
  "balance_rao": 1000000000,
  "balance_tao": 1.0,
  "total_staked_rao": 2000000000,
  "stakes": [
    {
      "netuid": 1,
      "hotkey": "5H...",
      "stake_rao": 2000000000,
      "alpha_raw": 12345,
      "subnet_name": "tau1",
      "price": 0.012345
    }
  ],
  "identity": { "name": "Alice", "url": "https://...", "discord": "alice#0001" },
  "is_delegate": false
}
```

`--at-block` path: includes `block`, `block_hash`; `identity` has `name/url/discord` only; omits `is_delegate` and `subnet_name`/`price`.

### Examples

```bash
agcli view account
agcli view account --address 5GrwvaEF...
agcli view account --at-block 4000000
agcli --output json view account
```

---

## view subnet-analytics

Detailed analytics for a specific subnet: neuron counts, validator/miner split, stake totals, emission rates, average trust/incentive/dividends, top miners by incentive, top validators by dividends.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |

### Read path

`pin_latest_block` → parallel `try_join!(get_subnet_info_pinned, get_dynamic_info_at_block, get_neurons_lite, get_subnet_hyperparams_pinned, get_subnet_identity_pinned)`.

### Pallet storage accessed (read-only)

- `SubtensorModule::SubnetInfo` runtime API.
- `SubtensorModule::DynamicInfo` runtime API.
- `SubtensorModule::NeuronsLite` runtime API.
- `SubtensorModule::SubnetHyperparams`.
- `SubtensorModule::SubnetIdentity`.

### JSON schema

```json
{
  "netuid": 1,
  "name": "tau1",
  "total_neurons": 256,
  "validators": 64,
  "miners": 192,
  "unique_owners": 180,
  "total_stake_tao": 50000.0,
  "total_emission": 7200.0,
  "avg_trust": 0.98,
  "avg_miner_incentive": 0.004,
  "avg_validator_dividends": 0.01,
  "price": 0.012345,
  "tao_in": 1000.0
}
```

### Examples

```bash
agcli view subnet-analytics --netuid 1
agcli --output json view subnet-analytics --netuid 18
```

---

## view staking-analytics

APY estimates and yield projections for all staking positions of a coldkey: per-position daily and annual emission estimates, weighted portfolio APY.

> **Audit finding**: APY estimates treat `DynamicInfo::total_emission()` as per-tempo (not per-block) and divide by tempo to get per-block rate, then multiply by 7200 blocks/day. This model is documented inline but not surfaced in the help text. Users unaware of this will misinterpret the formula.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--address` | `String` (SS58) | No | wallet coldkey |

### Read path

Parallel `try_join!(get_stake_for_coldkey, get_all_dynamic_info, get_block_emission)`.

### JSON schema

```json
{
  "address": "5G...",
  "total_staked_tao": 1000.0,
  "total_daily_emission_tao": 0.12345,
  "weighted_apy_pct": 45.0,
  "block_emission_rao": 1000000000,
  "positions": [
    {
      "netuid": 1,
      "name": "tau1",
      "staked_tao": 500.0,
      "price": 0.012345,
      "estimated_daily_tao": 0.06,
      "estimated_apy_pct": 43.8
    }
  ]
}
```

### Examples

```bash
agcli view staking-analytics
agcli view staking-analytics --address 5GrwvaEF...
agcli --output json view staking-analytics
```

---

## view swap-sim

Simulate a TAO → Alpha or Alpha → TAO swap through the subnet AMM without executing. Shows expected output amount, fees, effective price, and slippage.

> **Audit finding**: When neither `--tao` nor `--alpha` is provided, the handler calls `anyhow::bail!` which produces exit code **1** (GENERIC) rather than **12** (VALIDATION). This is inconsistent with other missing-argument errors in the view group.

### Flags

| Flag | Type | Required | Default | Notes |
|------|------|----------|---------|-------|
| `--netuid` | `u16` | **Yes** | — | Subnet to simulate swap on. |
| `--tao` | `f64` | Conditional | — | TAO amount to swap to alpha. Must be positive. |
| `--alpha` | `f64` | Conditional | — | Alpha amount to swap to TAO. Must be positive. |

At least one of `--tao` or `--alpha` must be provided (checked at runtime, not clap time).

### Read path

1. `current_alpha_price(netuid)` — fetches current price.
2. `sim_swap_tao_for_alpha(netuid, rao_amount)` or `sim_swap_alpha_for_tao(netuid, alpha_rao)` — runtime API calls.

### Pallet storage accessed (read-only)

- `SubtensorModule::SubnetDynamicInfo` — pool state for swap simulation.
- Runtime API: `simulate_swap_tao_for_alpha` / `simulate_swap_alpha_for_tao`.

### JSON schema

```json
{
  "direction": "tao_to_alpha",
  "netuid": 1,
  "amount_in": 10.0,
  "amount_out": 800.0,
  "tao_fee": 0.001,
  "alpha_fee": 0.0,
  "effective_price": 0.0125,
  "current_price": 0.0124
}
```

### Examples

```bash
agcli view swap-sim --netuid 1 --tao 10.0
agcli view swap-sim --netuid 1 --alpha 1000.0
agcli --output json view swap-sim --netuid 1 --tao 10.0
```

---

## view nominations

Show all delegators and their stake for a specific validator hotkey.

> **Audit finding**: Output is always human-readable text; `--output json` serializes via `print_json_ser(&delegates)` which outputs the raw `Vec<DelegateInfo>` struct, and `--output csv` is **silently ignored** (the handler short-circuits after the JSON branch with no CSV path).

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--hotkey-address` | `String` (SS58) | **Yes** | — |

### Read path

`client.get_delegated(hotkey)` — fetches all delegate records where this hotkey appears.

### Pallet storage accessed (read-only)

- `SubtensorModule::Delegates` — per-hotkey: take, owner, nominators (coldkey → stake), registrations, validator permits.

### JSON schema

Serialized `Vec<DelegateInfo>` (raw struct serialization via `serde`):

```json
[
  {
    "hotkey": "5H...",
    "owner": "5G...",
    "take": 0.18,
    "total_stake": ...,
    "nominators": [["5G...", ...]],
    "registrations": [1, 3],
    "validator_permits": [1]
  }
]
```

### Examples

```bash
agcli view nominations --hotkey-address 5H...
agcli --output json view nominations --hotkey-address 5H...
```

---

## view metagraph

Full metagraph dump for a subnet (sorted by emission descending), or diff mode comparing current state against a historical block.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |
| `--since-block` | `u32` | No | — | When set, shows only neurons that changed since this block. |
| `--limit` | `usize` | No | — (all neurons) | Limit rows shown. |

Live mode: `--live [SECS]` polls `live_metagraph`.

### Read path

1. `require_subnet_exists(netuid)` — bails with "Subnet N not found" (exit 12) if missing.
2. `get_neurons_lite(netuid)` — lightweight neuron array.
3. **Diff mode** (`--since-block`): parallel `try_join!(get_block_hash(block_num), get_block_number)` → `get_neurons_lite_at_block(netuid, hash)` → diff computation.

### Pallet storage accessed (read-only)

- `SubtensorModule::NeuronsLite` runtime API — returns all neuron records for a subnet.

### JSON schema

**Full dump:**

```json
{
  "netuid": 1,
  "block": 5000000,
  "n": 256,
  "neurons": [
    {
      "uid": 0, "hotkey": "5H...", "coldkey": "5G...",
      "stake_tao": 1.234, "emission": 0.001234,
      "incentive": 0.0012, "consensus": 0.998,
      "trust": 0.999, "dividends": 0.012,
      "validator_trust": 0.998, "validator_permit": true,
      "last_update": 5000000, "active": true
    }
  ]
}
```

**Diff mode:**

```json
{
  "netuid": 1,
  "since_block": 4990000,
  "current_block": 5000000,
  "total_neurons": 256,
  "changed": 12,
  "diffs": [
    {
      "uid": 5, "hotkey": "5H...", "change": "changed",
      "stake_diff": 0.5, "emission_diff": 0.0001,
      "incentive_diff": 0.0002, "trust_diff": -0.0001
    }
  ]
}
```

### Examples

```bash
agcli view metagraph --netuid 1
agcli view metagraph --netuid 1 --limit 20
agcli view metagraph --netuid 1 --since-block 4990000
agcli --output json view metagraph --netuid 1
agcli --live view metagraph --netuid 1
```

---

## view axon

Look up the axon (serving endpoint) and prometheus info for a specific neuron, identified by UID or hotkey.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |
| `--uid` | `u16` | Conditional | — |
| `--hotkey-address` | `String` (SS58) | Conditional | — |

At least one of `--uid` or `--hotkey-address` must be provided (checked at runtime via `anyhow::bail!`; exit 1, not exit 12 — see audit finding below).

> **Audit finding**: When neither `--uid` nor `--hotkey-address` is provided, the handler calls `anyhow::bail!("Provide either --uid or --hotkey-address")` which produces exit code **1** (GENERIC). This is inconsistent with the VALIDATION exit code (12) used by other argument-missing errors.

### Read path

1. If `--uid N` given: use directly.
2. If `--hotkey-address HK`: `get_neurons_lite(netuid)` → find UID where `hotkey == hk`; bail (exit 1) if not found.
3. `get_neuron(netuid, uid)` — full neuron info for axon/prometheus fields.

### Pallet storage accessed (read-only)

- `SubtensorModule::NeuronsLite` runtime API (hotkey lookup path).
- `SubtensorModule::NeuronInfo` runtime API — full record including `AxonInfo` and `PrometheusInfo`.

### IP formatting note

Axon IPs are stored as `u128` strings on-chain. The handler converts IPv4 values (`ip_type == 4`, value fits in `u32`) to dotted-quad via `format_ip` / `format_prometheus_ip`. IPv6 is returned as-is.

### JSON schema

```json
{
  "netuid": 1,
  "uid": 7,
  "hotkey": "5H...",
  "axon": {
    "ip": "192.168.1.1",
    "port": 8091,
    "protocol": 4,
    "version": 1
  },
  "prometheus": {
    "ip": "192.168.1.1",
    "port": 9090,
    "version": 1
  }
}
```

`axon` and `prometheus` are `null` if the neuron is not serving.

### Examples

```bash
agcli view axon --netuid 1 --uid 7
agcli view axon --netuid 1 --hotkey-address 5H...
agcli --output json view axon --netuid 1 --uid 0
```

---

## view health

Subnet health dashboard: total neuron count, active ratio, validator permit count, dynamic info (tempo, price). Optionally TCP-probes each axon endpoint to check reachability.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |
| `--tcp-check` | `bool` (flag) | No | `false` |
| `--probe-timeout-ms` | `u64` | No | `2000` |

### Read path

Parallel `try_join!(get_neurons_lite(netuid), get_dynamic_info(netuid))` → `get_block_number`.

TCP probe path: fetches full `get_neuron(netuid, uid)` for each neuron (up to 256), then attempts `tokio::net::TcpStream::connect` with `probe_timeout_ms` timeout.

### Pallet storage accessed (read-only)

- `SubtensorModule::NeuronsLite` runtime API.
- `SubtensorModule::DynamicInfo` runtime API.
- `SubtensorModule::NeuronInfo` runtime API (TCP probe path only).

### JSON schema

```json
{
  "netuid": 1,
  "block": 5000000,
  "total_neurons": 256,
  "active": 240,
  "active_pct": 93.75,
  "validators": 64,
  "name": "tau1",
  "tempo": 360,
  "price": 0.012345,
  "tcp_probed": 256,
  "reachable": 180,
  "unreachable": 76,
  "probes": [
    { "uid": 0, "hotkey": "5H...", "endpoint": "192.168.1.1:8091", "reachable": true }
  ]
}
```

`tcp_probed`, `reachable`, `unreachable`, `probes` only present when `--tcp-check` is used.

### Examples

```bash
agcli view health --netuid 1
agcli view health --netuid 1 --tcp-check
agcli view health --netuid 1 --tcp-check --probe-timeout-ms 5000
agcli --output json view health --netuid 18
```

---

## view emissions

Per-UID emission breakdown for a subnet, sorted by emission descending.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--netuid` | `u16` | **Yes** | — |
| `--limit` | `usize` | No | — (all UIDs) |

### Read path

Parallel `try_join!(get_neurons_lite(netuid), get_dynamic_info(netuid))`.

### Pallet storage accessed (read-only)

- `SubtensorModule::NeuronsLite` runtime API.
- `SubtensorModule::DynamicInfo` runtime API (for subnet name).

### JSON schema

```json
{
  "netuid": 1,
  "total_emission": 7200.0,
  "name": "tau1",
  "neurons": [
    {
      "uid": 0,
      "hotkey": "5H...",
      "emission": 100.0,
      "emission_pct": 1.39,
      "incentive": 0.001,
      "dividends": 0.012,
      "validator_permit": true
    }
  ]
}
```

### Examples

```bash
agcli view emissions --netuid 1
agcli view emissions --netuid 1 --limit 20
agcli --output json view emissions --netuid 18
agcli --output csv view emissions --netuid 1
```

---

## agcli audit (top-level)

Full security audit of a coldkey account: proxies, delegates, stake exposure, childkey delegations, pending childkey changes, liquidity risk, identity status, scheduled coldkey swaps. Implemented in `src/cli/view_cmds.rs::handle_audit`.

> **Note**: `audit` is a top-level command (`agcli audit`), not a `view` subcommand.

### Flags

| Flag | Type | Required | Default |
|------|------|----------|---------|
| `--address` | `String` (SS58) | No | wallet coldkey |

### Read path

`pin_latest_block` → parallel `try_join!(get_balance_at_hash, get_stake_for_coldkey_pinned, get_identity_pinned, list_proxies_pinned, get_delegate_pinned, get_all_dynamic_info, get_coldkey_swap_scheduled_pinned)` → per-stake childkey queries (parallel).

### Pallet storage accessed (read-only)

- `System::Account`.
- `SubtensorModule::Stake`, `Alpha`.
- `Registry::IdentityOf`.
- `Proxy::Proxies`.
- `SubtensorModule::Delegates`.
- `SubtensorModule::DynamicInfo`.
- `SubtensorModule::ColdkeySwapScheduled`.
- `SubtensorModule::ChildKeys`, `PendingChildKeys`.

### Finding severity levels

| Severity | Meaning |
|----------|---------|
| `high` | Immediate risk (Any-type proxy, scheduled coldkey swap). |
| `medium` | Elevated risk (stake concentration >80%, low-liquidity exposure, pending childkey changes). |
| `low` | Minor concern (very low free balance). |
| `info` | Informational (childkey delegation, high delegate take, no identity). |

### Examples

```bash
agcli audit
agcli audit --address 5GrwvaEF...
agcli --output json audit
```

---

## Live Mode

Supported by `portfolio`, `dynamic`, and `metagraph`:

```bash
agcli --live view dynamic            # poll every 12 s (default)
agcli --live 30 view portfolio       # poll every 30 s
agcli --live view metagraph --netuid 1
```

## Historical Queries

Most `view` subcommands support `--at-block N`. Requires an archive node for blocks more than ~256 blocks in the past:

```bash
agcli --network archive view portfolio --at-block 3000000
agcli --network archive view dynamic --at-block 3500000
agcli --network archive view network --at-block 5000000
```

---

## Source

- Handler: [`src/cli/view_cmds.rs`](../../src/cli/view_cmds.rs)
- enum: `ViewCommands` in [`src/cli/mod.rs`](../../src/cli/mod.rs) at line 1350
- Tests: [`tests/audit_view.rs`](../../tests/audit_view.rs)

## Related Commands

- `agcli balance` — free TAO balance only
- `agcli stake list` — stake positions without dynamic price/name merge
- `agcli diff portfolio` — compare portfolio between two blocks
- `agcli diff subnet` — compare subnet state between two blocks
- `agcli explain --topic amm` — Dynamic TAO AMM mechanics
- `agcli explain --topic commit-reveal` — commit-reveal weight setting
