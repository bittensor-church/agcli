# diff — Historical state comparison

Compare read-only chain snapshots at **two block heights** (balance, subnet metrics, network totals, or metagraph deltas). Pair with **`agcli block`** to pick heights, then drill in with **`diff`**.

**Discoverability:** `agcli diff --help`; `agcli explain --topic diff` (alias topic **`compare`**) lists examples and archive notes.

**Flags:** Every subcommand uses **`--block1`** and **`--block2`** (both required, `u32`). There is no `--from-block` / `--to-block` spelling.

For heights outside your node's state window, use **`--network archive`** (or an archive **`--endpoint`**) — same pruning semantics as **`agcli balance --at-block`** (see errors below).

## Subcommands

| Subcommand   | Required flags              | Optional flags   | --output json |
|-------------|----------------------------|-----------------|---------------|
| `portfolio` | `--block1`, `--block2`      | `--address`     | yes           |
| `subnet`    | `--netuid`, `--block1`, `--block2` | —         | yes           |
| `network`   | `--block1`, `--block2`      | —               | yes           |
| `metagraph` | `--netuid`, `--block1`, `--block2` | —         | yes           |

---

### diff portfolio

Compare coldkey **free balance** and **aggregate stake positions** between two blocks.

Default address: wallet coldkey (`~/.bittensor/wallets/<name>/coldkeypub.txt`). Override with **`--address`**.

```bash
agcli diff portfolio --block1 100 --block2 200
agcli diff portfolio --block1 100 --block2 200 --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
agcli diff portfolio --block1 100 --block2 200 --output json
agcli --network archive diff portfolio --block1 1000000 --block2 2000000 --address 5GrwvaEF5...
```

**Flags**

| Flag        | Type            | Required | Description                              |
|-------------|-----------------|----------|------------------------------------------|
| `--block1`  | `u32`           | yes      | First (earlier) block number             |
| `--block2`  | `u32`           | yes      | Second (later) block number              |
| `--address` | `String` (SS58) | no       | SS58 coldkey address; defaults to wallet |

**Read path** (`src/cli/block_cmds.rs` `handle_diff::Portfolio`):

1. `try_join!(client.get_block_hash(block1), client.get_block_hash(block2))` — RPC `chain_getBlockHash`
2. `try_join!(get_balance_at_block(&addr, hash1), get_stake_for_coldkey_at_block(&addr, hash1), get_balance_at_block(&addr, hash2), get_stake_for_coldkey_at_block(&addr, hash2))`

**Pallets / storage keys**

| Data | Pallet | Storage / API |
|------|--------|---------------|
| Free balance | `Balances` (frame) | `System.Account.data.free` (`u128` rao) |
| Stake info   | `SubtensorModule` | Runtime API `StakeInfoRuntimeApi::get_stake_info_for_coldkey` |

**JSON schema** (`--output json`)

```json
{
  "address":         "<SS58>",
  "block1":          <u32>,
  "block2":          <u32>,
  "balance_tao":     [<f64 at block1>, <f64 at block2>],
  "balance_diff_tao": <f64>,
  "total_stake_tao": [<f64 at block1>, <f64 at block2>],
  "stake_diff_tao":  <f64>,
  "total_tao":       [<f64 at block1>, <f64 at block2>],
  "total_diff_tao":  <f64>,
  "stakes_block1":   <usize>,
  "stakes_block2":   <usize>
}
```

> **Note:** All `tao` fields are `f64` (rao ÷ 1_000_000_000). Diff fields are computed as `block2_value - block1_value`. There is no `stakes_diff` field — compute it as `stakes_block2 - stakes_block1`.

**Exit codes**

| Condition | Code | Notes |
|-----------|------|-------|
| Success | 0 | |
| Missing `--address`, no wallet / empty resolved address | 1 (GENERIC) | Message: `"No address provided and no wallet found. Use --address <SS58>."` — does not match VALIDATION patterns in `src/error.rs` |
| Invalid SS58 (address format) | 12 (VALIDATION) | Message contains `"invalid ss58"` |
| Missing `--block1` or `--block2` | 2 (clap) | clap rejects before handler runs |
| Block not found (`chain_getBlockHash` returns `None`) | 1 (GENERIC) | `"Block N not found"` — not covered by VALIDATION classify |
| Pruned / unknown block state | 1 (GENERIC) with archive hint | Message includes archive node suggestion |
| Network error | 10 (NETWORK) | |
| Timeout | 15 (TIMEOUT) | |

