<!-- orchestrate handoff
task: audit-utils-cli
branch: orch/agcli-audit/audit-utils-cli
agentId: bc-e5650f05-2171-420c-ae85-c1da7736565a
runId: run-c5370b58-b0ab-4b31-95a2-20b4444a50df
resultStatus: finished
finishedAt: 2026-05-27T13:28:09.952Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-utils-cli`

## What I did

- **`tests/audit_utils_cli.rs`** (new, ~310 LOC): 23 parse-surface + exit-code tests, 1 `#[ignore]` green-path integration stub:
  - `UtilsCommands::Convert` coverage: RAO→TAO (default), TAO→RAO (`--to-rao`), TAO→Alpha (`--tao` + `--netuid`), Alpha→TAO (`--alpha` + `--netuid`), zero amount, missing amount (optional), cross-field constraint (no clap enforcement), global `--output json` threading, unknown flag rejection (`--rao` does not exist — confirmed docs drift)
  - `UtilsCommands::Latency` coverage: default `pings=5`, `--pings` override, `--extra` single URL, `--extra` comma-separated, `--count` rejection (docs used wrong flag name), `--pings 0` boundary, global `--yes` threading
  - `utils` with no subcommand is an error (parse-level)
  - `safe_rao` arithmetic spot-checks (unit-level, no chain)
  - `error::classify()` tests: `--netuid required` → GENERIC, `Chain connection required` → GENERIC, `Invalid RAO amount` → VALIDATION (because message contains "must be ")
  - One `#[ignore]` green-path that drives the `agcli` binary against `ws://127.0.0.1:9944`

- **`docs/commands/utils.md`** (full rewrite, 54 → ~215 lines):
  - Subcommand table with chain-required column
  - Full flag tables with types and defaults for both `convert` and `latency`
  - All four `convert` modes documented with example invocations, JSON output schemas, chain call traced to `SwapRuntimeApi::sim_swap_tao_for_alpha` / `sim_swap_alpha_for_tao`
  - Exit-code table with known GENERIC-vs-VALIDATION drift
  - 6 concrete audit findings inline

## Measurements

- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_utils_cli`: compiles → compiles (exit 0)
- `cargo test --test audit_utils_cli`: 23 passed, 0 failed, 1 ignored
- `LOC(tests/audit_utils_cli.rs)`: 0 → ~310
- `LOC(docs/commands/utils.md)`: 54 → ~215

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

`utils convert` and `utils latency` are **read-only** paths — no extrinsics, no SCALE-encoded dispatchable arguments to validate.  The "pallet cross-reference" task therefore reduces to tracing the runtime-API call shape:

- TAO↔Alpha uses `SwapRuntimeApi` (part of the `swap` pallet's runtime-API surface), not a storage key lookup or a dispatchable.  There is no alpha-tao math pallet in the current subtensor submodule checkout; the math lives inside the `swap` runtime API implementation.
- `utils latency` uses `chain_getBlockNumber` RPC — not a pallet dispatchable.

Docker is not available in the cloud-agent VM, so the `#[ignore]` localnet test was not exercised end-to-end.

## Findings

1. **`--rao` flag does not exist; pre-audit docs were wrong.** The old docs showed `agcli utils convert --rao 1000000000` and `agcli utils convert --tao 1.5` as a TAO→RAO conversion.  Neither is correct: the actual flags are `--amount` (value) + optional `--to-rao` (direction).  `--tao` is the TAO→Alpha simulation flag, not a denomination selector.

2. **`--count` flag does not exist in `latency`; docs showed it.** Pre-audit docs claimed `--count 10`; the real flag is `--pings <N>` (default 5).

3. **`--netuid required` and `Chain connection required` errors exit GENERIC (1), not VALIDATION (12).** `error::classify()` does not match the strings emitted by those two `anyhow::bail!` paths.  The `"Invalid RAO amount: ... (must be a finite...)"` error correctly returns VALIDATION (12) because the message contains "must be ".

4. **`safe_rao` silently saturates on TAO→RAO overflow.** `--amount 9999999999999.0 --to-rao` produces a clamped `u64::MAX` result with no warning.  The amount validation guard only exists in the RAO→TAO branch, not the TAO→RAO branch.

5. **"Testing N endpoint(s)…" header printed to stdout before JSON object.** When `--output json`, the progress line breaks strict JSON parsers reading stdout.

6. **`connect_ms` present in human-readable output but absent from JSON schema.** The `EndpointResult` struct does not include `connect_ms`, so JSON consumers cannot measure connection time separately from ping round-trip time.

## Suggested follow-ups

- **Fix exit-code classification**: add `"--netuid is required"` and `"chain connection required"` patterns to the VALIDATION heuristics in `error::classify()`.
- **Add TAO→RAO overflow guard**: validate `amount` in the `--to-rao` branch (mirror the check in the RAO→TAO branch).
- **Suppress or redirect stdout header in JSON mode**: move "Testing N endpoint(s)…" to stderr when `--output json` is set.
- **Add `connect_ms` to JSON output**: include `"connect_ms": u128` in `EndpointResult`.
- **Add `--tao`/`--alpha` cross-field validation at parse time**: a custom clap validator or `requires` attribute would give better error messages than the runtime `anyhow::bail!`.