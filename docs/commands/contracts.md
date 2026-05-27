# contracts — WASM Smart Contract Operations

Deploy and interact with WASM smart contracts on the Bittensor chain via
`pallet-contracts` (pallet index 29 in the subtensor runtime).

## Prerequisites

- A funded coldkey wallet (required for every write operation).
- The contract must be compiled to WASM first (e.g. via `cargo contract build`).
- Maximum upload size: 16 MB per WASM binary (enforced client-side).

---

## Subcommands

### `contracts upload`

Upload WASM contract code to the chain (does **not** instantiate).

```bash
agcli contracts upload \
  --code /path/to/contract.wasm \
  [--storage-deposit-limit <RAO>]
```

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--code` | `String` (path) | yes | — | Path to the compiled `.wasm` file |
| `--storage-deposit-limit` | `u128` (RAO) | no | `None` (unlimited) | Maximum storage deposit to reserve |

**Pallet call:** `Contracts::upload_code`
- SCALE args: `code: Vec<u8>`, `storage_deposit_limit: Option<u128>`, `determinism: Determinism`
- The `determinism` argument is hard-coded to `Unrestricted` in agcli.

**Output (stdout):**
```
Uploading contract code (<N> bytes)
Contract code uploaded. Tx: 0x<tx-hash>
```

> **Note:** The WASM code hash (needed for `instantiate`) is **not** printed.
> Retrieve it from the `Contracts::CodeStored` event in the block containing
> the returned tx hash, or query chain storage after upload.

**On-chain event emitted:**
- `Contracts::CodeStored { code_hash: H256, deposit_held: Balance, uploader: AccountId }`

**Exit codes:**
| Code | Meaning |
|---|---|
| `0` | Success |
| `12` (VALIDATION) | WASM file not found, empty, too large (>16 MB), or invalid magic bytes |
| `11` (AUTH) | Wallet locked or keypair error |
| `10` (NETWORK) | Cannot connect to the node |
| `13` (CHAIN) | Extrinsic rejected (e.g. duplicate code hash, insufficient balance) |
| `14` (IO) | File read error |

---

### `contracts instantiate`

Create a contract instance from an already-uploaded code hash.

```bash
agcli contracts instantiate \
  --code-hash 0x<32-byte-hex> \
  [--value <RAO>] \
  [--data <hex>] \
  [--salt <hex>] \
  [--gas-ref-time <u64>] \
  [--gas-proof-size <u64>] \
  [--storage-deposit-limit <RAO>]
```

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--code-hash` | `String` (0x-prefixed hex, 32 bytes) | yes | — | Hash returned by `upload` (from `CodeStored` event) |
| `--value` | `u128` (RAO) | no | `0` | Balance to transfer to the contract on creation |
| `--data` | `String` (hex) | no | `0x` | Constructor selector + encoded arguments |
| `--salt` | `String` (hex) | no | `0x` | Unique salt for deterministic address derivation |
| `--gas-ref-time` | `u64` | no | `10000000000` | Computation gas limit (ref_time component of Weight) |
| `--gas-proof-size` | `u64` | no | `1048576` | PoV size gas limit (proof_size component of Weight) |
| `--storage-deposit-limit` | `u128` (RAO) | no | `None` (unlimited) | Maximum storage deposit to reserve |

**Pallet call:** `Contracts::instantiate`
- SCALE args: `value: u128`, `gas_limit: Weight { ref_time, proof_size }`,
  `storage_deposit_limit: Option<u128>`,
  `code: Code::Existing([u8; 32])`, `data: Vec<u8>`, `salt: Vec<u8>`
- The code hash is encoded as `Code::Existing(hash)` (enum variant), not a bare `[u8; 32]`.
- Gas limit is a SCALE struct `{ ref_time: u64, proof_size: u64 }` encoded as `Weight`.

**Output (stdout):**
```
Instantiating contract from code hash 0x<hash>
Contract instantiated. Tx: 0x<tx-hash>
```

> **Note:** The deployed contract's SS58 address is **not** printed.
> Retrieve it from the `Contracts::Instantiated` event in the block.

**On-chain event emitted:**
- `Contracts::Instantiated { deployer: AccountId, contract: AccountId }`