**On-chain events:** None. All reads are read-only storage / runtime-API queries.

**E2E coverage:** `tests/e2e_test.rs` Phase 20 `test_diff_queries` — `diff_portfolio_preflight`.

---

### diff subnet

Compare **dynamic subnet** fields (TAO in pool, price, emission) between two blocks.

```bash
agcli diff subnet --netuid 1 --block1 100 --block2 200
agcli diff subnet --netuid 1 --block1 100 --block2 200 --output json
agcli --network archive diff subnet --netuid 1 --block1 1000000 --block2 2000000
```

**Flags**

| Flag       | Type  | Required | Description          |
|------------|-------|----------|----------------------|
| `--netuid` | `u16` | yes      | Subnet UID (0–65535) |
| `--block1` | `u32` | yes      | First block number   |
| `--block2` | `u32` | yes      | Second block number  |

**Read path** (`handle_diff::Subnet`):

1. `try_join!(get_block_hash(block1), get_block_hash(block2))`
2. `try_join!(get_dynamic_info_at_block(netuid, hash1), get_dynamic_info_at_block(netuid, hash2))`

If either call returns `None` → `anyhow::bail!("Subnet {netuid} not found at block {N}")`.

**Pallets / storage keys**

| Data | Pallet | Storage / API |
|------|--------|---------------|
| Dynamic subnet info | `SubtensorModule` | Runtime API `SubnetInfoRuntimeApi::get_dynamic_info(netuid)` |

**JSON schema** (`--output json`)

```json
{
  "netuid":       <u16>,
  "name":         "<string from block2>",
  "block1":       <u32>,
  "block2":       <u32>,
  "tao_in":       [<f64 at block1>, <f64 at block2>],
  "tao_in_diff":  <f64>,
  "price":        [<f64 at block1>, <f64 at block2>],
  "price_diff":   <f64>,
  "emission":     [<u64 at block1>, <u64 at block2>],
  "emission_diff": <i128>
}
```

> **Known drift:** The human table (`--output text`) also displays `Tempo` and `Owner HK` for both blocks. These two fields are **absent from the JSON output**. Machine consumers cannot retrieve `tempo` or `owner_hotkey` diffs via `--output json`. See Findings.

**Exit codes**

| Condition | Code | Notes |
|-----------|------|-------|
| Success | 0 | |
| Subnet not found at either block | 12 (VALIDATION) | `classify()` matches `"subnet" && "not found"` |
| Missing `--netuid` | 2 (clap) | |
| Missing `--block1` or `--block2` | 2 (clap) | |
| Block not found | 1 (GENERIC) | |
| Pruned state | 1 (GENERIC) with archive hint | |
| Network error | 10 (NETWORK) | |
| Timeout | 15 (TIMEOUT) | |

**On-chain events:** None.

**E2E coverage:** `tests/e2e_test.rs` Phase 20 — `diff_subnet_preflight`.

---

### diff network

Compare **total issuance**, **total stake**, implied **staking ratio**, and **subnet count** between two blocks.

```bash
agcli diff network --block1 100 --block2 200
agcli diff network --block1 100 --block2 200 --output json
agcli --network archive diff network --block1 1000000 --block2 2000000
```

**Flags**

| Flag       | Type  | Required | Description         |
|------------|-------|----------|---------------------|
| `--block1` | `u32` | yes      | First block number  |
| `--block2` | `u32` | yes      | Second block number |

**Read path** (`handle_diff::Network`):

1. `try_join!(get_block_hash(block1), get_block_hash(block2))`
2. `try_join!(get_total_issuance_at_block(hash1), get_total_stake_at_block(hash1), get_all_subnets_at_block(hash1), get_total_issuance_at_block(hash2), get_total_stake_at_block(hash2), get_all_subnets_at_block(hash2))`

Staking ratio is computed locally: `stake.tao() / issuance.tao() * 100.0` (returns 0.0 if issuance is zero).

**Pallets / storage keys**

