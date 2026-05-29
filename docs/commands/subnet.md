# subnet — audited command reference

This page is an audit refresh for `SubnetCommands` in `src/cli/mod.rs` and handlers in
`src/cli/subnet_cmds.rs`.

## Exit codes (from `src/error.rs`)

All subnet subcommands use the global classifier:

- `0`: success
- `1`: generic/uncategorized failure
- `10`: network/connectivity failure
- `11`: auth/wallet failure
- `12`: validation failure (includes `Subnet N not found`)
- `13`: chain/runtime dispatch failure
- `14`: file I/O failure (cache commands)
- `15`: timeout

---

## Read/query subcommands

### `subnet list`
- Flags: `--at-block <u32>`
- JSON schema: `Array<SubnetInfo>`
- Pallet/runtime refs:
  - Runtime APIs: `subnet_info_runtime_api::get_subnets_info`, `subnet_info_runtime_api::get_all_dynamic_info`
  - Storage behind runtime API includes `NetworksAdded`, `SubnetworkN`, `TokenSymbol`, `Tempo`, `SubnetOwner`
- Events emitted: none (read-only)

### `subnet show` (alias: `subnet info`)
- Flags: `--netuid <u16> [--at-block <u32>]`
- JSON schema: `SubnetInfo`
- Pallet/runtime refs:
  - Runtime APIs: `subnet_info_runtime_api::get_subnet_info`, `subnet_info_runtime_api::get_dynamic_info`
  - Uses dynamic pool state (`SubnetTAO`, `SubnetAlphaIn`, `SubnetAlphaOut`) via runtime API
- Events emitted: none

### `subnet hyperparams`
- Flags: `--netuid <u16> [--at-block <u32>]`
- JSON schema: `SubnetHyperparameters`
- Pallet/runtime refs:
  - Runtime API: `subnet_info_runtime_api::get_subnet_hyperparams`
  - Includes values corresponding to storage such as `Tempo`, `Rho`, `Kappa`, `MinBurn`, `MaxBurn`, `WeightsSetRateLimit`
- Events emitted: none

### `subnet metagraph`
- Flags: `--netuid <u16> [--uid <u16>] [--at-block <u32>] [--full] [--save]`
- JSON schema:
  - with `--uid`: `NeuronInfo`
  - default: `Array<NeuronInfoLite>`
  - cache write side effect with `--save` writes `~/.agcli/metagraph/sn<N>/block-<B>.json`
- Pallet/runtime refs:
  - Runtime APIs: `neuron_info_runtime_api::get_neurons_lite`, `neuron_info_runtime_api::get_neuron`
  - Requires subnet existence check (`NetworksAdded`)
- Events emitted: none

### `subnet cache-load`
- Flags: `--netuid <u16> [--block <u64>]`
- JSON schema: `Metagraph` on hit, `{ "error": string }` on miss
- Pallet/runtime refs:
  - Chain read: subnet existence check before local file read
  - Local storage: `~/.agcli/metagraph/sn<N>/block-*.json`
- Events emitted: none

### `subnet cache-list`
- Flags: `--netuid <u16>`
- JSON schema: `{ "netuid": number, "snapshots": number[] }`
- Pallet/runtime refs: subnet existence check (`NetworksAdded`) + local cache directory listing
- Events emitted: none

### `subnet cache-diff`
- Flags: `--netuid <u16> [--from-block <u64>] [--to-block <u64>]`
- JSON schema: `Array<MetagraphDelta { uid, hotkey, kind }>`
- Pallet/runtime refs:
  - Requires subnet existence check (`NetworksAdded`)
  - Reads local cache; when `--to-block` omitted fetches live metagraph from runtime APIs
- Events emitted: none

### `subnet cache-prune`
- Flags: `--netuid <u16> [--keep <usize=10>]`
- JSON schema: `{ "netuid": number, "removed": number, "kept": number }`
- Pallet/runtime refs: subnet existence check (`NetworksAdded`) + local cache deletion
- Events emitted: none

### `subnet probe`
- Flags: `--netuid <u16> [--uids <string csv>] [--timeout-ms <u64=3000>] [--concurrency <usize=32>]`
- JSON schema:
  - success: `Array<{ uid, hotkey, ip, port, status, latency_ms|null, version }>`
  - empty: `{ "error": "No neurons to probe", "netuid": number }`
- Pallet/runtime refs:
  - Runtime APIs: `get_neurons_lite`, `get_neuron`
  - Subnet existence check via `NetworksAdded`
- Events emitted: none

### `subnet watch`
- Flags: `--netuid <u16> [--interval <u64=12>]`
- JSON schema: none (terminal dashboard only)
- Pallet/runtime refs:
  - Polls block number + subnet hyperparams + dynamic info
  - Subnet existence check first
- Events emitted: none

