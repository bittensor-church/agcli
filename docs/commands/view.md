# view — Query & Analytics Commands

Read-only commands for querying chain state, analytics, and account information. No wallet unlock required for most commands.

## view portfolio — Full coldkey portfolio (read-only)

Aggregates **free TAO**, **total staked** (TAO equivalent), and **per-subnet positions** (alpha, hotkey, subnet name, price) for a coldkey. Uses the default wallet coldkey or **`--address`**. Supports **`--at-block`**, **`--live`**, and global **`--output json|csv`**. No hot/cold unlock — only reads the default coldkey from disk when **`--address`** is omitted.

**Discoverability:** `agcli view portfolio --help`; Tier 1 in [`docs/llm.txt`](../llm.txt); `agcli explain --topic ow` (Phase 6) references the e2e log name; View row in `llm.txt` → this file.

### After `cargo install`

```bash
cargo install --git https://github.com/unarbos/agcli
agcli view portfolio
agcli view portfolio --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --output json view portfolio --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --output csv view portfolio
agcli view portfolio --at-block 100
agcli --network archive view portfolio --at-block 3500000 --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli --live 30 view portfolio
```

### Read path (RPC / runtime API)

Order matches [`ViewCommands::Portfolio`](https://github.com/unarbos/agcli/blob/main/src/cli/view_cmds.rs) in `src/cli/view_cmds.rs` (`handle_view`, `Portfolio` branch):

1. **`connect`** (global network / endpoint — same as other view commands).
2. **`resolve_and_validate_coldkey_address`** — if **`--address`** is set, **`validate_ss58(..., "portfolio --address")`**; else coldkey from wallet (`src/cli/helpers.rs`). Unresolved / empty coldkey bails before RPC (same pattern as `agcli balance` / `agcli stake list`).
3. **If `--at-block`:** **`get_block_hash(block)`** → **`try_join!(get_balance_at_block(addr, hash), get_stake_for_coldkey_at_block(addr, hash))`** — compact JSON (see below); no dynamic-info merge on this path.
4. **Else if `--live`:** polling loop calling **`fetch_portfolio`** each interval (`src/queries/portfolio.rs`, `src/live.rs`).
5. **Else (latest):** **`handle_portfolio`** → **`fetch_portfolio`**: **`pin_latest_block`** → **`try_join!(get_balance_at_hash, get_stake_for_coldkey_at_block, get_all_dynamic_info_at_block)`** — dynamic info fills subnet name and price; if that RPC fails, the code logs a warning and treats dynamic data as empty (`src/queries/portfolio.rs`).

### JSON shapes

**Latest** (`--output json`) — serialized [`Portfolio`](https://github.com/unarbos/agcli/blob/main/src/queries/portfolio.rs): `coldkey_ss58`, `free_balance`, `total_staked`, `positions` (`netuid`, `subnet_name`, `hotkey_ss58`, `alpha_stake`, `tao_equivalent`, `price`). Field names/types follow `serde` on `Balance` and the struct definitions in the crate.

**`--at-block`** — object built in `handle_portfolio_at_block`: `address`, `block`, `free_balance_rao` / `free_balance_tao`, `total_staked_rao` / `total_staked_tao`, `stakes` (`hotkey`, `netuid`, `stake_rao`, `stake_tao`).

### Exit codes

| Code | When |
|------|------|
| **0** | Successful query (including **empty** positions / stakes). |
| **2** | Clap / invalid global flags. |
| **10** | Network / WebSocket failure on `connect` or hard RPC errors. |
| **12** | Validation: invalid **`--address`** (SS58) per [`classify`](https://github.com/unarbos/agcli/blob/main/src/error.rs). |
| **15** | Timeout when applicable. |
| **1** | Generic: e.g. **`Block N not found`** for **`--at-block`**, could not resolve coldkey when no **`--address`**, pruned state at a historical height, or uncategorized errors. |

Messages for bad **`--address`** include **`portfolio --address`** — [`hint`](https://github.com/unarbos/agcli/blob/main/src/error.rs) points at **`docs/commands/view.md`**.

### E2E

Log lines **`view_portfolio_preflight`** in Phase 20 [`test_view_portfolio_preflight`](https://github.com/unarbos/agcli/blob/main/tests/e2e_test.rs): **`validate_ss58`** with label **`portfolio --address`**, **`pin_latest_block`** → **`try_join!(get_balance_at_hash, get_stake_for_coldkey_at_block, get_all_dynamic_info_at_block)`**, then head **`get_block_hash`** + **`try_join!(get_balance_at_block, get_stake_for_coldkey_at_block)`** — mirrors latest **`fetch_portfolio`** and **`--at-block`**. Broader view RPC checks remain in Phase 21 **`test_view_queries`**.

### Related

- `agcli balance` — free TAO only
- `agcli stake list` — stakes only (no dynamic price/name merge)
- `agcli diff portfolio` — compare two block heights

---

## Subcommands

### view portfolio

See **[view portfolio](#view-portfolio--full-coldkey-portfolio-read-only)** (install examples, read path, JSON, **`--at-block`**, **`--live`**, exit **12** for bad **`--address`**, e2e).

**Read-only:** `System::Account`, stake storage, dynamic info (latest path). No extrinsic.

### view network
Network-wide overview: total issuance, total stake, emission rate, block height, active subnets.

```bash
agcli view network [--at-block N]
```

### view dynamic
Dynamic TAO info for all subnets: prices, AMM pool depths, volumes, emissions.

```bash
agcli view dynamic [--at-block N]
# CSV: netuid,name,price,tao_in,alpha_in,alpha_out,emission
```

### view neuron
Full detail for a single neuron: stake, weights, bonds, axon, rank, trust, consensus.

```bash
agcli view neuron --netuid 1 --uid 0 [--at-block N]
```

### view validators
Top validators by stake across subnets.

```bash
agcli view validators [--netuid N] [--limit 50] [--at-block N]
```

### view history
Recent extrinsics for an account (via Subscan API).

```bash
agcli view history [--address SS58] [--limit 20]
```

Fetches from `https://bittensor.api.subscan.io/api/v2/scan/extrinsics`.

### view account
Comprehensive account explorer: balance, stakes, identities, proxy delegations, recent history.

```bash
agcli view account [--address SS58] [--at-block N]
```

### view subnet-analytics
Detailed subnet analysis: neuron distribution, weight patterns, emission concentration.

```bash
agcli view subnet-analytics --netuid 1
```

### view staking-analytics
APY estimates and yield analysis for staked positions.

```bash
agcli view staking-analytics [--address SS58]
```

### view swap-sim
Simulate an AMM swap without executing. Shows expected output, fees, and price impact.

```bash
agcli view swap-sim --netuid 1 --tao 10.0
agcli view swap-sim --netuid 1 --alpha 1000.0
# JSON: {"alpha_out", "tao_fee", "alpha_fee", "price_impact_pct"}
```

Use this before `stake add` to estimate slippage.

### view nominations
Show who delegates to a specific validator.

```bash
agcli view nominations --hotkey-address SS58
```

## Live Mode
Any view command can be polled continuously with `--live`:

```bash
agcli --live view dynamic          # poll with delta tracking
agcli --live 30 view portfolio     # poll every 30s
agcli --live subnet metagraph --netuid 1  # track neuron changes
```

## Historical Queries
Most view commands support `--at-block N`:

```bash
agcli --network archive view portfolio --at-block 3000000
agcli --network archive view dynamic --at-block 3500000
```

Requires an archive node for blocks beyond ~256 block pruning window.

## Source Code
**agcli handler**: [`src/cli/view_cmds.rs`](https://github.com/unarbos/agcli/blob/main/src/cli/view_cmds.rs) — `handle_view()` at L9, subcommands: Portfolio L17, Network L27, Dynamic L28, Neuron L37, Validators L42, History L47, Account L51, SubnetAnalytics L55, StakingAnalytics L58, SwapSim L62, Nominations L65. Audit: `handle_audit()` at L1287.

**On-chain**: read-only queries against `System::Account`, `SubtensorModule` storage maps (Stake, Alpha, DynamicInfo, SubnetHyperparams, Metagraph, etc.). History uses [Subscan API](https://bittensor.api.subscan.io/api/v2/scan/extrinsics).

## Related Commands
- `agcli balance` — Simple balance check
- `agcli stake list` — Stake positions only
- `agcli subnet metagraph` — Full metagraph data
- `agcli explain --topic amm` — How Dynamic TAO AMM works

---

## audit (top-level command) — security audit of a coldkey account

`audit` is a top-level command (`Commands::Audit`) dispatched from `src/cli/commands.rs` into `view_cmds::handle_audit`.
It is not a `view` subcommand, but the handler and output format live in `src/cli/view_cmds.rs`.

### Clap surface (flags + types)

```bash
agcli audit [--address <SS58>]
```

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--address` | `String` (SS58) | no | If omitted, resolves wallet coldkey via `resolve_and_validate_coldkey_address`. |
| `--output` (global) | `table \| json \| csv` (`OutputFormat`) | no | JSON output is documented below. |
| `--network`, `--endpoint`, `--timeout`, etc. (global) | global CLI flags | no | Standard global connection/runtime controls apply. |

### Handler and chain read path

`handle_audit(client, address, output)` performs read-only state inspection and local risk scoring:

1. `pin_latest_block()` (pins one block hash for consistency).
2. In parallel at pinned hash:
   - `get_balance_at_hash(address, pin)` → `System::Account`.
   - `get_stake_for_coldkey_pinned(address, pin)` → `StakeInfoRuntimeApi::get_stake_info_for_coldkey`.
   - `get_identity_pinned(address, pin)` → `Registry::IdentityOf`.
   - `list_proxies_pinned(address, pin)` → `Proxy::Proxies`.
   - `get_delegate_pinned(address, pin)` → `DelegateInfoRuntimeApi::get_delegate`.
   - `get_coldkey_swap_scheduled_pinned(address, pin)` → `SubtensorModule::ColdkeySwapAnnouncements`.
3. Non-fatal supplemental latest-state query:
   - `get_all_dynamic_info()` → `SubnetInfoRuntimeApi::get_all_dynamic_info`.
4. For each staked hotkey/netuid pair at pinned hash:
   - `get_child_keys_pinned(hotkey, netuid, pin)` → `SubtensorModule::ChildKeys`.
   - `get_pending_child_keys_pinned(hotkey, netuid, pin)` → `SubtensorModule::PendingChildKeys`.

### Subxt pallet/runtime API mapping and SCALE key/arg encoding

| agcli call | Subxt target | Chain reference | SCALE key/arg shape |
|---|---|---|---|
| `get_balance_at_hash` | `api::storage().system().account(&account_id)` | FRAME `System::Account` | key: `AccountId32` decoded from SS58 |
| `get_stake_for_coldkey_pinned` | `api::apis().stake_info_runtime_api().get_stake_info_for_coldkey(account_id)` | `subtensor/pallets/subtensor/runtime-api/src/lib.rs` (`StakeInfoRuntimeApi`) | arg: `AccountId32` |
| `get_identity_pinned` | `api::storage().registry().identity_of(&account_id)` | `subtensor/pallets/registry/src/lib.rs` (`IdentityOf`) | key: `AccountId32` |
| `list_proxies_pinned` | `api::storage().proxy().proxies(&account_id)` | `subtensor/pallets/proxy/src/lib.rs` (`Proxies`) | key: `AccountId32` |
| `get_delegate_pinned` | `api::apis().delegate_info_runtime_api().get_delegate(account_id)` | `subtensor/pallets/subtensor/runtime-api/src/lib.rs` (`DelegateInfoRuntimeApi`) | arg: `AccountId32` |
| `get_coldkey_swap_scheduled_pinned` | `api::storage().subtensor_module().coldkey_swap_announcements(&account_id)` | `subtensor/pallets/subtensor/src/lib.rs` (`ColdkeySwapAnnouncements`) | key: `AccountId32` |
| `get_child_keys_pinned` | `api::storage().subtensor_module().child_keys(&account_id, netuid)` | `subtensor/pallets/subtensor/src/lib.rs` (`ChildKeys`) | key1: `AccountId32`, key2: `NetUid` (`u16`) |
| `get_pending_child_keys_pinned` | `api::storage().subtensor_module().pending_child_keys(netuid, &account_id)` | `subtensor/pallets/subtensor/src/lib.rs` (`PendingChildKeys`) | key1: `NetUid` (`u16`), key2: `AccountId32` |
| `get_all_dynamic_info` | `api::apis().subnet_info_runtime_api().get_all_dynamic_info()` | `subtensor/pallets/subtensor/runtime-api/src/lib.rs` (`SubnetInfoRuntimeApi`) | no args |

Dispatchables submitted by `agcli audit`: **none** (query-only command).

### JSON output schema (`--output json`)

Top-level object fields:

| Field | Type |
|---|---|
| `address` | `string` |
| `balance_tao` | `number` |
| `total_staked_tao` | `number` |
| `total_value_tao` | `number` |
| `num_stakes` | `number` |
| `num_proxies` | `number` |
| `is_delegate` | `boolean` |
| `has_identity` | `boolean` |
| `coldkey_swap_scheduled` | `object \| null` (`execution_block: number`, `new_coldkey_hash: string`) |
| `childkey_delegations` | `array<object>` |
| `proxies` | `array<object>` (`delegate`, `proxy_type`, `delay`) |
| `stakes` | `array<object>` (`netuid`, `hotkey`, `stake_tao`, `subnet_name`, `price`, `tao_in_pool`) |
| `findings` | `array<object>` (`category`, `severity`, `message`) |

`childkey_delegations[*]` shape:
- `hotkey: string`
- `netuid: number`
- `children: array<{ proportion_raw: number, proportion_pct: number, child: string }>`
- optional `pending: { children: [...], cooldown_block: number }`

### Exit codes (from `src/error.rs`)

| Code | Meaning | Typical `agcli audit` triggers |
|---:|---|---|
| `0` | success | Query completed (including empty proxies/findings). |
| `1` | generic | Uncategorized errors. |
| `2` | clap parse | Invalid CLI syntax. |
| `10` | network | Endpoint unavailable / RPC connectivity failures. |
| `12` | validation | Invalid `--address` SS58 (or unresolved wallet coldkey). |
| `13` | chain | Runtime/storage read failures classified as chain errors. |
| `14` | I/O | Wallet file/path permission issues while resolving default coldkey. |
| `15` | timeout | Timeout from RPC/request path. |

### On-chain events

- **Events emitted by `agcli audit` itself:** none (no extrinsic submission).
- **Related events for the state this command inspects:**
  - Proxy state: `ProxyAdded`, `ProxyRemoved`, `Announced`, `ProxyExecuted` (`subtensor/pallets/proxy/src/lib.rs`).
  - Delegate state: `DelegateAdded`, `TakeIncreased`, `TakeDecreased` (`subtensor/pallets/subtensor/src/macros/events.rs`).
  - Child-key state: `SetChildrenScheduled`, `SetChildren`, `ChildKeyTakeSet` (`subtensor` events).
  - Coldkey swap state: `ColdkeySwapAnnounced`, `ColdkeySwapDisputed`, `ColdkeySwapReset`, `ColdkeySwapped`, `ColdkeySwapCleared`.
  - Identity state: `IdentitySet`, `IdentityDissolved` (`subtensor/pallets/registry/src/lib.rs`).
