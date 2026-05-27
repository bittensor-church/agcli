# block — Block Explorer

Query block headers, timestamps, and extrinsic counts from the chain.  All subcommands are **read-only** — no transaction is submitted and no wallet is required.

**Discoverability:** `agcli block --help`; `agcli explain --topic archive` lists `block latest` / `block info` / `block range` with archive context.

---

## Subcommands

### block latest

Show the latest block as seen by the connected node: height, hash, extrinsic count, and timestamp (when the `Timestamp::Now` inherent is present).

```bash
agcli block latest
agcli block latest --output json
```

**Flags:** none beyond global flags (`--network`, `--endpoint`, `--output`).

**Read path** (`handle_block` → `BlockCommands::Latest`):
1. `client.get_block_number()` → `u64` (best / non-finalized head via `blocks().at_latest()`)
2. `u64 → u32` narrowing (panics with exit 1 if chain height > `u32::MAX` — not expected in practice)
3. `client.get_block_hash(u32)` → `H256` via RPC `chain_getBlockHash`
4. `tokio::try_join!(get_block_extrinsic_count, get_block_timestamp)`

> **⚠ Finding:** The clap doc-comment says *"Show the latest **finalized** block"* but the handler calls `get_block_number()` which is documented as *"best / non-finalized"*.  The finalized-head equivalent is `get_finalized_block_number()` (uses RPC `chain_getFinalizedHead`).  On a live network the difference is typically 2–4 blocks.  See [Suggested follow-ups](#suggested-follow-ups).

**JSON output schema** (`--output json`):

```json
{
  "block_number": 4000000,
  "block_hash": "0xabcd…",
  "extrinsic_count": 7,
  "timestamp_ms": 1700000000000,
  "timestamp": "2023-11-14T22:13:20+00:00"
}
```

`timestamp_ms` and `timestamp` are omitted when the node does not expose `Timestamp::Now` at that block.

**Human-readable output:**

```
Latest Block: #4000000
  Hash:        0xabcd…
  Extrinsics:  7
  Timestamp:   2023-11-14 22:13:20 UTC
```

**Exit codes** (per `src/error.rs`):

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | Generic — including `u32` overflow guard or unexpected RPC response |
| 10   | Network — connection refused, DNS failure |
| 15   | Timeout |

**Storage / pallet reference:**

| What | Where |
|------|-------|
| Block hash lookup | RPC `chain_getBlockHash` (no storage key; served by the node's block DB) |
| Block extrinsics | `blocks().at(hash).extrinsics()` — parses the raw block body from the node |
| Timestamp | `Timestamp::Now` storage item — `pallet-timestamp` (standard Substrate pallet) |

**Events emitted:** none (read-only).

---

### block info

Show details for a specific block by number: number, hash, parent hash, state root, extrinsic count, and timestamp.

```bash
agcli block info --number 4000000
agcli block info --number 4000000 --output json
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--number` | `u32` | yes | Block number to inspect |

**Read path** (`handle_block` → `BlockCommands::Info`):
1. `client.get_block_hash(number)` → `H256`
2. `tokio::try_join!(get_block_header, get_block_extrinsic_count, get_block_timestamp)`

**JSON output schema:**

```json
{
  "block_number": 4000000,
  "block_hash": "0xabcd…",
  "parent_hash": "0xef01…",
  "state_root": "0x2345…",
  "extrinsic_count": 7,
  "timestamp_ms": 1700000000000,
  "timestamp": "2023-11-14T22:13:20+00:00"
}
```

`timestamp_ms` / `timestamp` omitted when absent.

> **⚠ Finding:** `get_block_header` doc comment lists `extrinsics_root` as a returned field but the actual return type is `(u32, H256, H256, H256)` — only number, hash, parent_hash, and state_root.  `extrinsics_root` is **not** exposed in either the query or the CLI output.  See [Suggested follow-ups](#suggested-follow-ups).

> **⚠ Finding:** Hash fields are serialized with `format!("{:?}", hash)` (Rust Debug format) rather than a dedicated hex formatter.  The output is a `0x`-prefixed lowercase hex string today, but `Debug` format is not a stability guarantee across subxt versions.

**Human-readable output:**

```
Block #4000000
  Hash:        0xabcd…
  Parent:      0xef01…
  State root:  0x2345…
  Extrinsics:  7
  Timestamp:   2023-11-14 22:13:20 UTC
```

**Exit codes:**

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | Generic — "Block N not found" when the block doesn't exist or has been pruned |
| 2    | Clap validation — `--number` missing or not a valid `u32` |
| 10   | Network |
| 15   | Timeout |

> **⚠ Finding:** "Block N not found" (returned when `chain_getBlockHash` returns `None`) is classified as exit code **1** (GENERIC) by `src/error.rs`.  It should map to **12** (VALIDATION — bad user input) or a dedicated code.  The pruned-block hint in `annotate_at_block_error` only fires for *storage* reads, not for the hash-lookup path.

**Storage / pallet reference:** same as `block latest`.

**Events emitted:** none.

---

### block range

Summarize a range of consecutive blocks (max 1000).  Returns one row per block with height, hash, timestamp, and extrinsic count.

```bash
agcli block range --from 3999900 --to 4000000
agcli block range --from 3999900 --to 4000000 --output json
agcli block range --from 3999900 --to 4000000 --output csv
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--from` | `u32` | yes | First block (inclusive) |
| `--to`   | `u32` | yes | Last block (inclusive) |

**Constraints enforced in handler (exit 1 if violated):**

- `--from` ≤ `--to`
- span ≤ 1000 blocks (`(to as u64 - from as u64 + 1) ≤ 1000`)

> **Note:** The count arithmetic was widened to `u64` to avoid a wrap-around bug when `from = 0, to = u32::MAX`.

**Read path** (`handle_block` → `BlockCommands::Range`):
1. Local guard checks (from ≤ to, span ≤ 1000)
2. `futures::future::try_join_all` over `get_block_hash` for each block (concurrent)
3. `futures::future::try_join_all` over `try_join!(get_block_extrinsic_count, get_block_timestamp)` per hash (concurrent)
4. `render_rows` — table / CSV / JSON based on `--output`

**JSON output schema** (array):

```json
[
  {
    "block": 3999900,
    "hash": "0xabcd…",
    "timestamp": "2023-11-14 22:13:20",
    "extrinsics": 7
  },
  …
]
```

> **⚠ Finding:** In JSON output the `timestamp` field is a locale-formatted string (`"%Y-%m-%d %H:%M:%S"`) rather than RFC 3339 / ISO 8601.  `block info` and `block latest` produce RFC 3339 in their JSON path.  The inconsistency makes machine parsing unreliable.

> **⚠ Finding:** When `get_block_timestamp` returns `None` for a block, the `timestamp` field is an empty string (`""`), not `null`.  Consumers cannot distinguish "no timestamp" from a parse failure.

**Exit codes:**

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | Generic — range validation failure or RPC error |
| 2    | Clap — `--from` or `--to` missing / not `u32` |
| 10   | Network |
| 15   | Timeout |

**Storage / pallet reference:** same as `block latest`.

**Events emitted:** none.

---

## Source Code

| File | Purpose |
|------|---------|
| `src/cli/mod.rs` — `BlockCommands` | clap enum (Info, Latest, Range) |
| `src/cli/block_cmds.rs` — `handle_block` | Dispatch and output formatting |
| `src/chain/mod.rs` — `get_block_hash`, `get_block_number`, `get_finalized_block_number` | RPC helpers |
| `src/chain/queries.rs` — `get_block_header`, `get_block_extrinsic_count`, `get_block_timestamp` | Header + extrinsic + timestamp reads |

---

## diff — Block-diff Subcommands

`agcli diff` is a separate top-level command whose handler lives in the same source file (`src/cli/block_cmds.rs`).  It compares chain state between two block numbers.

### diff portfolio

Compare free balance and stake positions for an address between two blocks.

```bash
agcli diff portfolio --address <SS58> --block1 3999900 --block2 4000000
agcli diff portfolio --block1 3999900 --block2 4000000   # uses wallet coldkey
agcli diff portfolio --address <SS58> --block1 3999900 --block2 4000000 --output json
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--address` | SS58 string | no | Coldkey address (defaults to wallet coldkey) |
| `--block1` | `u32` | yes | Earlier block |
| `--block2` | `u32` | yes | Later block |

**Read path:** `get_block_hash` × 2 → `try_join!(get_balance_at_block, get_stake_for_coldkey_at_block)` × 2.

**JSON schema:**

```json
{
  "address": "5F…",
  "block1": 3999900,
  "block2": 4000000,
  "balance_tao": [1.0, 1.1],
  "balance_diff_tao": 0.1,
  "total_stake_tao": [2.0, 2.5],
  "stake_diff_tao": 0.5,
  "total_tao": [3.0, 3.6],
  "total_diff_tao": 0.6,
  "stakes_block1": 3,
  "stakes_block2": 4
}
```

**Exit codes:** 0 success, 1 generic, 10 network, 11 auth (wallet missing), 15 timeout.

**Storage:** `System::Account` (balance), `SubtensorModule::StakingHotkeys` + `SubtensorModule::Stake` (stakes).

**Events emitted:** none.

---

### diff subnet

Compare subnet dynamic info (TAO in, price, emission, tempo) between two blocks.

```bash
agcli diff subnet --netuid 1 --block1 3999900 --block2 4000000
agcli diff subnet --netuid 1 --block1 3999900 --block2 4000000 --output json
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | yes | Subnet UID |
| `--block1` | `u32` | yes | Earlier block |
| `--block2` | `u32` | yes | Later block |

**Exit codes:** 0, 1 (subnet not found at block), 10, 15.

**Storage:** `SubtensorModule` dynamic subnet info.

**Events emitted:** none.

---

### diff network

Compare network-wide totals (issuance, total stake, staking ratio, subnet count) between two blocks.

```bash
agcli diff network --block1 3999900 --block2 4000000
agcli diff network --block1 3999900 --block2 4000000 --output json
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--block1` | `u32` | yes | Earlier block |
| `--block2` | `u32` | yes | Later block |

**JSON schema:**

```json
{
  "block1": 3999900,
  "block2": 4000000,
  "total_issuance_tao": [10000000.0, 10000001.0],
  "total_stake_tao": [5000000.0, 5000001.5],
  "staking_ratio_pct": [50.0, 50.0],
  "subnet_count": [32, 32]
}
```

**Exit codes:** 0, 1, 10, 15.

**Storage:** `Balances::TotalIssuance`, `SubtensorModule::TotalStake`, dynamic subnet list.

**Events emitted:** none.

---

### diff metagraph

Compare per-neuron state (stake, emission, incentive, hotkey replacements) between two blocks for a subnet.  Only neurons with changes above the noise floor are shown.

```bash
agcli diff metagraph --netuid 1 --block1 3999900 --block2 4000000
agcli diff metagraph --netuid 1 --block1 3999900 --block2 4000000 --output json
```

**Flags:**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--netuid` | `u16` | yes | Subnet UID |
| `--block1` | `u32` | yes | Earlier block |
| `--block2` | `u32` | yes | Later block |

**Noise floor thresholds (hard-coded in source):**

| Field | Min change to appear |
|-------|---------------------|
| stake | > 0.001 τ |
| emission | > 0.0001 |
| incentive | > 0.0001 |

**JSON schema:**

```json
{
  "netuid": 1,
  "block1": 3999900,
  "block2": 4000000,
  "neurons_block1": 256,
  "neurons_block2": 256,
  "changed": 12,
  "diffs": [
    {
      "uid": 42,
      "hotkey": "5F…",
      "change": "changed",
      "stake_diff": 0.15,
      "emission_diff": 0.002,
      "incentive_diff": 0.001
    }
  ]
}
```

`change` is one of `"changed"`, `"replaced"` (hotkey swap), or `"new"` (UID appeared in block2).

**Exit codes:** 0, 1, 10, 15.

**Storage:** `SubtensorModule::NeuronsLite` (via `get_neurons_lite_at_block`).

**Events emitted:** none.

---

## Coverage Table

| Subcommand | Clap variant | Handler | Documented | Notes |
|------------|-------------|---------|------------|-------|
| `block info` | `BlockCommands::Info` | `handle_block` | ✅ | `extrinsics_root` missing from output |
| `block latest` | `BlockCommands::Latest` | `handle_block` | ✅ | Uses non-finalized head; doc says finalized |
| `block range` | `BlockCommands::Range` | `handle_block` | ✅ | Timestamp JSON is locale string, not RFC 3339 |
| `diff portfolio` | `DiffCommands::Portfolio` | `handle_diff` | ✅ | |
| `diff subnet` | `DiffCommands::Subnet` | `handle_diff` | ✅ | |
| `diff network` | `DiffCommands::Network` | `handle_diff` | ✅ | |
| `diff metagraph` | `DiffCommands::Metagraph` | `handle_diff` | ✅ | |

All `block` and `diff` subcommands are covered.  No dispatchables (write operations) exist in this group — it is entirely read-only.

---

## Related Commands

- `agcli subscribe blocks` — watch finalized blocks in real time
- `agcli --network archive block info --number N` — query historical blocks on an archive node
- `agcli view network` — current network stats without block-diff overhead

---

## Suggested follow-ups

1. **`block latest` should call `get_finalized_block_number()`** (uses `chain_getFinalizedHead`) instead of `get_block_number()` (best head) to match its doc-comment.
2. **`get_block_header` should return `extrinsics_root`** — add a fifth field to the tuple and surface it in `block info` JSON (`"extrinsics_root": "0x…"`).  The doc comment already lists it.
3. **Consistent timestamp serialization in `block range`** — use `dt.to_rfc3339()` instead of `dt.format("%Y-%m-%d %H:%M:%S")` in the `BlockRow::timestamp` field so JSON output matches `block info`/`block latest`.
4. **Missing timestamp should serialize as `null`** not `""` in `block range` JSON rows.
5. **Hash fields should use `Display` not `Debug`** — replace `format!("{:?}", hash)` with `format!("{}", hash)` or `hex::encode(hash.0)` for stable output.
6. **"Block N not found" should map to exit code 12 (VALIDATION)** in `src/error.rs` so scripts can distinguish bad user input from transient network errors.