| Data | Pallet | Storage / API |
|------|--------|---------------|
| Total issuance | `Balances` (frame) | `Balances.TotalIssuance` (`u128` rao) |
| Total stake | `SubtensorModule` | `SubtensorModule.TotalStake` (`u64` rao) |
| Subnet list | `SubtensorModule` | Runtime API `SubnetInfoRuntimeApi::get_subnets_info()` |

**JSON schema** (`--output json`)

```json
{
  "block1":             <u32>,
  "block2":             <u32>,
  "total_issuance_tao": [<f64 at block1>, <f64 at block2>],
  "total_stake_tao":    [<f64 at block1>, <f64 at block2>],
  "staking_ratio_pct":  [<f64 at block1>, <f64 at block2>],
  "subnet_count":       [<usize at block1>, <usize at block2>]
}
```

> **Known drift:** Unlike `diff portfolio` and `diff subnet`, there are **no `*_diff` scalar fields** in the JSON output. Consumers must compute deltas themselves (e.g., `subnet_count[1] - subnet_count[0]`). The human table does print signed deltas. See Findings.

**Exit codes**

| Condition | Code |
|-----------|------|
| Success | 0 |
| Missing `--block1` or `--block2` | 2 (clap) |
| Block not found | 1 (GENERIC) |
| Pruned state | 1 (GENERIC) with archive hint |
| Network error | 10 (NETWORK) |
| Timeout | 15 (TIMEOUT) |

**On-chain events:** None.

**E2E coverage:** `tests/e2e_test.rs` Phase 20 — `diff_network_preflight`.

---

### diff metagraph

Load **lite metagraph** at both heights and print neurons that changed (stake / emission / incentive deltas above thresholds, hotkey replacement, or new UIDs).

```bash
agcli diff metagraph --netuid 1 --block1 100 --block2 200
agcli diff metagraph --netuid 1 --block1 100 --block2 200 --output json
agcli --network archive diff metagraph --netuid 1 --block1 1000000 --block2 2000000
```

**Flags**

| Flag       | Type  | Required | Description          |
|------------|-------|----------|----------------------|
| `--netuid` | `u16` | yes      | Subnet UID (0–65535) |
| `--block1` | `u32` | yes      | First block number   |
| `--block2` | `u32` | yes      | Second block number  |

**Read path** (`handle_diff::Metagraph`):

1. `try_join!(get_block_hash(block1), get_block_hash(block2))`
2. `try_join!(get_neurons_lite_at_block(netuid, hash1), get_neurons_lite_at_block(netuid, hash2))`

Diff is computed locally: builds a `HashMap<uid, NeuronInfoLite>` from block1 neurons, then iterates block2 neurons. A change record is emitted when:

- `stake_diff.abs() > 0.001`
- `emission_diff.abs() > 0.0001`
- `incentive_diff.abs() > 0.0001`
- `n2.hotkey != n1.hotkey` (hotkey replacement → `"change": "replaced"`)

UIDs present in block2 but absent in block1 map to `"change": "new"`.

> **Known drift:** UIDs present in block1 but **absent** in block2 are **not reported** — the removed-neuron case is silently dropped. See Findings.

**Pallets / storage keys**

| Data | Pallet | Storage / API |
|------|--------|---------------|
| Lite neuron info | `SubtensorModule` | Runtime API `NeuronInfoRuntimeApi::get_neurons_lite(netuid)` |

**JSON schema** (`--output json`)

```json
{
  "netuid":         <u16>,
  "block1":         <u32>,
  "block2":         <u32>,
  "neurons_block1": <usize>,
  "neurons_block2": <usize>,
  "changed":        <usize>,
  "diffs": [
    {
      "uid":            <u16>,
      "hotkey":         "<SS58>",
      "change":         "changed" | "replaced" | "new",
      "stake_diff":     <f64>,
      "emission_diff":  <f64>,
      "incentive_diff": <f64>
    }
  ]
}
```

Empty metagraphs are valid — returns `"changed": 0` with an empty `"diffs"` array (exit 0).

**Exit codes**

