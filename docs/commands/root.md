# root — Root Network Operations

The root network (SN0, `netuid = 0`) is the meta-subnet that governs emission distribution across all subnets. Root validators set weights to determine which subnets receive more emissions each epoch.

## Subcommands

| Subcommand | Description | Pallet dispatchable |
|---|---|---|
| `root register` | Register hotkey on root network | `SubtensorModule::root_register` |
| `root weights` | Set root weights for subnet emission distribution | `SubtensorModule::set_weights` (netuid=0) |

---

### `root register`

Register a hotkey on the root network (SN0). The coldkey signs the transaction; the hotkey to register is derived from the wallet's hotkeys directory.

```bash
agcli root register [global options]
```

**Clap flags / types**

| Flag | Type | Required | Description |
|---|---|---|---|
| *(none at subcommand level)* | | | All context comes from global flags |

**Relevant global flags**

| Flag | Default | Description |
|---|---|---|
| `--wallet` / `-w` | `default` | Wallet name under `--wallet-dir` |
| `--hotkey-name` | `default` | Hotkey file name under `<wallet>/hotkeys/` |
| `--wallet-dir` | `~/.bittensor/wallets` | Root wallet directory |
| `--password` | (none) | Coldkey password (env: `AGCLI_PASSWORD`) |
| `--yes` / `-y` | false | Skip confirmation prompts |
| `--dry-run` | false | Parse and validate; do not submit |
| `--output` | `table` | Output format (`table`, `json`, `csv`) |

**Output**

Human-readable only (see Findings). `--output json` is **silently ignored**; the handler always writes to stdout with `println!`.

```
Registering on root network with hotkey 5Abc…
Registered on root network with hotkey 5Abc….
  Tx: 0xdeadbeef…
```

**Exit codes**

| Code | Meaning |
|---|---|
| `0` | Success — hotkey registered on root |
| `11` (AUTH) | Wallet locked, wrong password, or missing hotkey file |
| `12` (VALIDATION) | Invalid SS58 address, or hotkey already registered on root |
| `13` (CHAIN) | Extrinsic rejected by runtime (e.g., `StakeTooLowForRoot`, `HotKeyAlreadyRegisteredInSubNet`) |
| `10` (NETWORK) | RPC endpoint unreachable or connection timeout |
| `15` (TIMEOUT) | Finalization deadline exceeded (default 30 s) |

**Pallet ref**

- Pallet: `SubtensorModule`
- Dispatchable: `root_register(origin: OriginFor<T>, hotkey: T::AccountId)`
- Call index: 62
- Origin: signed coldkey
- File: `subtensor/pallets/subtensor/src/macros/dispatches.rs` line ~1009
- Implementation: `subtensor/pallets/subtensor/src/subnets/` (do_root_register)

**SCALE encoding**

| Arg | On-chain type | SDK encoding |
|---|---|---|
| `hotkey` | `T::AccountId` (32-byte AccountId32) | `ss58_to_account_id` → `AccountId32` via typed `api::tx()` |

Encoding is compile-time verified through the generated `api::tx().subtensor_module().root_register(hk)` typed call.

**On-chain events emitted**

| Event | When |
|---|---|
| `NeuronRegistered(NetUid(0), uid, hotkey_account)` | Hotkey successfully registered on root (SN0) |

---

### `root weights`

Set validator weights on the root network (SN0) to control relative emission share per subnet. The hotkey signs the transaction. Weights are u16 values; they are normalized on-chain.

```bash
agcli root weights --weights "NETUID:WEIGHT[,NETUID:WEIGHT...]" [global options]
```

**Clap flags / types**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--weights` | `String` | Yes | Comma-separated `netuid:weight` pairs, e.g. `"1:500,2:300,3:200"` |

**Relevant global flags**

| Flag | Default | Description |
|---|---|---|
| `--wallet` / `-w` | `default` | Wallet name |
| `--hotkey-name` | `default` | Hotkey that signs the transaction |
| `--wallet-dir` | `~/.bittensor/wallets` | Root wallet directory |
| `--password` | (none) | Coldkey password (unlocks wallet to load hotkey pair) |
| `--yes` / `-y` | false | Skip confirmation prompts |
| `--dry-run` | false | Parse and validate; do not submit |
| `--output` | `table` | Output format (`table`, `json`, `csv`) |

**Output**

Human-readable only (see Findings). `--output json` is **silently ignored**.

```
Setting 3 root weights
Root weights set (3 UIDs).
  Tx: 0xdeadbeef…