**Exit codes:** same table as `upload`.

---

### `contracts call`

Call a method on a deployed contract.

```bash
agcli contracts call \
  --contract <SS58-address> \
  --data <hex> \
  [--value <RAO>] \
  [--gas-ref-time <u64>] \
  [--gas-proof-size <u64>] \
  [--storage-deposit-limit <RAO>]
```

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--contract` | `String` (SS58) | yes | — | SS58 address of the deployed contract |
| `--data` | `String` (hex) | yes | — | Method selector + encoded arguments |
| `--value` | `u128` (RAO) | no | `0` | Balance to transfer to the contract on the call |
| `--gas-ref-time` | `u64` | no | `10000000000` | Computation gas limit |
| `--gas-proof-size` | `u64` | no | `1048576` | PoV size gas limit |
| `--storage-deposit-limit` | `u128` (RAO) | no | `None` (unlimited) | Maximum storage deposit to reserve |

**Pallet call:** `Contracts::call`
- SCALE args: `dest: AccountIdLookup::Id(AccountId32)`, `value: u128`,
  `gas_limit: Weight { ref_time, proof_size }`,
  `storage_deposit_limit: Option<u128>`, `data: Vec<u8>`
- `--contract` is validated as a valid SS58 address before encoding.

**Output (stdout):**
```
Calling contract <short-addr> (<N> bytes input)
Contract call submitted. Tx: 0x<tx-hash>
```

**On-chain event emitted:**
- `Contracts::Called { caller: Origin, contract: AccountId }`

**Exit codes:** same table as `upload`. Additionally, exit code `12` is returned
for an invalid SS58 address.

---

### `contracts remove-code`

Remove previously uploaded contract code from the chain (reclaims storage deposit).
Only the original uploader can remove the code.

```bash
agcli contracts remove-code \
  --code-hash 0x<32-byte-hex>
```

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--code-hash` | `String` (0x-prefixed hex, 32 bytes) | yes | — | Hash of the code to remove |

**Pallet call:** `Contracts::remove_code`
- SCALE args: `code_hash: H256`

**Output (stdout):**
```
Removing contract code 0x<hash>
Contract code removed. Tx: 0x<tx-hash>
```

**On-chain event emitted:**
- `Contracts::CodeRemoved { code_hash: H256, deposit_released: Balance, remover: AccountId }`

**Exit codes:** same table as `upload`.

---

## Pallet Reference

| agcli subcommand | Pallet call | Pallet index |
|---|---|---|
| `upload` | `Contracts::upload_code` | 29 |
| `instantiate` | `Contracts::instantiate` | 29 |
| `call` | `Contracts::call` | 29 |
| `remove-code` | `Contracts::remove_code` | 29 |

**Pallet source:** `subtensor/runtime/src/lib.rs` (Config impl at ~line 1608),
upstream at `pallet-contracts` from `https://github.com/opentensor/polkadot-sdk`.

**Storage keys (pallet `Contracts`):**
| Storage item | Key | Description |
|---|---|---|
| `CodeInfoOf` | `Blake2_128Concat(code_hash)` | Metadata per uploaded code (owner, deposit, ref count) |
| `PristineCode` | `Blake2_128Concat(code_hash)` | Raw WASM bytes |
| `ContractInfoOf` | `Blake2_128Concat(account_id)` | Per-contract metadata (trie id, code hash, storage) |
| `DeletionQueue` | `Twox64Concat(index)` | Contracts queued for lazy deletion |

---

## Missing Surface (Audit Findings)

The following `pallet-contracts` dispatchables exist in the subtensor runtime
but have **no agcli subcommand**:

- `instantiate_with_code` — uploads **and** instantiates in a single extrinsic.
  Currently requires two separate `upload` + `instantiate` round-trips.
- `set_code` — replaces the code of an existing contract (migration / upgrade).
- `call` (read-only dry-run via RPC `contracts_call`) — agcli does not expose
  the runtime RPC for dry-running a call without submitting an extrinsic.

---

## Related Commands

- `agcli evm call` — EVM (Solidity) contract interaction
- `agcli preimage note` / `agcli scheduler schedule` — schedule deferred contract calls via on-chain scheduler