| Condition | Code |
|-----------|------|
| Success (including empty metagraph) | 0 |
| Missing `--netuid` | 2 (clap) |
| Missing `--block1` or `--block2` | 2 (clap) |
| Block not found | 1 (GENERIC) |
| Pruned state | 1 (GENERIC) with archive hint |
| Network error | 10 (NETWORK) |
| Timeout | 15 (TIMEOUT) |

**On-chain events:** None.

**E2E coverage:** `tests/e2e_test.rs` Phase 20 — `diff_metagraph_preflight`.

---

## Exit code reference

| Code | Constant | Meaning |
|------|----------|---------|
| 0  | — | Success |
| 1  | `GENERIC` | Unclassified runtime error |
| 2  | (clap) | Argument parse error (missing required flag, type mismatch, unknown flag) |
| 10 | `NETWORK` | RPC transport / connection failure |
| 12 | `VALIDATION` | Invalid user input (e.g., subnet not found, invalid SS58) |
| 15 | `TIMEOUT` | RPC timed out |

Source: `src/error.rs` — `pub mod exit_code` + `pub fn classify`.

---

## Source code

**Handler:** `src/cli/block_cmds.rs` — `pub(super) async fn handle_diff()`:
- `Portfolio` ~L176
- `Subnet` ~L279
- `Network` ~L382
- `Metagraph` ~L465

**Query functions:** `src/chain/mod.rs` (`get_block_hash`, `get_balance_at_block`, `get_total_stake_at_block`, `get_total_issuance_at_block`) and `src/chain/queries.rs` (`get_stake_for_coldkey_at_block`, `get_all_subnets_at_block`, `get_dynamic_info_at_block`, `get_neurons_lite_at_block`).

**CLI definition:** `src/cli/mod.rs` — `pub enum DiffCommands` ~L2248.

**All diff commands are read-only** — no extrinsics, no events, no SCALE-encoded writes.

---

## Findings (audit)

1. **`diff subnet` JSON omits `tempo` and `owner_hotkey`** — the human table prints both fields for both blocks; the JSON object has neither. Machine consumers cannot retrieve these fields via `--output json`. Suggested fix: add `"tempo": [d1.tempo, d2.tempo]`, `"owner_hotkey": [d1.owner_hotkey, d2.owner_hotkey]`, and diff fields.

2. **`diff network` JSON has no diff fields** — `diff portfolio` and `diff subnet` include scalar `*_diff` fields alongside two-element arrays. `diff network` only returns the arrays (`[val_at_block1, val_at_block2]`), forcing consumers to compute deltas themselves. Suggested fix: add `total_issuance_diff_tao`, `total_stake_diff_tao`, `staking_ratio_diff_pct`, `subnet_count_diff`.

3. **`diff metagraph` does not track removed neurons** — the diff loop iterates `neurons2` and looks up UIDs in a map built from `neurons1`. UIDs present in block1 but absent in block2 are never emitted as `"change": "removed"`. A complete diff should iterate both directions.

4. **`diff portfolio` "no address" error exits GENERIC (1) not VALIDATION (12)** — the message `"No address provided and no wallet found."` does not match any VALIDATION pattern in `src/error.rs`'s `classify()`. From a scripting perspective this is a user-input error and should exit 12. The `docs/commands/diff.md` (original) correctly documents exit 1, but the behaviour differs from what a consistent exit-code taxonomy would suggest.

5. **`diff portfolio` float arithmetic for diffs** — `balance_diff_tao` is `bal2.tao() - bal1.tao()` (f64 subtraction after conversion). For sub-rao differences the result may not be exact. Computing `(rao2 - rao1) as f64 / 1e9` would be exact for values in the u64 range.

6. **`diff subnet` `emission_diff` type is `i128` in JSON** — the actual serde value is `d2.emission as i128 - d1.emission as i128`. `emission` fields are `u64`, so the diff can underflow if not handled as signed. The cast is correct, but the JSON schema should note that `emission_diff` can be negative.

---

## Related commands

- `agcli block latest` / `block info` / `block range` — Pick safe block heights before diffing.
- `agcli subnet metagraph --diff` — Live head vs pinned block (different UX than `diff metagraph`).
- `agcli subnet cache-diff` — Compare **cached** metagraph files, not arbitrary on-chain heights.
- `agcli explain --topic diff` — Conceptual overview and examples.
