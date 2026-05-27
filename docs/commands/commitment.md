# commitment — Commitment Pallet CLI Surface

Miner/operator metadata commitments are managed through the `commitment` command group.

Handler entrypoint:
- `src/cli/network_cmds.rs::handle_commitment`

Clap enum:
- `src/cli/mod.rs::CommitmentCommands`

Pallet reference:
- `subtensor/pallets/commitments/src/lib.rs`

## Subcommand index

1. `commitment set`
2. `commitment get`
3. `commitment list`

---

## `commitment set`

Submit a commitment payload to the chain.

```bash
agcli commitment set \
  --netuid 1 \
  --data "endpoint:http://127.0.0.1:8091,version:1.0"
```

### Clap flags + types

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | Subnet UID |
| `--data` | `String` | yes | Comma-separated fields; validated non-empty and `<= 1024` bytes total |

### Runtime call mapping

- agcli call path:
  - `handle_commitment(Set)` -> `Client::set_commitment`
  - `src/chain/extrinsics.rs::set_commitment`
- subxt dynamic tx:
  - pallet: `"Commitments"`
  - dispatchable: `"set_commitment"`
  - args (metadata-typed SCALE):
    1. `netuid` -> provided as numeric value (fits runtime `NetUid` / `u16`)
    2. `info` -> `CommitmentInfo { fields: Vec<Data> }`
- agcli payload encoding detail:
  - Splits `--data` on commas.
  - Encodes each field as `Data::RawN([u8; N])` where `N <= 128`.
  - Uses variant names `Raw0..Raw128` from runtime metadata.
  - Fields longer than 128 bytes are truncated to 128 before SCALE encoding.

### Pallet/storage/events

- Pallet fn: `Commitments::set_commitment(origin, netuid, info)`
- Primary storage touched by pallet:
  - `CommitmentOf(netuid, who)` (write)
  - `LastCommitment(netuid, who)` (write)
  - `UsedSpaceOf(netuid, who)` (write)
  - `LastBondsReset(netuid, who)` (conditional write)
  - `TimelockedIndex` (write)
- Events emitted by pallet:
  - `Commitment { netuid, who }` for non-timelocked payloads
  - `TimelockCommitment { netuid, who, reveal_round }` for timelocked payloads

### Output JSON schema (`--output json`)

```json
{
  "type": "object",
  "required": ["tx_hash"],
  "properties": {
    "tx_hash": { "type": "string" }
  },
  "additionalProperties": false
}
```

### Exit codes

Classification is from `src/error.rs`:
- `0` success
- `11` auth/wallet unlock issues
- `12` validation (`invalid netuid`, empty/oversized `--data`, parse issues)
- `13` chain dispatch failure (`TooManyFieldsInCommitmentInfo`, `AccountNotAllowedCommit`, `SpaceLimitExceeded`, etc.)
- `10` network/RPC connection errors
- `15` timeouts
- `1` generic fallback

---

## `commitment get`

Read one on-chain commitment entry.

```bash
agcli commitment get \
  --netuid 1 \
  --hotkey-address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### Clap flags + types

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | Subnet UID |
| `--hotkey-address` | `String` (SS58) | yes | Queried account key |

### Runtime query mapping

- agcli call path:
  - `handle_commitment(Get)` -> `Client::get_commitment`
  - `src/chain/queries.rs::get_commitment`
- storage key:
  - `Commitments::CommitmentOf(netuid, account_id)`
- no dispatchable call is submitted for this command.

### Pallet/storage/events

- Storage read:
  - `CommitmentOf`
- Events emitted:
  - none (read-only command)

### Output JSON schemas (`--output json`)

When found:
```json
{
  "type": "object",
  "required": ["hotkey", "netuid", "block", "fields"],
  "properties": {
    "hotkey": { "type": "string" },
    "netuid": { "type": "integer", "minimum": 0, "maximum": 65535 },
    "block": { "type": "integer", "minimum": 0 },
    "fields": { "type": "array", "items": { "type": "string" } }
  },
  "additionalProperties": false
}
```

When missing:
```json
{
  "type": "object",
  "required": ["hotkey", "netuid", "found"],
  "properties": {
    "hotkey": { "type": "string" },
    "netuid": { "type": "integer", "minimum": 0, "maximum": 65535 },
    "found": { "type": "boolean", "const": false }
  },
  "additionalProperties": false
}
```

### Exit codes

- `0` success (includes "not found" JSON/text output)
- `12` validation (`invalid netuid`, invalid SS58)
- `10` network/RPC errors
- `15` timeout
- `1` generic fallback

---

## `commitment list`

List all commitment entries for a subnet.

```bash
agcli commitment list --netuid 1
```

### Clap flags + types

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--netuid` | `u16` | yes | Subnet UID |

### Runtime query mapping

- agcli call path:
  - `handle_commitment(List)` -> `Client::get_all_commitments`
  - `src/chain/queries.rs::get_all_commitments`
- storage iteration key:
  - `Commitments::CommitmentOf` prefix by `netuid`
- no dispatchable call is submitted for this command.

### Pallet/storage/events

- Storage read:
  - `CommitmentOf` (prefix iteration)
- Pallet helper relation:
  - Equivalent data domain to `pallet_commitments::Pallet::get_commitments(netuid)` (agcli reads storage directly, not this helper fn)
- Events emitted:
  - none (read-only command)

### Output JSON schema (`--output json`)

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": ["hotkey", "block", "fields"],
    "properties": {
      "hotkey": { "type": "string" },
      "block": { "type": "integer", "minimum": 0 },
      "fields": { "type": "array", "items": { "type": "string" } }
    },
    "additionalProperties": false
  }
}
```

### Exit codes

- `0` success
- `12` validation (`invalid netuid`)
- `10` network/RPC errors
- `15` timeout
- `1` generic fallback

---

## In-scope pallet functions not surfaced as `agcli commitment` subcommands

From `subtensor/pallets/commitments/src/lib.rs`:
- `set_max_space(origin, new_limit)` (root-only dispatchable)
- `reveal_timelocked_commitments()` (hook/helper, no direct CLI command)
- `purge_netuid(netuid)` (internal helper, no dispatchable exposed by this pallet call section)

## Related commands

- `agcli serve axon` (legacy endpoint advertisement path)
- `agcli identity set-subnet` (subnet identity metadata, not commitment records)