### `subnet liquidity`
- Flags: `[--netuid <u16>]`
- JSON schema:
  - `Array<{ netuid, name, price, tao_in, alpha_in, liquidity_depth_tao, slippage_estimates[] }>`
- Pallet/runtime refs:
  - Runtime API: `subnet_info_runtime_api::get_dynamic_info` / `get_all_dynamic_info`
  - Dynamic pool storage includes `SubnetTAO`, `SubnetAlphaIn`, `SubnetAlphaOut`, `SubnetVolume`
- Events emitted: none

### `subnet monitor`
- Flags: `--netuid <u16> [--interval <u64=24>] [--json]`
- JSON schema (`--json` stream): one object per line with `"event"` in
  `registration|deregistration|hotkey_change|emission_shift|incentive_shift|inactive`
- Pallet/runtime refs:
  - Polls block number + `get_neurons_lite`
  - Subnet existence check first
- Events emitted: none (local diff stream)

### `subnet health`
- Flags: `--netuid <u16>`
- JSON schema:
  `{ netuid, block, total_neurons, active, validators, miners, zero_emission, stale_neurons, price, commit_reveal, neurons[] }`
- Pallet/runtime refs:
  - Pinned block reads: `get_neurons_lite_at_block`, `get_dynamic_info_at_block`, `get_subnet_hyperparams_at_block`
  - Subnet existence check first
- Events emitted: none

### `subnet emissions`
- Flags: `--netuid <u16>`
- JSON schema:
  `{ netuid, total_emission_per_block_tao, daily_emission_tao, tempo, neurons[] }`
- Pallet/runtime refs:
  - Pinned block reads: neurons + dynamic info
  - Subnet existence check first
- Events emitted: none

### `subnet cost`
- Flags: `--netuid <u16>`
- JSON schema:
  `{ netuid, burn_rao, burn_tao, difficulty, neurons, max_neurons, registration_allowed, price, min_burn, max_burn }`
- Pallet/runtime refs:
  - Pinned reads: subnet info + hyperparams + dynamic info
  - Subnet existence check first
- Events emitted: none

### `subnet commits`
- Flags: `--netuid <u16> [--hotkey-address <ss58>]`
- JSON schema:
  - CR disabled: `{ netuid, commit_reveal_enabled: false, message }`
  - CR enabled: `{ netuid, block, commit_reveal_enabled: true, reveal_period_epochs, commits[] }`
- Pallet/runtime refs:
  - Storage: `WeightCommits`, `RevealPeriodEpochs`
  - Reads hyperparams for commit-reveal enablement
- Events emitted: none

### `subnet emission-split`
- Flags: `--netuid <u16>`
- JSON schema:
  - configured: `{ netuid, configured: true, total_weight, split[] }`
  - default: `{ netuid, configured: false, message }`
- Pallet/storage refs:
  - Storage: `SubtensorModule::MechanismEmissionSplit`
- Events emitted: none

### `subnet mechanism-count`
- Flags: `--netuid <u16>`
- JSON schema: `{ "netuid": number, "mechanism_count": number }`
- Pallet/storage refs:
  - Storage: `SubtensorModule::MechanismCountCurrent`
- Events emitted: none

### `subnet check-start`
- Flags: `--netuid <u16>`
- JSON schema:
  `{ netuid, active, neurons, min_neurons_to_start, can_start, tempo }`
- Pallet/storage refs:
  - `active` currently comes from `SubtensorModule::NetworksAdded` (existence bit)
  - Also reads hyperparams + neuron list
- Events emitted: none

### `subnet create-cost`
- Flags: none
- JSON schema: `{ cost_rao, cost_tao }`
- Pallet/runtime refs:
  - Runtime API: `subnet_registration_runtime_api::get_network_registration_cost`
- Events emitted: none

---

## Write/extrinsic subcommands

### `subnet register`
- Flags: none (wallet/global flags still apply)
- JSON schema: none (human text only)
- Pallet call: `SubtensorModule::register_network(hotkey)`
- Storage touched (from subnet registration flow): `NetworksAdded`, `SubnetOwner`, `SubnetOwnerHotkey`,
  `NetworkRegisteredAt`, `TokenSymbol`, `SubnetworkN`, `TotalNetworks`, plus initial subnet parameter storage
- Events: `NetworkAdded` (+`NetworkRemoved` if pruning happened)

### `subnet register-with-identity`
- Flags:
  `--name <string> --github <string> --contact <string> --url <string> --discord <string> --description <string> --additional <string>`
- JSON schema: none (human text only)
- Pallet call (intended): `SubtensorModule::register_network_with_identity(hotkey, Option<SubnetIdentityOfV3>)`
- Storage/event intent:
  - New subnet registration storage (same as `register`)
  - `SubnetIdentitiesV3`
  - Events: `NetworkAdded`, `SubnetIdentitySet` (when identity supplied)

