# evm — Ethereum Virtual Machine Operations

EVM write operations exposed by `agcli evm`.

## Command surface audited

`EvmCommands` currently exposes:

1. `evm call`
2. `evm withdraw`

`Client` also has `evm_create` and `evm_create2` helpers in `src/chain/extrinsics.rs`, but there is no CLI surface for them under `agcli evm`.

---

## evm call

Issue an `EVM::call` dispatch.

```bash
agcli evm call \
  --source 0x1111111111111111111111111111111111111111 \
  --target 0x2222222222222222222222222222222222222222 \
  [--input 0x] \
  [--value 0x0000000000000000000000000000000000000000000000000000000000000000] \
  [--gas-limit 21000] \
  [--max-fee-per-gas 0x0000000000000000000000000000000000000000000000000000000000000001]
```

### Clap flags and types

| Flag | Type | Required | Default |
|---|---|---:|---|
| `--source` | `String` (hex EVM address, 20 bytes) | yes | - |
| `--target` | `String` (hex EVM address, 20 bytes) | yes | - |
| `--input` | `String` (hex calldata) | no | `0x` |
| `--value` | `String` (hex, 32-byte U256) | no | `0x00..00` (32-byte zero) |
| `--gas-limit` | `u64` | no | `21000` |
| `--max-fee-per-gas` | `String` (hex, 32-byte U256) | no | `0x00..01` |

### Handler and dispatch path

- `src/cli/network_cmds.rs::handle_evm` (`EvmCommands::Call`)
- `src/chain/extrinsics.rs::Client::evm_call`
- Dynamic subxt call: `tx("EVM", "call", ...)`

### SCALE encoding map (CLI -> subxt dynamic -> pallet type)

| Argument | agcli dynamic value | pallet-evm `call` expects |
|---|---|---|
| `source` | `Value::from_bytes([u8;20])` | `H160` |
| `target` | `Value::from_bytes([u8;20])` | `H160` |
| `input` | `Value::from_bytes(Vec<u8>)` | `Vec<u8>` |
| `value` | `Value::from_bytes([u8;32])` | `U256` |
| `gas_limit` | `Value::u128(gas_limit as u128)` | `u64` |
| `max_fee_per_gas` | `Value::from_bytes([u8;32])` | `U256` |
| `max_priority_fee_per_gas` | `Option<bytes32>` | `Option<U256>` |
| `nonce` | `Option<bytes32>` | `Option<U256>` |
| `access_list` | empty composite | `Vec<(H160, Vec<H256>)>` |
| `authorization_list` | **not supplied by agcli** | `AuthorizationList` |

### Exit codes (`src/error.rs`)

| Code | Meaning | Typical `evm call` triggers |
|---:|---|---|
| `0` | success | extrinsic submitted/finalized |
| `10` | network | websocket/connectivity failures |
| `11` | auth | wallet unlock/coldkey issues |
| `12` | validation | invalid EVM address, bad hex, invalid gas limit |
| `13` | chain | dispatch/runtime rejection (nonce, fee, gas, balance, origin) |
| `14` | I/O | wallet/key file access errors |
| `15` | timeout | RPC or finalization timeout |
| `1` | generic | uncategorized failure |

### Output JSON schema

Current implementation prints plain text for all output modes (including `--output json`).

Expected machine-friendly schema (not yet emitted by `handle_evm`):

```json
{
  "type": "object",
  "required": ["operation", "tx_hash", "source", "target", "gas_limit"],
  "properties": {
    "operation": { "const": "evm.call" },
    "tx_hash": { "type": "string" },
    "source": { "type": "string" },
    "target": { "type": "string" },
    "input_hex": { "type": "string" },
    "value_hex": { "type": "string" },
    "gas_limit": { "type": "integer", "minimum": 0 },
    "max_fee_per_gas_hex": { "type": "string" }
  }
}
```

### Pallet reference and storage keys

- Dispatchable: `EVM::call` (`pallet-evm`, call index `1`)
- Storage used by this pallet during execution:
  - `EVM::AccountCodes`
  - `EVM::AccountCodesMetadata`
  - `EVM::AccountStorages`

### On-chain events emitted

From `pallet-evm`:

- `EVM::Executed { address }` on success
- `EVM::ExecutedFailed { address }` on revert/failure
- `EVM::Log { log }` for emitted contract logs

---

## evm withdraw

Issue an `EVM::withdraw` dispatch.

```bash
agcli evm withdraw \
  --address 0x1111111111111111111111111111111111111111 \
  --amount 1000000000
```

### Clap flags and types

| Flag | Type | Required | Default |
|---|---|---:|---|
| `--address` | `String` (hex EVM address, 20 bytes) | yes | - |
| `--amount` | `u128` (RAO) | yes | - |

### Handler and dispatch path

- `src/cli/network_cmds.rs::handle_evm` (`EvmCommands::Withdraw`)
- `src/chain/extrinsics.rs::Client::evm_withdraw`
- Dynamic subxt call: `tx("EVM", "withdraw", ...)`

### SCALE encoding map (CLI -> subxt dynamic -> pallet type)

| Argument | agcli dynamic value | pallet-evm `withdraw` expects |
|---|---|---|
| `address` | `Value::from_bytes([u8;20])` | `H160` |
| `amount` | `Value::u128(amount)` | `BalanceOf<T>` |

### Exit codes (`src/error.rs`)

Same exit code categories as `evm call`:
`0`, `10`, `11`, `12`, `13`, `14`, `15`, and fallback `1`.

### Output JSON schema

Current implementation prints plain text for all output modes (including `--output json`).

Expected machine-friendly schema (not yet emitted by `handle_evm`):

```json
{
  "type": "object",
  "required": ["operation", "tx_hash", "address", "amount_rao"],
  "properties": {
    "operation": { "const": "evm.withdraw" },
    "tx_hash": { "type": "string" },
    "address": { "type": "string" },
    "amount_rao": { "type": "integer", "minimum": 0 }
  }
}
```

### Pallet reference and storage keys

- Dispatchable: `EVM::withdraw` (`pallet-evm`, call index `0`)
- No direct `pallet-evm` event or dedicated storage mutation; this call transfers balance from the address-mapped account through the runtime currency implementation.

### On-chain events emitted

- `pallet-evm` emits no dedicated withdraw event in this dispatchable.
- In Subtensor runtime configurations using balances as currency, balance transfer events may be emitted by the balances pallet.

---

## Coverage note: missing EVM dispatchables on CLI

`pallet-evm` includes:

- `EVM::call`
- `EVM::withdraw`
- `EVM::create`
- `EVM::create2`

`agcli evm` currently exposes only `call` and `withdraw`.

## Related commands

- `agcli contracts call` for WASM contracts (`pallet-contracts`)
