<!-- orchestrate handoff
task: audit-diff
branch: orch/agcli-audit/audit-diff
agentId: bc-cc755e6a-33a4-4e61-ad8d-820f3ab85bac
runId: run-a612de87-4e6c-4147-9f81-ac21537c4a69
resultStatus: finished
finishedAt: 2026-05-27T13:31:53.719Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-diff`

## What I did

- **`tests/audit_diff.rs`** (new, ~300 LOC): 39 parse-surface tests + 1 `#[ignore]`-gated integration test covering all 4 `DiffCommands` subcommands (`portfolio`, `subnet`, `network`, `metagraph`). Tests include: variant matching via destructuring, required-flag rejection with `ErrorKind::MissingRequiredArgument` assertions, u32/u16 boundary checks (zero, max, overflow), unknown-flag rejection, global-flag threading (`--output json`, `--endpoint`, `--network archive`), 4 exit-code classification tests against `agcli::error::classify`, and 4 compile-time audit guards that document the code-vs-docs drift as executable assertions. One `#[ignore]`-gated `green_path_diff` tokio test for localnet (Docker unavailable in VM).

- **`docs/commands/diff.md`** (full rewrite, ~280 LOC): Every subcommand documented with a flags table (type + required), exit code table cross-referenced to `src/error.rs` constants, exact JSON schema with field names matching the `serde_json::json!{}` calls in the handler, pallet + storage key refs for every read (Balances, SubtensorModule storage, three runtime APIs), on-chain events (none — all read-only), and a Findings section with 6 concrete drift items sourced from code inspection.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_diff`: exit 0 (compiles)
- `cargo test --test audit_diff -- --skip green_path_diff`: 39 passed, 0 failed, 1 filtered (ignored)

## Verification
`unit-test-verified` — 39 parse-surface and audit-guard tests pass. Localnet integration test is `#[ignore]`-gated (Docker unavailable in VM).

## Findings

1. **`diff subnet` JSON output omits `tempo` and `owner_hotkey`** — the human table in `handle_diff::Subnet` prints both fields for both blocks (L372–L378), but the `serde_json::json!{}` object at L301–L312 includes neither. Machine consumers using `--output json` cannot retrieve subnet tempo or owner hotkey changes. Suggested fix: add `"tempo": [d1.tempo, d2.tempo]`, `"owner_hotkey": [d1.owner_hotkey, d2.owner_hotkey]`, `"tempo_diff"`, in the JSON branch.

2. **`diff network` JSON has no diff scalar fields** — `diff portfolio` and `diff subnet` both include `*_diff` scalar companions for each two-element array (e.g., `"balance_diff_tao"`, `"tao_in_diff"`). `diff network` returns only the raw arrays (`[val_block1, val_block2]`), forcing consumers to compute deltas themselves. Inconsistent schema across the diff group. Suggested fix: add `total_issuance_diff_tao`, `total_stake_diff_tao`, `staking_ratio_diff_pct`, `subnet_count_diff`.

3. **`diff metagraph` silently drops removed neurons** — the diff loop in `handle_diff::Metagraph` (L482–L506) iterates `neurons2` and does a HashMap lookup into `neurons1`. UIDs present in block1 but absent in block2 are never added to `changes`, so the "removed" case is invisible in both human and JSON output. A complete diff should iterate both directions.

4. **`diff portfolio` "no address" error exits GENERIC (1) not VALIDATION (12)** — `anyhow::bail!("No address provided and no wallet found.")` at L188 does not match any VALIDATION pattern in `classify()`. From a scripting perspective this is a user-input error; the inconsistency means exit-code consumers can't distinguish it from a transport failure.

5. **`diff portfolio` balance diff uses f64 subtraction** — `balance_diff_tao` = `bal2.tao() - bal1.tao()` (f64 after rao→tao conversion). For sub-rao differences the result may be inexact due to floating-point cancellation. Computing `(rao2 - rao1) as f64 / 1e9` at the integer level first is always exact.

6. **`diff subnet` `emission_diff` JSON is `i128`** — `emission` fields are `u64`; the diff `d2.emission as i128 - d1.emission as i128` is correct, but the JSON schema was undocumented and the type diverges from the other diff fields (which are `f64`). Documented in the refreshed diff.md.

## Notes, concerns, deviations, findings, thoughts, feedback

- All four `DiffCommands` subcommands are entirely read-only; no extrinsics to audit for SCALE encoding. The audit focused on read-path accuracy, output schema completeness, and exit-code classification.
- The upstream `audit-block` worker already added some diff parse tests in `tests/cli_test_modules/part01.rs` and `part04.rs`. `tests/audit_diff.rs` provides focused, self-contained audit coverage with explicit variant matching and audit-guard tests that would otherwise be scattered.
- Docker was unavailable in this VM; the `green_path_diff` integration test is `#[ignore]`-gated accordingly.
- The subtensor submodule was not needed — all diff commands use standard runtime APIs (`StakeInfoRuntimeApi`, `SubnetInfoRuntimeApi`, `NeuronInfoRuntimeApi`) and frame storage, all of which are accessible via the generated `api` types in the compiled binary.

## Suggested follow-ups

- **Planner:** Fix `handle_diff::Subnet` JSON to add `tempo` and `owner_hotkey` fields (both values + diffs). Source: `src/cli/block_cmds.rs` L301–L312.
- **Planner:** Fix `handle_diff::Network` JSON to add `*_diff` scalar fields alongside the two-element arrays for `total_issuance_tao`, `total_stake_tao`, `staking_ratio_pct`, and `subnet_count`. Source: `src/cli/block_cmds.rs` L406–L414.
- **Planner:** Fix `handle_diff::Metagraph` to emit `"change": "removed"` entries for UIDs in block1 not present in block2 (iterate `map1` after the block2 loop). Source: `src/cli/block_cmds.rs` L482–L506.
- **Planner:** Add `"no address provided"` message to `classify()` VALIDATION patterns, or pass a labelled error message (e.g., `"diff portfolio --address"`) so it exits 12 consistently with other missing-input errors.
- **Planner:** Normalize balance diffs to integer subtraction at rao level before converting to f64 in `handle_diff::Portfolio`.