### `subnet register-leased`
- Flags: `[--emissions-share <u8=100>] [--end-block <u32>]`
- JSON schema: none (human text only)
- Pallet call: `SubtensorModule::register_leased_network(emissions_share, end_block)`
- Signer: coldkey (not hotkey)
- Storage/event intent:
  - lease maps `SubnetLeases`, `SubnetUidToLeaseId`, `SubnetLeaseShares`
  - events include `SubnetLeaseCreated` and subnet creation `NetworkAdded`

### `subnet terminate-lease`
- Flags: `--netuid <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call target: `SubtensorModule::terminate_lease(...)`
- Storage/event intent:
  - lease cleanup in `SubnetLeases`, `SubnetLeaseShares`, `AccumulatedLeaseDividends`
  - subnet ownership transition via `SubnetOwner` and owner hotkey storage
  - event: `SubnetLeaseTerminated`

### `subnet root-dissolve`
- Flags: `--netuid <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call: `SubtensorModule::root_dissolve_network(netuid)`
- Storage touched: subnet teardown (`NetworksAdded`, `SubnetOwner`, `SubnetworkN`, `TokenSymbol`, many subnet maps)
- Events: `NetworkRemoved`

### `subnet register-neuron`
- Flags: `--netuid <u16>`
- JSON schema: none (human text only)
- Pallet call: `SubtensorModule::burned_register(netuid, hotkey)` → `do_register`
- **Cost:** dynamic burn in **τ (TAO)** from coldkey free balance — **permanently burned/recycled**, not staked. Current price: `agcli subnet cost --netuid N`. Hyperparam bounds: `min_burn` / `max_burn` (RAO on-chain; set via `subnet set-param`).
- **Preflight (client):** subnet exists, registration allowed, balance ≥ burn, capacity warning if full.
- Storage touched: neuron registry maps (`Uids`, `Keys`, `IsNetworkMember`, stake/metric maps)
- Events: `NeuronRegistered`
- Common errors (exit 13): `NotEnoughBalanceToStake` (insufficient τ for burn), `HotKeyAlreadyRegisteredInSubNet`, `SubNetRegistrationDisabled`, `NoNeuronIdAvailable`, `TooManyRegistrationsThisBlock`

### `subnet register-limit`
- Flags: `--netuid <u16> --limit-price <f64>` (decimal **τ**; encoded as RAO u64 on-chain)
- JSON schema: none (human text only)
- Pallet call: `SubtensorModule::register_limit(netuid, hotkey, limit_price)` → `do_register_limit` → `do_register`
- **Cost:** same burn semantics as `register-neuron`, but tx fails if `get_burn(netuid) > limit_price` at execution (`RegistrationPriceLimitExceeded`).
- **Preflight (client):** same as `register-neuron`, plus current burn ≤ `--limit-price`.
- Use when burn is volatile and you want a cap between quote and block inclusion.

### `subnet pow`
- Flags: `--netuid <u16> [--threads <u32=4>]`
- JSON schema: none
- Pallet calls:
  - registration submit: `SubtensorModule::register(netuid, block_number, nonce, work, hotkey, coldkey)`
  - prereads: `Difficulty`, `NetworkPowRegistrationAllowed`, `registration_allowed`
- **Preflight (client):** subnet exists, registration allowed, PoW registration allowed, difficulty bounds shown.
- **Failure:** exits non-zero if PoW not found (was silent success before).
- Events: `NeuronRegistered` on successful registration
- Common errors: `InvalidWorkBlock` (template >3 blocks old), `InvalidDifficulty`, `POWRegistrationDisabled`

