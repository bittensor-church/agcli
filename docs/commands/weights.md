# weights — Weight Setting Operations

Validators set weights to score miners on a subnet. Weights determine how emissions are distributed. Supports direct set, two-phase commit-reveal, timelocked commit (drand), mechanism-specific weights, and an atomic commit-reveal workflow.

## From a binary-only install

- **`agcli explain --topic weights`** — built-in cheat sheet (aliases: `settingweights`, `setweights`).
- **`agcli explain --topic weights --full`** — prints this file when `docs/commands/` is next to the binary or when run from the repo.
- **`agcli weights --help`** — subcommand list and flags.

## Subcommands

### weights show

Read-only: list validators on a subnet who have set weights and their targets. Optionally filter to one hotkey. **No wallet required** — only RPC reads.

```bash
agcli weights show --netuid <U16> [--hotkey-address <SS58>] [--limit <N>] [--output json]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--hotkey-address` | SS58 string | no | — | Filter to one validator's hotkey |
| `--limit` | usize | no | all | Cap output to N validators |

**Exit codes:**
- `0` — success
- `12` — validation: netuid=0, invalid SS58, invalid limit, subnet not found
- `10` — RPC error fetching neurons or weights (hyperparams RPC error → warn+continue)

**Pre-flight:** `validate_netuid` → optional `validate_ss58` → optional `validate_view_limit` → `require_subnet_exists_for_weights_cmd` (latest-head `get_subnet_hyperparams`; RPC failure here only warns and continues). Then one of: (a) `get_neurons_lite` + `get_weights_for_uid` for single-hotkey mode, (b) `get_all_weights` + `get_neurons_lite` for all-validators mode.

**JSON output (`--output json`):**

All-validators mode:
```json
{
  "netuid": 1,
  "validators_with_weights": 12,
  "entries": [
    {
      "uid": 0,
      "hotkey": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "num_weights": 256,
      "weights": [{"uid": 0, "weight": 100}, {"uid": 1, "weight": 200}]
    }
  ]
}
```

Single-hotkey mode:
```json
{
  "netuid": 1,
  "uid": 3,
  "hotkey": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "weights": [{"uid": 0, "weight": 100}]
}
```

**Hotkey not found:** `Hotkey … not found on SN…` — generic anyhow error, **exit 1** (not exit 12).

**Storage keys:** `SubtensorModule::Weights(netuid, uid)` (double-map), `SubtensorModule::NeuronsLite(netuid, uid)`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — no extrinsic; read-only storage query.

**Source:** `WeightCommands::Show` → `handle_weights_show` in `src/cli/weights_cmds.rs`.

---

### weights set

Directly set weights on a subnet. **Only valid when commit-reveal is disabled** on the subnet. If commit-reveal is enabled, use `weights commit-reveal` instead.

