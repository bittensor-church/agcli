# localnet — Docker subtensor management

`agcli localnet` manages a Dockerized subtensor node and can scaffold a full dev state (subnet + funded keys + registrations).

Subcommands in this group:

- `localnet start`
- `localnet stop`
- `localnet status`
- `localnet reset`
- `localnet logs`
- `localnet scaffold`

## Exit codes used by this command group

`main.rs` classifies errors with `src/error.rs`:

- `0`: success
- `1` (`GENERIC`): uncategorized failures (includes some localnet-specific timeout/not-found strings)
- `10` (`NETWORK`): websocket/RPC connection failures (`status`, `scaffold` chain calls)
- `12` (`VALIDATION`): invalid flag values (for example malformed `--container`, `--image`, or `--port 0`)
- `14` (`IO`): process/file errors (for example Docker binary not found)
- `15` (`TIMEOUT`): only if error text includes timeout patterns (for example global `--timeout` from `main.rs`)

Clap parse errors (missing required args / invalid value formats) exit with clap's code `2` before agcli error classification runs.

---

## `localnet start`

Start a Docker container and optionally wait for block production.

```bash
agcli localnet start \
  --image ghcr.io/opentensor/subtensor-localnet:devnet-ready \
  --container agcli_localnet \
  --port 9944 \
  --wait true \
  --timeout 120
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--image` | `Option<String>` | `ghcr.io/opentensor/subtensor-localnet:devnet-ready` |
| `--container` | `Option<String>` | `agcli_localnet` |
| `--port` | `Option<u16>` | `9944` |
| `--wait` | `Option<bool>` | `true` |
| `--timeout` | `Option<u64>` | `120` |

### JSON output schema (`--output json`)

```json
{
  "status": "started",
  "container_name": "string",
  "container_id": "string",
  "image": "string",
  "endpoint": "ws://127.0.0.1:<port>",
  "port": 9944,
  "block_height": 123,
  "dev_accounts": [
    { "name": "string", "uri": "string", "ss58": "string", "balance": "string" }
  ]
}
```

### Pallet / storage / events

- On-chain dispatchables: **none** (Docker orchestration only).
- Storage keys: **none**.
- Events: **none**.

### Exit-code notes

- Invalid Docker name/image or `--port 0` -> `12`.
- Port `>= 65534` is rejected at runtime because localnet maps two ports (`p` and `p+1`) -> `1`.
- Docker process failures are usually `14` (missing binary) or `1` (daemon/image failures).

---

## `localnet stop`

Force-remove the target container (`docker rm -f`).

```bash
agcli localnet stop --container agcli_localnet
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--container` | `Option<String>` | `agcli_localnet` |

### JSON output schema (`--output json`)

```json
{ "status": "stopped", "container_name": "string" }
```

### Pallet / storage / events

- On-chain dispatchables: **none**.
- Storage keys: **none**.
- Events: **none**.

### Exit-code notes

- Missing container currently returns a generic error string and maps to `1` (not idempotent).
- Invalid container name validation maps to `12`.

---

## `localnet status`

Inspect Docker state and, if running, query current block height over websocket.

```bash
agcli localnet status --container agcli_localnet --port 9944
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--container` | `Option<String>` | `agcli_localnet` |
| `--port` | `Option<u16>` | `9944` |

### JSON output schema (`--output json`)

```json
{
  "running": true,
  "container_name": "string",
  "container_id": "string|null",
  "image": "string|null",
  "endpoint": "string|null",
  "block_height": 123,
  "uptime": "docker-started-at-string|null"
}
```

### Pallet / storage / events

- On-chain dispatchables: **none**.
- Reads latest block header via RPC (`chain_getBlock` path in subxt block client), not pallet storage.
- Events: **none** (read-only).

### Exit-code notes

- Container-missing returns success (`running: false`), not an error.
- RPC failures when probing block height classify as `10`/`15`/`1`.

---

## `localnet reset`

Stop the container (best effort) then start fresh with the provided config.

```bash
agcli localnet reset \
  --image ghcr.io/opentensor/subtensor-localnet:devnet-ready \
  --container agcli_localnet \
  --port 9944 \
  --timeout 120
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--image` | `Option<String>` | `ghcr.io/opentensor/subtensor-localnet:devnet-ready` |
| `--container` | `Option<String>` | `agcli_localnet` |
| `--port` | `Option<u16>` | `9944` |
| `--timeout` | `Option<u64>` | `120` |

### JSON output schema (`--output json`)

```json
{
  "status": "reset",
  "container_name": "string",
  "container_id": "string",
  "endpoint": "ws://127.0.0.1:<port>",
  "block_height": 123
}
```

### Pallet / storage / events

- On-chain dispatchables: **none**.
- Storage keys: **none**.
- Events: **none**.

### Exit-code notes

- Same classification as `localnet start`.

---

## `localnet logs`

Fetch container logs through `docker logs`.

```bash
agcli localnet logs --container agcli_localnet --tail 200
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--container` | `Option<String>` | `agcli_localnet` |
| `--tail` | `Option<u32>` | `None` (full logs) |

### Output schema

- Always plain text log bytes.
- `--output json` does not change `logs` output shape.

### Pallet / storage / events

- On-chain dispatchables: **none**.
- Storage keys: **none**.
- Events: **none**.

### Exit-code notes

- Missing container and Docker failures are generic/IO (`1` or `14`).

---

## `localnet scaffold`

Run full local dev bootstrap:

1. Start or connect to local chain.
2. Register subnet(s).
3. Apply selected hyperparameters via sudo/AdminUtils.
4. Fund deterministic keys from `//Alice`.
5. Burn-register neurons and resolve UIDs.