### `subnet dissolve`
- Flags: `--netuid <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call currently used by agcli: `SubtensorModule::dissolve_network(coldkey, netuid)`
- Storage/event intent: same subnet teardown path as root dissolve; event `NetworkRemoved`

### `subnet set-param`
- Flags: `--netuid <u16> --param <string> [--value <string>]`
- JSON schema:
  - `--param list`: `{ "parameters": [{ name, type, value_encoding, scope, description }] }`
  - write success: `{ "tx_hash": "0x..." }`
- Pallet call: dynamic `AdminUtils::<sudo_set_*>`
- **Value encoding (critical for agents):**
  - `kappa`, `bonds_penalty`: on-chain `u16`; runtime uses `value ÷ 65535` as float in [0, 1]. Pass `--value 0.5` (decimal) or raw u16 (e.g. `32767`).
  - `rho`: on-chain `u16` sigmoid steepness; **not** divided by 65535 (default chain value 10; typical range ~1–40).
  - `min_burn`, `max_burn`: on-chain RAO. Pass decimal TAO (e.g. `1.0`) or raw RAO integer.
  - `alpha_values`: comma-separated `low,high` normalized decimals (e.g. `0.7,0.9`) or raw u16 pair. Requires `liquid_alpha_enabled`.
  - Most other params: plain integer (blocks, counts, version keys).
  - Run `agcli subnet set-param --netuid N --param list` for the full table with `value_encoding` column.
- Storage/events:
  - depends on param; maps to Subtensor storage keys like `Tempo`, `Rho`, `Kappa`, `MaxAllowedUids`,
    `MinBurn`, `MaxBurn`, `WeightsSetRateLimit`, `MechanismCountCurrent`, etc.
  - typical events from Subtensor: `TempoSet`, `RhoSet`, `KappaSet`, `ImmunityPeriodSet`,
    `MinAllowedWeightSet`, `MaxAllowedUidsSet`, `MaxAllowedValidatorsSet`, `MinDifficultySet`,
    `MaxDifficultySet`, `WeightsVersionKeySet`, `WeightsSetRateLimitSet`, `AdjustmentIntervalSet`,
    `AdjustmentAlphaSet`, `ActivityCutoffSet`, `RegistrationAllowed`, `PowRegistrationAllowed`,
    `RegistrationPerIntervalSet`, `MinBurnSet`, `MaxBurnSet`, `BondsMovingAverageSet`,
    `MaxRegistrationsPerBlockSet`, `ServingRateLimitSet`, `DifficultySet`,
    `CommitRevealEnabled`, `CommitRevealPeriodsSet`, `BondsPenaltySet`, `CommitRevealVersionSet`,
    `MinAllowedUidsSet`, `MinNonImmuneUidsSet`
  - admin-utils-specific events appear for some params (`Yuma3EnableToggled`, `BondsResetToggled`)

### `subnet set-symbol`
- Flags: `--netuid <u16> --symbol <string>`
- JSON schema: none (always human text today)
- Pallet call: `SubtensorModule::update_symbol(netuid, symbol)`
- Storage touched: `TokenSymbol`
- Events: `SymbolUpdated { netuid, symbol }`

### `subnet trim`
- Flags: `--netuid <u16> --max-uids <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call (agcli dynamic): `SubtensorModule::sudo_set_max_allowed_uids(netuid, max_uids)`
- Storage touched: `MaxAllowedUids` (+ downstream UID trimming side effects if runtime enforces)
- Events: `MaxAllowedUidsSet`

### `subnet start`
- Flags: `--netuid <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call: `SubtensorModule::start_call(netuid)`
- Storage touched: `FirstEmissionBlockNumber`, `SubtokenEnabled`
- Events: `FirstEmissionBlockNumberSet(netuid, block)`

### `subnet set-mechanism-count`
- Flags: `--netuid <u16> --count <u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call (agcli dynamic): `SubtensorModule::sudo_set_mechanism_count(netuid, count)`
- Storage touched: `MechanismCountCurrent` (and may clear `MechanismEmissionSplit`)
- Events: no dedicated event emitted by `do_set_mechanism_count` in current pallet code

### `subnet set-emission-split`
- Flags: `--netuid <u16> --weights <comma-separated u16>`
- JSON schema: `{ "tx_hash": "0x..." }` when `--output json`
- Pallet call (agcli dynamic): `SubtensorModule::sudo_set_mechanism_emission_split(netuid, weights)`
- Storage touched: `MechanismEmissionSplit`
- Events: no dedicated event emitted by `do_set_emission_split` in current pallet code

### `subnet snipe`
- Flags:
  `--netuid <u16> [--max-cost <f64>] [--max-attempts <u64>] [--all-hotkeys] [--fast] [--watch]`
- JSON schema:
  - success in register mode: `{ status, netuid, hotkey, tx_hash, attempts, elapsed_secs, burn_rao }`
  - watch mode: none (streaming terminal output)
- Pallet calls:
  - write path: `SubtensorModule::burned_register(netuid, hotkey)`
  - read preflight: subnet info + account balance
- Events: `NeuronRegistered` on successful register

---

## Audit-only call mapping notes (code vs pallet)

These are implementation facts discovered while tracing `src/cli/subnet_cmds.rs` into subxt payloads:

1. `terminate-lease` accepts `--netuid` at the CLI; `Client::terminate_lease` resolves `lease_id` and lease `hotkey` from storage before submitting `terminate_lease(lease_id, hotkey)`.
2. `register-with-identity` builds `SubnetIdentity` without `logo_url`, but current
   `SubnetIdentityV3` includes `logo_url`.
3. `dissolve` command is labeled owner flow in CLI UX, but the called dispatchable
   `dissolve_network(origin, _coldkey, netuid)` is root-gated in current pallet code.

---

## In-scope pallet dispatchables without subnet-command surface

Within the audited dispatchables list, these are not directly surfaced as `agcli subnet ...` commands:

- `SubtensorModule::set_subnet_identity` (available under identity command group, not subnet group)