```bash
agcli weights set --netuid <U16> --weights <WEIGHTS> [--version-key <U64>] [--dry-run]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID (must be ≥1) |
| `--weights` | string | yes | — | Weight input (see Weight Format) |
| `--version-key` | u64 | no | `0` | Must match `weights_version` from `subnet hyperparams` |
| `--dry-run` (global) | bool flag | no | false | Print JSON pre-flight; no extrinsic submitted |

**Exit codes:**
- `0` — weights set successfully
- `12` — validation: netuid=0, empty weights, subnet not found (before wallet open)
- `11` — wallet/hotkey error (wrong password, missing keyfile)
- `13` — chain rejection: `NotEnoughStakeToSetWeights`, `SettingWeightsTooFast`, `CommitRevealEnabled`, `IncorrectWeightVersionKey`, `WeightVecLengthIsLow`, `WeightVecNotEqualSize`, `UidVecContainInvalidOne`
- `10` — RPC/network error

**Pre-flight:** `validate_netuid` → `validate_weight_input` → `resolve_weights` (parse) → `get_subnet_hyperparams` (subnet existence + CR flag + rate limit; **RPC error bails**) → wallet unlock → optional stake-weight hint → extrinsic submit.

**Note:** dry-run still opens the wallet so it can check stake-weight and show the `stake_sufficient` field.

**Dry-run JSON output:**
```json
{
  "dry_run": true,
  "netuid": 1,
  "num_weights": 3,
  "version_key": 0,
  "stake_sufficient": true,
  "commit_reveal_enabled": false,
  "weights_rate_limit_blocks": 100,
  "weights": [{"uid": 0, "weight": 100}, {"uid": 1, "weight": 200}]
}
```

**Live output:** Human text only (`--output json` is silently ignored for the live path — see Findings §5).

**On-chain extrinsic:** `SubtensorModule::set_weights(origin, netuid: u16, dests: Vec<u16>, weights: Vec<u16>, version_key: u64)`

**SCALE encoding:** `netuid` as `u16` (Compact), `dests`/`weights` as `Vec<u16>` (Compact-encoded per element), `version_key` as `u64`.

**Storage writes:** `SubtensorModule::Weights(netuid, hotkey_uid)`.

**Events emitted:** `WeightsSet { netuid, uid }` (from `macros/events.rs`).

**Pallet errors:** `NotEnoughStakeToSetWeights` · `SettingWeightsTooFast` · `CommitRevealEnabled` · `IncorrectWeightVersionKey` · `WeightVecLengthIsLow` · `WeightVecNotEqualSize` · `UidVecContainInvalidOne` · `InvalidUid` · `MaxWeightExceeded`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — `fn set_weights`.

**Source:** `WeightCommands::Set` in `src/cli/weights_cmds.rs`.

---

### weights commit

Commit a blake2b-256 hash of weights (phase 1 of commit-reveal). Save the printed salt for reveal.

```bash
agcli weights commit --netuid <U16> --weights <WEIGHTS> [--salt <STRING>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--weights` | string | yes | — | Weight input (see Weight Format) |
| `--salt` | string | no | random 32-char alphanumeric | Salt for the commit hash. Printed to stdout if generated. |

**Exit codes:** same as `weights set` except chain errors are `CommitRevealDisabled`, `CommittingWeightsTooFast`, `TooManyUnrevealedCommits`.

**Hash computation:** `blake2b-256(uids_le_bytes || values_le_bytes || salt_raw_bytes)` via `compute_weight_commit_hash` in `src/extrinsics/weights.rs`. The commit is **over raw bytes of the salt string**, not u16-encoded.

**Important:** If `--salt` is omitted, agcli prints the generated salt to stdout. You **must** save it — it is required for the matching `weights reveal`.

**Output:** Human text only (`--output json` is silently ignored — see Findings §5).

**On-chain extrinsic:** `SubtensorModule::commit_weights(origin, netuid: u16, commit: H256)`

> Note: the typed API dispatch name in the generated subxt code is `commit_weights`. The docs and pallet source may reference this as `commit_crv3_weights` internally — the wire-level dispatch name is `commit_weights` (see Findings §3).

**Storage writes:** `SubtensorModule::WeightCommits(netuid, hotkey)`.

**Events emitted:** `CRV3WeightsCommitted { account, netuid, commit }`.

**Pallet errors:** `CommitRevealDisabled` · `CommittingWeightsTooFast` · `TooManyUnrevealedCommits`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — `fn commit_crv3_weights`.

**Source:** `WeightCommands::Commit` in `src/cli/weights_cmds.rs`.

---

### weights reveal

Reveal previously committed weights (phase 2 of commit-reveal). Must match the exact weights and salt used in `weights commit`.

```bash
agcli weights reveal --netuid <U16> --weights <WEIGHTS> --salt <STRING> [--version-key <U64>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--weights` | string | yes | — | Same weight input as at commit time |
| `--salt` | string | yes | — | Same salt string used at commit time |
| `--version-key` | u64 | no | `0` | Version key for the reveal |

**Exit codes:** same as `weights commit` plus `NoWeightsCommitFound`, `InvalidRevealCommitHashNotMatch`, `ExpiredWeightCommit`, `RevealTooEarly`.

**Salt encoding for the extrinsic:** The salt string is split into **little-endian u16 pairs** (two UTF-8 bytes per u16; the last u16 is zero-padded in the high byte if the string has odd length). This is the encoding the pallet expects in `Vec<u16>`. The hash was computed from raw bytes — the on-chain pallet reconstructs the hash from these u16 pairs by doing the reverse LE decode, which is consistent.

**Output:** Human text only (`--output json` is silently ignored — see Findings §5).

**On-chain extrinsic:** `SubtensorModule::reveal_weights(origin, netuid: u16, uids: Vec<u16>, values: Vec<u16>, salt: Vec<u16>, version_key: u64)`

> Note: the typed API dispatch name is `reveal_weights`. The pallet source calls this `reveal_crv3_weights` internally — see Findings §3.

**Events emitted:** `CRV3WeightsRevealed { netuid, account }`.

**Pallet errors:** `NoWeightsCommitFound` · `InvalidRevealCommitHashNotMatch` · `ExpiredWeightCommit` · `RevealTooEarly` · `CommitRevealDisabled`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — `fn reveal_crv3_weights`.

**Source:** `WeightCommands::Reveal` in `src/cli/weights_cmds.rs`.

---

### weights status

Check **your** hotkey's pending commit-reveal weight commits on a subnet. Shows commit hash, commit block, reveal window range, and a human-readable phase (WAITING / READY TO REVEAL / EXPIRED).

```bash
agcli weights status --netuid <U16>
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |

Uses the default wallet/hotkey from global flags.

**Exit codes:**
- `0` — success (including "no pending commits" case)
- `12` — validation: netuid=0, subnet not found
- `11` — wallet/hotkey error
- `10` — RPC error on storage queries after preflight

**Pre-flight:** `validate_netuid` → `require_subnet_exists_for_weights_cmd` (subnet existence; RPC error warns + continues) → wallet unlock → hotkey load → `try_join!(get_weight_commits, get_block_number, get_subnet_hyperparams, get_reveal_period_epochs)`.

**Output:** Human text only — **no JSON mode** (see Findings §6). The output includes: hotkey (short SS58), current block, commit-reveal enabled/disabled, reveal period in epochs, and per-commit: hash, commit block, reveal window blocks, and phase status.

**No extrinsic submitted** — read-only.

**Storage keys:** `SubtensorModule::WeightCommits(netuid, hotkey)` · `SubtensorModule::RevealPeriodEpochs(netuid)`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — storage reads only.

**Source:** `WeightCommands::Status` in `src/cli/weights_cmds.rs`.

---

### weights commit-timelocked

Commit weights under **drand** timelock. The chain stores a hash tied to a `--round`; decryption is driven automatically by drand when that round becomes available. No manual reveal step is required — the chain handles decryption via `reveal_timelocked_commitments` when the drand pulse for the round arrives.

```bash
agcli weights commit-timelocked --netuid <U16> --weights <WEIGHTS> --round <U64> [--salt <STRING>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--weights` | string | yes | — | Weight input (see Weight Format) |
| `--round` | u64 | yes | — | Drand round number for timelock reveal |
| `--salt` | string | no | random 32-char alphanumeric | Salt for the commit hash; printed if generated |

**Exit codes:**
- `0` — committed
- `12` — netuid=0, subnet not found, empty weights
- `11` — wallet error
- `13` — chain: `IncorrectCommitRevealVersion` (dispatch 111) if `commit_reveal_version` mismatches; `CommittingWeightsTooFast`; `TooManyUnrevealedCommits`
- `10` — RPC error fetching `CommitRevealWeightsVersion` before submit

**Pre-flight:** `validate_netuid` → `validate_weight_input` → `require_subnet_exists_for_weights_cmd` (RPC error warns + continues) → wallet unlock → `compute_weight_commit_hash(uids, weights, salt_bytes)` → **`get_commit_reveal_weights_version()`** (RPC; failure → exit 10) → extrinsic submit.

**Hash computation:** same as `weights commit` — blake2b-256 over `uids_le_bytes || values_le_bytes || salt_raw_bytes`.

**Output:** Human text only (`--output json` silently ignored — see Findings §5).

**On-chain extrinsic (raw dynamic call):** `SubtensorModule::commit_timelocked_weights(netuid: u128, commit: bytes[32], reveal_round: u128, commit_reveal_version: u128)`

> This extrinsic is called via `submit_raw_call` (dynamic string dispatch), not the typed subxt API. Dispatchable name `"commit_timelocked_weights"` is a string literal — no compile-time pallet verification (see Findings §4).

**Events emitted:** `TimelockedWeightsCommitted { account, netuid, commit, reveal_round }`.

**Pallet errors:** `IncorrectCommitRevealVersion` (111) · `CommittingWeightsTooFast` · `TooManyUnrevealedCommits`.

**Storage writes:** `SubtensorModule::TimelockedWeightCommits(netuid, hotkey)`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — `fn commit_timelocked_weights`.

**Source:** `WeightCommands::CommitTimelocked` in `src/cli/weights_cmds.rs`.

---

### weights commit-reveal

Atomic: commit weights, wait for the reveal window, then auto-reveal — all in a single blocking command.

```bash
agcli weights commit-reveal --netuid <U16> --weights <WEIGHTS> [--version-key <U64>] [--wait]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--weights` | string | yes | — | Weight input (see Weight Format) |
| `--version-key` | u64 | no | `0` | Version key for the reveal extrinsic |
| `--wait` | bool flag | no | false | After reveal, print a JSON summary and exit |

**Exit codes:**
- `0` — commit + reveal complete (or fallback set complete)
- `12` — netuid=0, subnet not found (before wallet)
- `11` — wallet error
- `13` — chain rejection at commit or reveal step
- `10` — RPC error (hyperparams required here; RPC failure → exit 10, not warn+continue)

**Pre-flight (strict):** `validate_netuid` → `validate_weight_input` → `resolve_weights` → **`get_subnet_hyperparams`** (required; RPC failure here exits with error + connectivity hint, unlike `weights commit`/`reveal`/`status`).

**Behavior:**
1. If commit-reveal **disabled** on subnet: prints warning, falls back to `set_weights` (direct). No exit-code difference from the enabled path — see Findings §7.
2. If commit-reveal **enabled**: generates random 32-char alphanumeric salt, computes blake2b-256 hash, submits `commit_weights`, polls finalized block every 12s until `block_at_commit + commit_reveal_weights_interval × tempo`, then submits `reveal_weights`.
3. `--wait`: after reveal, queries finalized block number and prints JSON summary.

**`--wait` JSON output:**
```json
{
  "status": "complete",
  "netuid": 1,
  "commit_tx": "0xabc...",
  "reveal_tx": "0xdef...",
  "commit_block": 12000,
  "reveal_block": 12720,
  "num_weights": 256
}
```

**Long-running:** The polling loop blocks for `commit_reveal_weights_interval × tempo × 12s` (e.g. 2 × 360 × 12 = 8640s ≈ 144 minutes for a typical mainnet subnet). Use `--finalization-timeout` to tune per-block wait.

**Pallet ref / events:** same as `weights commit` + `weights reveal` (see those sections).

**Source:** `WeightCommands::CommitReveal` in `src/cli/weights_cmds.rs`.

---

### weights set-mechanism

Set weights for a single mechanism (`--mechanism-id`: **0** = Yuma, **1** = Oracle) without commit-reveal. Same weight input format as `weights set`.

```bash
agcli weights set-mechanism --netuid <U16> --mechanism-id <U16> --weights <WEIGHTS> [--version-key <U64>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--mechanism-id` | u16 | yes | — | Mechanism index (0=Yuma, 1=Oracle) |
| `--weights` | string | yes | — | Weight input (see Weight Format) |
| `--version-key` | u64 | no | `0` | Version key |
| `--dry-run` (global) | bool flag | no | false | Print JSON; no extrinsic submitted |

**Exit codes:** same pattern as `weights set`.

**Dry-run JSON output:**
```json
{
  "dry_run": true,
  "netuid": 1,
  "mechanism_id": 0,
  "mechanism": "Yuma",
  "num_weights": 3,
  "version_key": 0
}
```

**Output (live):** Human text only (`--output json` silently ignored — see Findings §5).

**On-chain extrinsic (raw dynamic call):** `SubtensorModule::set_mechanism_weights(netuid: u128, mechanism_id: u128, uids: Vec<u128>, values: Vec<u128>, version_key: u128)`

> Dispatched via `submit_raw_call` with string literal `"set_mechanism_weights"`. Values are encoded as `Value::u128` per element. No compile-time type check (see Findings §4).

**Events emitted:** `MechanismWeightsSet { netuid, mechanism_id, uid }` (event name as emitted by pallet — verify against `macros/events.rs`).

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs` — `fn set_mechanism_weights`.

**Source:** `WeightCommands::SetMechanism` in `src/cli/weights_cmds.rs`.

---

### weights commit-mechanism

Commit a precomputed blake2b-256 hash for one mechanism's weight vector. Unlike `weights commit`, **you supply the hash directly** — compute it offline using the same algorithm as `weights commit`: `blake2b-256(uids_le_bytes || values_le_bytes || salt_raw_bytes)`.

```bash
agcli weights commit-mechanism --netuid <U16> --mechanism-id <U16> --hash <HEX64>
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--mechanism-id` | u16 | yes | — | Mechanism index (0=Yuma, 1=Oracle) |
| `--hash` | hex string | yes | — | 32-byte blake2b-256 hash (64 hex chars; optional `0x` prefix) |

**Exit codes:**
- `12` — invalid hex, not exactly 32 bytes, netuid=0, subnet not found
- `11` — wallet error
- `13` — chain: `CommitRevealDisabled`, `CommittingWeightsTooFast`, `TooManyUnrevealedCommits`
- `10` — RPC error

**Contrast with `weights commit`:** `weights commit` takes `--weights` + optional `--salt` and computes the hash internally. `weights commit-mechanism` requires a **precomputed** `--hash`; there is no `--salt` flag on this subcommand. This is an API asymmetry that can confuse agents.

**Output:** Human text only.

**On-chain extrinsic (raw dynamic call):** `SubtensorModule::commit_mechanism_weights(netuid: u128, mechanism_id: u128, commit_hash: bytes[32])`

> Dispatched via `submit_raw_call` with string literal `"commit_mechanism_weights"` (see Findings §4).

**Events emitted:** `MechanismWeightsCommitted { account, netuid, mechanism_id, commit }`.

**Pallet errors:** `CommitRevealDisabled` · `CommittingWeightsTooFast` · `TooManyUnrevealedCommits`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs`.

**Source:** `WeightCommands::CommitMechanism` in `src/cli/weights_cmds.rs`.

---

### weights reveal-mechanism

Reveal mechanism-specific weights after `weights commit-mechanism`. Submit the **same** uid:weight vector, `--salt`, and `--version-key` used when building the commit hash.

```bash
agcli weights reveal-mechanism --netuid <U16> --mechanism-id <U16> --weights <WEIGHTS> --salt <STRING> [--version-key <U64>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--netuid` | u16 | yes | — | Target subnet UID |
| `--mechanism-id` | u16 | yes | — | Mechanism index (0=Yuma, 1=Oracle) |
| `--weights` | string | yes | — | Same weight input as used to build the commit hash |
| `--salt` | string | yes | — | Same salt string used to build the commit hash |
| `--version-key` | u64 | no | `0` | Version key for the reveal |

**Exit codes:** same pattern as `weights reveal`.

**Salt encoding:** identical to `weights reveal` — UTF-8 bytes split into little-endian u16 pairs.

**Output:** Human text only (`--output json` silently ignored — see Findings §5).

**On-chain extrinsic (raw dynamic call):** `SubtensorModule::reveal_mechanism_weights(netuid: u128, mechanism_id: u128, uids: Vec<u128>, values: Vec<u128>, salt: Vec<u128>, version_key: u128)`

> Dispatched via `submit_raw_call` with string literal `"reveal_mechanism_weights"` (see Findings §4).

**Events emitted:** `MechanismWeightsRevealed { netuid, account, mechanism_id }`.

**Pallet errors:** `NoWeightsCommitFound` · `InvalidRevealCommitHashNotMatch` · `ExpiredWeightCommit` · `RevealTooEarly`.

**Pallet ref:** `subtensor/pallets/subtensor/src/subnets/weights.rs`.

**Source:** `WeightCommands::RevealMechanism` in `src/cli/weights_cmds.rs`.

---

## Weight Format

The `--weights` argument accepts four formats:

| Format | Example |
|--------|---------|
| `uid:weight` pairs | `"0:100,1:200,2:50"` |
| JSON array | `'[{"uid":0,"weight":100},{"uid":1,"weight":200}]'` |
| JSON object | `'{"0":100,"1":200}'` |
| stdin (`-`) | pipe JSON to stdin |
| file (`@path`) | `"@weights.json"` |

- `uid` = neuron UID (u16, range 0–65535; must exist in metagraph for the set call to succeed)
- `weight` = weight value (u16, range 0–65535)
- Weights are normalized on-chain to sum to u16::MAX (65535 = 1.0)
- Overflow (uid or weight > 65535) is rejected at parse time with an explicit error
- Object map key order is not guaranteed — use array format for deterministic ordering

## Commit-Reveal Flow

```
1. agcli weights commit --netuid N --weights "..." [--salt S]
   → prints commit hash (0x...) and salt to stdout
   → save the salt

2. Wait for reveal window:
   block_current >= block_at_commit + commit_reveal_weights_interval * tempo

3. agcli weights reveal --netuid N --weights "..." --salt S [--version-key V]
   → must use the EXACT same weights and salt string from step 1
```

Or use `agcli weights commit-reveal` to automate steps 1–3.

Check pending commits: `agcli weights status --netuid N`

## Advanced: Mechanism Weights

Subnets with multiple consensus mechanisms (indexed by MechId) have per-mechanism weight matrices. The storage index is `netuid * MAX_MECHANISMS + mecid`.

| CLI | Pallet dispatch | Notes |
|-----|----------------|-------|
| `weights set-mechanism` | `set_mechanism_weights` | direct, no CR; raw dynamic call |
| `weights commit-mechanism` | `commit_mechanism_weights` | takes precomputed `--hash`; raw dynamic call |
| `weights reveal-mechanism` | `reveal_mechanism_weights` | takes `--weights` + `--salt`; raw dynamic call |

All three mechanism weight extrinsics use `submit_raw_call` (string-based dispatch), not the typed subxt API. See Findings §4.

## Advanced: Timelocked Weights (Drand)

Weights can be committed with drand-based timelock encryption — auto-decrypted when the specified drand round arrives, without requiring a reveal transaction from the submitter.

On-chain: `commit_timelocked_weights(netuid, commit, reveal_round, commit_reveal_version)`
- Events: `TimelockedWeightsCommitted { account, netuid, commit, reveal_round }`
- Storage: `TimelockedWeightCommits(netuid, hotkey)`
- The `commit_reveal_version` is read from chain storage (`CommitRevealWeightsVersion`) at submit time; error `IncorrectCommitRevealVersion` (dispatch code 111) means version mismatch.

## Advanced: Batch Weight Operations (SDK only — no CLI surface)

The SDK in `src/chain/extrinsics.rs` exposes batch operations not yet wired to any CLI subcommand:

| SDK function | Pallet dispatch | CLI surface |
|-------------|----------------|-------------|
| `batch_set_weights` | `SubtensorModule::batch_set_weights` | **none** |
| `batch_commit_weights` | `SubtensorModule::batch_commit_weights` | **none** |
| `batch_reveal_weights` | `SubtensorModule::batch_reveal_weights` | **none** |

Agents cannot use these via `agcli weights`. See Findings §1.

## Extrinsic Finalization Timeouts

After submit, agcli waits for inclusion/finalization (default 30s). If the chain stops producing blocks or the RPC lags: `Transaction timed out after Ns waiting for finalization`. Increase via `--finalization-timeout <SECS>`, env `AGCLI_FINALIZATION_TIMEOUT`, or `finalization_timeout` in `~/.agcli/config.toml`. Exit code: `15` (timeout).

## Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `NotEnoughStakeToSetWeights` | Hotkey alpha < ~1000τ on subnet | Stake more on this subnet |
| `SettingWeightsTooFast` | Rate limit not expired | Wait `weights_rate_limit` blocks |
| `CommitRevealEnabled` | Used `set` when CR is on | Use `commit-reveal` instead |
| `CommitRevealDisabled` | Used `commit`/`reveal` when CR is off | Use `set` instead |
| `InvalidRevealCommitHashNotMatch` | Wrong weights or salt on reveal | Use exact same values from commit |
| `ExpiredWeightCommit` | Reveal window passed | Re-commit and reveal sooner |
| `RevealTooEarly` | Reveal window not open yet | Wait for reveal window |
| `UidVecContainInvalidOne` | UID not in metagraph | Check `agcli subnet metagraph` |
| `WeightVecLengthIsLow` | Fewer UIDs than `min_allowed_weights` | Check `agcli subnet hyperparams --netuid N` |
| `IncorrectCommitRevealVersion` (111) | `commit_reveal_version` mismatch | Update agcli; CLI reads chain version before submit |
| `TooManyUnrevealedCommits` | Max pending commits exceeded | Reveal or wait for expiry of existing commits |
| Finalization timeout | No new finalized blocks within `--finalization-timeout` | Increase timeout / fix RPC |

## Exit Code Reference

| Code | Constant | Meaning (weights context) |
|------|----------|--------------------------|
| 0 | — | Success |
| 1 | `GENERIC` | Uncategorized error (e.g. hotkey not found on subnet) |
| 10 | `NETWORK` | RPC/WebSocket unreachable or timeout |
| 11 | `AUTH` | Wrong password, missing keyfile, no hotkey loaded |
| 12 | `VALIDATION` | Invalid input before chain contact (netuid=0, bad hex hash, subnet not found, empty weights) |
| 13 | `CHAIN` | Extrinsic rejected on-chain (rate limit, stake, hash mismatch, version key, etc.) |
| 14 | `IO` | File read error for `@path` weights |
| 15 | `TIMEOUT` | Finalization timeout |

## Source Code

**agcli handler:** `src/cli/weights_cmds.rs` — `handle_weights()`.

**Subcommands:** `Show` · `Set` · `Commit` · `Reveal` · `CommitReveal` · `Status` · `CommitTimelocked` · `SetMechanism` · `CommitMechanism` · `RevealMechanism`

**Hash helper:** `src/extrinsics/weights.rs` — `compute_weight_commit_hash`.

**Subtensor pallet:**
- `subtensor/pallets/subtensor/src/subnets/weights.rs` — `set_weights`, `commit_crv3_weights`, `reveal_crv3_weights`, mechanism weights, timelocked weights
- `subtensor/pallets/subtensor/src/macros/dispatches.rs` — dispatch entry points
- `subtensor/pallets/subtensor/src/macros/events.rs` — WeightsSet, CRV3WeightsCommitted, CRV3WeightsRevealed, TimelockedWeightsCommitted, BatchWeightsCompleted, BatchCompletedWithErrors, BatchWeightItemFailed
- `subtensor/pallets/subtensor/src/macros/errors.rs` — weight-related error definitions

## Related Commands

- `agcli subnet hyperparams --netuid N` — Check `weights_rate_limit`, `commit_reveal_weights_enabled`, `commit_reveal_weights_interval`, `tempo`, `min_allowed_weights`
- `agcli subnet watch --netuid N` — Live tempo countdown and weight window status
- `agcli subnet commits --netuid N` — See all pending commits on a subnet
- `agcli explain --topic commit-reveal` — How commit-reveal works
- `agcli explain --topic rate-limits` — Weight rate limit details
- `agcli explain --topic yuma` — How weights feed into consensus

---

## Findings

The following drift and issues were found during this audit. These are listed for the planner to triage; no source files have been modified.

### §1 — Batch weight operations have SDK functions but no CLI surface

`src/chain/extrinsics.rs` exposes `batch_set_weights`, `batch_commit_weights`, and `batch_reveal_weights` with fully typed pallet calls and length-validation logic. None of these have corresponding `WeightCommands` variants. Agents cannot invoke any batch weight operation through `agcli weights`. The pallet supports batch operations (`batch_set_weights`, `batch_commit_weights`, `batch_reveal_weights` in `macros/dispatches.rs`).

**Suggested follow-up:** Add `WeightCommands::BatchSet`, `BatchCommit`, `BatchReveal` variants wired to the existing SDK functions.

### §2 — `commit_crv3_mechanism_weights` has no CLI surface and no SDK function

The task scope lists `commit_crv3_mechanism_weights` as a pallet dispatchable in scope. This function is not present anywhere in `src/` — neither as an extrinsic in `extrinsics.rs` nor as a CLI subcommand. If the pallet exposes this dispatch (separate from `commit_mechanism_weights`), it is entirely absent from agcli.

**Suggested follow-up:** Verify whether `commit_crv3_mechanism_weights` is a distinct pallet dispatch or the same as `commit_mechanism_weights`; add SDK + CLI surface if distinct.

### §3 — Dispatchable name mismatch between docs and typed API calls

`docs/commands/weights.md` (previous version) stated the on-chain functions are `commit_crv3_weights` and `reveal_crv3_weights`. The actual typed subxt API calls in `extrinsics.rs` are `api::tx().subtensor_module().commit_weights(...)` and `api::tx().subtensor_module().reveal_weights(...)`. These are the generated names from the chain metadata. Either the docs were wrong (the metadata exposes them under the shorter names) or the metadata is older than the pallet source that uses `_crv3_` internally. Either way, the pallet-ref documentation was misleading.

### §4 — Mechanism weights and timelocked weights use unverified raw dynamic dispatch

`set_mechanism_weights`, `commit_mechanism_weights`, `reveal_mechanism_weights`, and `commit_timelocked_weights` all call `submit_raw_call` with string literals for the pallet name and dispatch name. This means:
- No compile-time verification that the dispatch exists in the metadata.
- If the pallet renames or removes these dispatches, the calls will fail at runtime with `DispatchNotFound` rather than at build time.
- `set_weights`, `commit_weights`, `reveal_weights`, `batch_set_weights`, `batch_commit_weights`, `batch_reveal_weights` all use the typed `api::tx()` path and *do* get compile-time verification.

**Suggested follow-up:** Generate typed bindings for these four dispatches (they appear in the task's pallet scope) and switch `extrinsics.rs` to typed calls.

### §5 — Write commands silently ignore `--output json`

Every write subcommand (`weights set` live path, `weights commit`, `weights reveal`, `weights commit-timelocked`, `weights set-mechanism`, `weights commit-mechanism`, `weights reveal-mechanism`) prints human-readable text and never reads `ctx.output`. Only the dry-run JSON path in `weights set` and `weights set-mechanism`, and the `--wait` path in `weights commit-reveal`, produce any JSON. Agents expecting machine-parseable output on write commands receive plain text with exit 0.

**Suggested follow-up:** All write commands should check `ctx.output.is_json()` and emit `{"tx": "<hash>", "netuid": N, ...}` when enabled.

### §6 — `weights status` has no JSON output mode

`WeightCommands::Status` produces human text unconditionally. The output includes structured data (hash, block numbers, reveal window, phase status) that agents would benefit from consuming as JSON.

**Suggested follow-up:** Add a JSON output branch to `WeightCommands::Status` emitting `{"hotkey": "...", "current_block": N, "commit_reveal_enabled": bool, "reveal_period_epochs": N, "commits": [...]}`.

### §7 — `weights commit-reveal` fallback to `set_weights` is silent and undifferentiated

When commit-reveal is disabled on the subnet, `WeightCommands::CommitReveal` silently falls back to `set_weights` (direct set), printing only `"Warning: SN{N} does NOT have commit-reveal enabled. Using direct set_weights instead."` to stderr. The command exits 0 either way. An agent that expects a commit-reveal flow (two-phase, waiting for window) will silently get a direct set instead. There is no flag to disable the fallback or force failure when CR is off.

**Suggested follow-up:** Add `--no-fallback` flag to `weights commit-reveal` that exits 12 (validation) when commit-reveal is disabled rather than silently falling back to direct set.

### §8 — Salt encoding asymmetry between `weights commit` and `weights commit-mechanism`

`weights commit` accepts `--weights` + optional `--salt` and computes the hash internally using `compute_weight_commit_hash`. `weights commit-mechanism` requires a **precomputed** `--hash` with no `--weights` or `--salt` flag. This asymmetry means an agent using the mechanism commit-reveal flow must compute the hash out-of-band (e.g. via a separate tool or script) whereas the global commit-reveal flow has an end-to-end command. The docs did not clearly explain this difference.

**Suggested follow-up:** Add a `--weights` + `--salt` path to `weights commit-mechanism` that computes the hash internally (matching `weights commit`), or at minimum add a `agcli utils hash-weights` subcommand for offline hash computation.

### §9 — No parse-surface tests for four subcommands in existing test files

Prior to this audit, `tests/cli_weights.rs` had zero tests for `commit-timelocked`, `set-mechanism`, `commit-mechanism`, and `reveal-mechanism`. These four subcommands also had no error-path coverage (missing required flags). The new `tests/audit_weights.rs` adds coverage for all four.

### §10 — `validate_weights_args` is a no-op for `WeightCommands::Show`

`validate_weights_args` in `src/cli/weights_cmds.rs` matches `WeightCommands::Show { .. } => {}` — it does nothing. The `netuid` and `hotkey` validations for `Show` happen inside `handle_weights` at runtime. This is functionally correct but means early-exit validation (`weights_cmd_requires_wallet` → fast-fail path in the dispatcher) skips all Show-path validation. Not a bug, but worth noting for consistency if additional validation is ever added.