```

**Weight string format**

- Format: `"NETUID:WEIGHT[,NETUID:WEIGHT,...]"`
- NETUID and WEIGHT must both be in `[0, 65535]` (u16 range).
- Parser (`parse_weight_pairs`) trims whitespace around `:` and `,`.
- Empty string, missing `:`, extra `:`, or out-of-range values are rejected at runtime with exit code `12`.

**Version key**

`version_key` is hardcoded to `0` in the handler. There is no `--version-key` flag (see Findings).

**Exit codes**

| Code | Meaning |
|---|---|
| `0` | Success — weights set on root |
| `11` (AUTH) | Wallet locked, wrong password, or missing hotkey |
| `12` (VALIDATION) | Malformed `--weights` string, UID/weight out of u16 range |
| `13` (CHAIN) | Extrinsic rejected (e.g., `NotEnoughStakeToSetWeights`, `SettingWeightsTooFast`, `NotRootSubnet`) |
| `10` (NETWORK) | RPC endpoint unreachable |
| `15` (TIMEOUT) | Finalization deadline exceeded |

**Pallet ref**

- Pallet: `SubtensorModule`
- Dispatchable: `set_weights(origin, netuid, dests, weights, version_key)`
- Origin: signed hotkey
- `netuid` passed: `0` (i.e., `NetUid::ROOT`)
- File: `subtensor/pallets/subtensor/src/macros/dispatches.rs` line ~85

**SCALE encoding**

| Arg | On-chain type | SDK encoding |
|---|---|---|
| `netuid` | `NetUid` (u16 newtype) | `netuid.0` → `u16` |
| `dests` | `Vec<u16>` | `uids.to_vec()` |
| `weights` | `Vec<u16>` | `wts.to_vec()` |
| `version_key` | `u64` | hardcoded `0u64` |

Encoding verified at compile time via `api::tx().subtensor_module().set_weights(...)`.

**Storage key**

Root weights are stored under the general `Weights` storage map keyed by `(NetUid(0), uid)`:
- Pallet: `SubtensorModule`
- Storage: `Weights` (`StorageDoubleMap<NetUid, uid: u16, Vec<(u16, u16)>>`)

**On-chain events emitted**

| Event | When |
|---|---|
| `WeightsSet(NetUidStorageIndex(0), uid)` | Weights successfully recorded for the hotkey's UID on root |

---

## Related pallet dispatches with no `root` CLI surface

The following dispatchables are implemented in `src/chain/extrinsics.rs` but are NOT accessible under `agcli root`. They are surfaced under other command groups or have no CLI surface at all:

| Dispatch | SDK function | CLI surface | Notes |
|---|---|---|---|
| `root_dissolve_network(netuid)` | `client.root_dissolve_network(pair, netuid)` | `agcli subnet root-dissolve --netuid N` | Root-origin only; uses `submit_raw_call` (not typed) |
| `claim_root(subnets)` | `client.claim_root(pair, &[netuid])` | `agcli stake claim-root --netuid N` | Claims root dividends; CLI limited to single subnet |

---

## Exit code reference

Full exit code table from `src/error.rs`:

| Code | Constant | Trigger |
|---|---|---|
| `0` | — | Success |
| `1` | `GENERIC` | Uncategorized / unexpected error |
| `10` | `NETWORK` | Network/RPC unreachable, DNS failure |
| `11` | `AUTH` | Wrong password, missing key, locked wallet |
| `12` | `VALIDATION` | Bad input, invalid SS58, parse failure |
| `13` | `CHAIN` | Extrinsic rejected by runtime |
| `14` | `IO` | File permission denied, missing file |
| `15` | `TIMEOUT` | Operation exceeded deadline |

---

## Source code references

- **agcli handler**: `src/cli/network_cmds.rs` — `handle_root()` starting at line 11
  - `RootCommands::Register` handled at line 19
  - `RootCommands::Weights` handled at line 36
- **CLI enum**: `src/cli/mod.rs` — `pub enum RootCommands` at line ~1308
- **Extrinsics**:
  - `root_register`: `src/chain/extrinsics.rs` line ~753
  - `set_weights` (used for root): `src/chain/extrinsics.rs` line ~445
  - `root_dissolve_network`: `src/chain/extrinsics.rs` line ~2357 (raw call)
  - `claim_root`: `src/chain/extrinsics.rs` line ~257 (typed call)
- **Subtensor pallet dispatches**: `subtensor/pallets/subtensor/src/macros/dispatches.rs`
  - `root_register` — call index 62
  - `set_weights` — call index (see dispatch file, line ~85)
  - `root_dissolve_network` — call index 120
  - `claim_root` — call index 121

---

## Findings (audit observations)

**1. Docs referenced `set_root_weights` — a dispatch that does not exist.**
The previous `docs/commands/root.md` cited `SubtensorModule::set_root_weights(origin, netuid, hotkey, dests, weights, version_key)`. No such dispatch exists in the pallet. The actual on-chain call made by `root weights` is `SubtensorModule::set_weights(origin, netuid=0, dests, weights, version_key)`. The pallet has no dedicated `set_root_weights` entry point; root weight-setting is ordinary `set_weights` with `netuid=0`.

**2. `--version-key` missing from `root weights`.**
The `set_weights` extrinsic requires a `version_key: u64` argument. `RootCommands::Weights` hardcodes `version_key = 0`. If the root subnet's `WeightsVersionKey` storage advances, callers have no way to supply the correct version key through the CLI, causing `VersionKeyMismatch` on-chain. A `--version-key` flag (defaulting to `0`) should be added.

**3. Neither `root register` nor `root weights` respects `--output json`.**
Both handlers write output with `println!` and never read `ctx.output`. An agent that passes `--output json` receives human-readable text with exit `0`, making it impossible to machine-parse the transaction hash or detect success programmatically without parsing stderr/stdout.

**4. `root_dissolve_network` uses `Value::u128` for a `NetUid` (u16 newtype).**
`client.root_dissolve_network` calls `submit_raw_call` with `Value::u128(netuid.0 as u128)`. The pallet dispatch takes `netuid: NetUid` (a newtype around `u16`). The subxt dynamic API resolves encoding from metadata at runtime, so this works in practice — but an incorrect metadata entry or a future pallet change would cause a silent SCALE mismatch rather than a compile-time error. The typed `api::tx()` path would provide compile-time verification.

**5. `claim_root` CLI (`stake claim-root`) accepts only one `--netuid`; pallet accepts `BTreeSet<NetUid>` up to `MAX_SUBNET_CLAIMS`.**
The pallet `claim_root` dispatch accepts a `BTreeSet<NetUid>` — callers can batch-claim dividends from multiple subnets in a single transaction. `StakeCommands::ClaimRoot` exposes only a single `--netuid: u16` flag, forcing agents to make N separate transactions to claim from N subnets. This is surfaced under `stake claim-root`, not `root`, but is a root-pallet gap.

**6. `RootCommands` has only 2 variants; 3 root-pallet dispatches are hidden under other groups.**
`agcli root` covers `root_register` and `set_weights(netuid=0)`. `root_dissolve_network` and `claim_root` live under `agcli subnet root-dissolve` and `agcli stake claim-root` respectively. There is no `agcli root dissolve` or `agcli root claim` alias, making the `root` group an incomplete surface for root-network operations.

---

## Suggested follow-ups

- **Source** (`network_cmds.rs`): Add `--version-key` flag (type `u64`, default `0`) to `RootCommands::Weights` and thread it through to `client.set_weights(..., version_key)`.
- **Source** (`network_cmds.rs`): Emit JSON (`{"tx": "0x…", "hotkey": "5…"}`) when `ctx.output.is_json()` is true in both `Register` and `Weights` handlers.
- **Source** (`extrinsics.rs`): Switch `root_dissolve_network` from `submit_raw_call` to the typed `api::tx().subtensor_module().root_dissolve_network(netuid)` path to get compile-time verification.
- **Source** (`stake_cmds.rs` / `mod.rs`): Extend `StakeCommands::ClaimRoot` to accept `--netuids` as a comma-separated list (Vec<u16>) to allow multi-subnet claims matching the pallet `BTreeSet<NetUid>` API.
- **Docs** (`root.md` and `stake.md`): Cross-link `agcli stake claim-root` and `agcli subnet root-dissolve` in the root docs so users understand the full root-network operation surface.

---

## Related commands

- `agcli view dynamic` — Current emission distribution per subnet
- `agcli subnet list` — All registered subnets and their netuid values
- `agcli stake claim-root --netuid N` — Claim root dividends (ClaimRoot pallet dispatch)
- `agcli subnet root-dissolve --netuid N` — Root-dissolve a subnet (root origin required)
- `agcli explain --topic root` — Root network concept docs