```bash
agcli localnet scaffold \
  --config examples/scaffold.toml \
  --image ghcr.io/opentensor/subtensor-localnet:devnet-ready \
  --port 9944 \
  --no-start
```

### Clap flags

| Flag | Type | Default |
|---|---|---|
| `--config` | `Option<String>` | built-in defaults (`ScaffoldConfig::default`) |
| `--image` | `Option<String>` | from config/default localnet image |
| `--port` | `Option<u16>` | from config/default `9944` |
| `--no-start` | `bool` | `false` |

### JSON output schema (`--output json`)

```json
{
  "endpoint": "ws://127.0.0.1:9944",
  "container": "agcli_localnet|null",
  "block_height": 123,
  "subnets": [
    {
      "netuid": 1,
      "hyperparams": { "tempo": 100, "max_allowed_validators": 8 },
      "neurons": [
        {
          "name": "validator1",
          "ss58": "5...",
          "uid": 0,
          "balance_tao": 1000.0
        }
      ]
    }
  ]
}
```

`seed` is intentionally **not** serialized in JSON output (`NeuronResult.seed` has `#[serde(skip)]`).

### Pallet dispatchables, SCALE args, storage keys, and events

| Scaffold step | Dispatchable (pallet) | SCALE arg shape (as sent by agcli) | Primary storage keys touched/read | Emitted events |
|---|---|---|---|---|
| Create subnet | `SubtensorModule.register_network` | `(hotkey: AccountId32)` | `SubtensorModule::TotalNetworks`, `NetworksAdded`, `SubnetOwner`, `SubnetOwnerHotkey`, `NetworkRegisteredAt` | `SubtensorModule::NetworkAdded` |
| Fund neuron | `Balances.transfer_allow_death` | `(dest: MultiAddress::Id(AccountId32), value: u128)` | `System::Account` (sender+dest) | `Balances::Transfer` |
| Register neuron | `SubtensorModule.burned_register` | `(netuid: u16, hotkey: AccountId32)` | `SubtensorModule::Uids`, `Keys`, `IsNetworkMember`, `SubnetworkN` | `SubtensorModule::NeuronRegistered` |
| Set tempo | `Sudo.sudo(AdminUtils.sudo_set_tempo)` | `(netuid: u16, tempo: u16)` | `SubtensorModule::Tempo` | `Sudo::Sudid`, `SubtensorModule::TempoSet` |
| Set max validators | `Sudo.sudo(AdminUtils.sudo_set_max_allowed_validators)` | `(netuid: u16, max_allowed_validators: u16)` | `SubtensorModule::MaxAllowedValidators` | `Sudo::Sudid`, `SubtensorModule::MaxAllowedValidatorsSet` |
| Set max UIDs | `Sudo.sudo(AdminUtils.sudo_set_max_allowed_uids)` | `(netuid: u16, max_allowed_uids: u16)` | `SubtensorModule::MaxAllowedUids` | `Sudo::Sudid`, `SubtensorModule::MaxAllowedUidsSet` |
| Set min weights | `Sudo.sudo(AdminUtils.sudo_set_min_allowed_weights)` | `(netuid: u16, min_allowed_weights: u16)` | `SubtensorModule::MinAllowedWeights` | `Sudo::Sudid`, `SubtensorModule::MinAllowedWeightSet` |
| Set immunity period | `Sudo.sudo(AdminUtils.sudo_set_immunity_period)` | `(netuid: u16, immunity_period: u16)` | `SubtensorModule::ImmunityPeriod` | `Sudo::Sudid`, `SubtensorModule::ImmunityPeriodSet` |
| Set weights rate limit | `Sudo.sudo(AdminUtils.sudo_set_weights_set_rate_limit)` | `(netuid: u16, weights_set_rate_limit: u64)` | `SubtensorModule::WeightsSetRateLimit` | `Sudo::Sudid`, `SubtensorModule::WeightsSetRateLimitSet` |
| Set commit-reveal | `Sudo.sudo(AdminUtils.sudo_set_commit_reveal_weights_enabled)` | `(netuid: u16, enabled: bool)` | `SubtensorModule::CommitRevealWeightsEnabled` | `Sudo::Sudid`, `SubtensorModule::CommitRevealEnabled` |
| Set activity cutoff | `Sudo.sudo(AdminUtils.sudo_set_activity_cutoff)` | `(netuid: u16, activity_cutoff: u16)` | `SubtensorModule::ActivityCutoff` | `Sudo::Sudid`, `SubtensorModule::ActivityCutoffSet` |

`scaffold` also attempts `AdminUtils.sudo_set_max_weight_limit`, but that call is not present in the current subtensor runtime metadata; agcli catches this and logs a warning instead of panicking.

### Exit-code notes

- Config parse/load failures: `14` or `1`.
- Local endpoint safety guard (non-local websocket target): `1`.
- Chain connectivity failures: `10`/`15`.
- Sudo inner dispatch failures bubble as chain errors (`13`) when the error string carries pallet context.

## Source references

- CLI handler: `src/cli/localnet_cmds.rs`
- Docker manager: `src/localnet.rs`
- Scaffold orchestration: `src/scaffold.rs`
- Error classification: `src/error.rs`
- Subtensor dispatchables/events/storage:
  - `subtensor/pallets/subtensor/src/macros/dispatches.rs`
  - `subtensor/pallets/subtensor/src/macros/events.rs`
  - `subtensor/pallets/subtensor/src/lib.rs`
  - `subtensor/pallets/admin-utils/src/lib.rs`
