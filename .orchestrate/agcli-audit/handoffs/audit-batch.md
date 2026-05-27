<!-- orchestrate handoff
task: audit-batch
branch: orch/agcli-audit/audit-batch
agentId: bc-68aac5ab-c145-4220-a752-0fbcf596f165
runId: run-05e8385b-2f74-4f9f-b4fc-ae2a183a320f
resultStatus: finished
finishedAt: 2026-05-27T13:16:05.365Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-batch`

## What I did

- **`tests/audit_batch.rs`** (new, ~290 LOC): 28 parse-surface + exit-code tests + 1 `#[ignore]` localnet integration stub:
  - `Commands::Batch` parse coverage: default mode, `--no-atomic`, `--force`, both flags simultaneously, missing `--file` (expected error), global `--yes`/`--output json`/`--batch` interop, paths with spaces, absolute paths
  - `validate_batch_file` unit tests: empty array, invalid JSON, non-array JSON, missing `pallet`/`call`/`args` fields, non-array `args`, 1001-call cap rejection, 1000-call boundary acceptance, non-object entries, valid single/multi call
  - Exit-code classification: IO (14) for missing file via chained `io::Error`, VALIDATION (12) for `serde_json::Error` chain, GENERIC (1) for plain `bail!` messages (documents the classify() gap), CHAIN (13) for on-chain `TooManyCalls`
  - One `#[ignore]` green-path test that invokes the `agcli` binary via `std::process::Command` (internal `handle_batch` is not pub)

- **`docs/commands/batch.md`** (refreshed, 43 → ~185 lines): Complete rewrite including clap flag tables with types, batch mode table mapping each flag to its Utility pallet dispatchable, JSON format spec with field requirements and type mapping, 1000-call limit, on-chain events (`BatchCompleted`, `BatchCompletedWithErrors`, `BatchInterrupted`, `ItemCompleted`, `ItemFailed`), exit-code table for all 7 codes, full Utility pallet dispatchable coverage table, and Audit Findings section

## Measurements

- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_batch`: compiles → compiles (exit 0)
- `cargo test --test audit_batch`: 28 passed, 0 failed, 1 ignored
- `LOC(tests/audit_batch.rs)`: 0 → ~290
- `LOC(docs/commands/batch.md)`: 43 → ~185

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

The `agcli batch` command is a **single `Commands::Batch { file, no_atomic, force }` variant**, not a subcommand group. It uses `subxt::dynamic::tx` for inner call encoding and wraps them in `Utility::{batch_all,batch,force_batch}` with `Value::from_bytes(SCALE-bytes)` for each inner call. This is the expected dynamic subxt approach for opaque `RuntimeCall` bytes.

### Findings (concrete audit observations)

1. **Four Utility dispatchables have no agcli surface.** `as_derivative`, `dispatch_as`, `with_weight`, and `if_else` are in the `Utility` pallet but cannot be called from agcli. Agents needing derivative-account dispatch or weight-capped calls have no CLI path.

2. **`--no-atomic` and `--force` can be combined; `--no-atomic` is silently discarded.** Clap has no `conflicts_with` between the two flags. The handler resolves the conflict with `if force { ... } else if no_atomic { ... }`, making `--force` win silently. An agent supplying both flags expecting non-atomic behavior would submit a `force_batch` instead, with no warning.

3. **Client-side `validate_batch_file` errors exit with GENERIC (1) instead of VALIDATION (12).** Messages like "is empty", "has too many calls", "missing pallet field" are plain `anyhow::bail!` strings not matched by `error::classify()`. All such validation failures yield exit code 1, making it impossible for an agent to distinguish input-format problems from unexpected runtime failures.

4. **JSON output (`--output json`) is too sparse.** `print_tx_result` emits only `{"tx_hash": "..."}`. The number of calls submitted, the batch mode used, and whether MEV shielding was applied are written to stderr only. Agents that need to verify what was actually submitted must parse stderr, which is fragile.

5. **Diagnostic `eprintln!` output goes to stderr unconditionally.** Lines such as `"Batch: 3 calls, mode=batch_all (atomic)"` and `"  #0: SubtensorModule.add_stake (42 bytes)"` are emitted regardless of `--output json`. This is correct behavior (diagnostics on stderr) but is not documented, and agents may need to suppress or redirect stderr.

6. **No stdin support.** `--file` always requires a filesystem path. There is no `--file -` convention for reading a batch JSON from stdin, which limits pipe-oriented agent workflows.

## Suggested follow-ups

- **Add `conflicts_with`** between `--no-atomic` and `--force` in the clap definition (or document priority explicitly in the help text).
- **Expose `as_derivative` and `dispatch_as`** as new flags/subcommands on `batch` for agents that need derivative or sudo-origin calls.
- **Improve exit-code coverage in `error::classify()`** for `validate_batch_file` messages: match on "is empty", "too many calls", "missing field" → VALIDATION (12).
- **Extend JSON output** to include `{"tx_hash": "...", "calls_count": N, "mode": "batch_all", "mev": false}`.
- **Add `--file -` stdin support** to `batch` for pipeline composition.