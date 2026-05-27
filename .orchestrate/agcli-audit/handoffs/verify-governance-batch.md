<!-- orchestrate handoff
task: verify-governance-batch
branch: orch/agcli-audit/audit-scheduler
agentId: bc-c0f68c90-1a6a-4301-8ef7-79586e271084
runId: run-b7b0f6cf-5f1e-44ea-bafe-1da35ca13428
resultStatus: finished
finishedAt: 2026-05-27T13:15:10.989Z
-->

## Verification
verifier-failed

## Target
`audit-scheduler` on branch `orch/agcli-audit/audit-scheduler`

## Branch
`orch/agcli-audit/audit-scheduler`

## Execution
- `rustup install stable && rustup default stable` → selected Rust 1.95.0 / Cargo 1.95.0.
- `cargo check --all-targets` → passed; finished in 1m34s.
- `cargo build --bin agcli` → passed; finished in 1m43s.
- `cargo test --no-run --workspace` → passed; compiled workspace tests, but only `tests/audit_scheduler.rs` existed among the governance-batch audit tests.
- `cargo test --no-run --test audit_scheduler` → passed.
- `cargo test --test audit_scheduler -- --skip green_path_scheduler_named_local_chain` → passed: 1 passed, 0 failed, 1 filtered.
- Docs/enum/test coverage script over scheduler, preimage, contracts, evm, safe-mode, drand → failed because non-scheduler `tests/audit_*.rs` files were absent on this checked-out branch. Docs headings did enumerate all current enum variants for all six groups.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes on the merged branch: passed on checked-out branch `orch/agcli-audit/audit-scheduler`. (met for current branch)
- [x] `cargo build --bin agcli` passes on the merged branch: passed. (met for current branch)
- [x] `cargo test --no-run --workspace` succeeds: passed. (met for current branch)
- [x] Each touched `docs/commands/<group>.md` enumerates every subcommand by matching `*Commands` enum variants against markdown headings: scheduler, preimage, contracts, evm, safe-mode, and drand docs all contained headings for every current enum variant. (met)
- [ ] Each `tests/audit_<group>.rs` file parses every subcommand variant via `Cli::try_parse_from`: `tests/audit_scheduler.rs` exists and passes; `tests/audit_preimage.rs`, `tests/audit_contracts.rs`, `tests/audit_evm.rs`, `tests/audit_safe_mode.rs`, and `tests/audit_drand.rs` are missing on this branch. (not met)

Other findings, severity-ordered:
- (high) Batch verification cannot be marked success: the checked-out branch is not a merged synthetic branch containing all dependent worker test files.
- (high) On-disk handoff files for `audit-scheduler`, `audit-contracts`, `audit-safe-mode`, and `audit-drand` are unstructured error handoffs, despite the upstream context in the prompt containing success handoffs.
- (med) Findings rollup from upstream context:
  - Scheduler: missing CLI surface for `schedule_after` / `schedule_named_after`; named task ID encoding drift; plaintext output ignores JSON mode; root privilege not surfaced; scheduler runtime errors collapse to generic exit code.
  - Preimage: missing `request_preimage` / `unrequest_preimage`; success output ignores JSON mode; numeric JSON conversion can lose intended SCALE type shape for large values.
  - Contracts: upload/instantiate omit key event-derived outputs; missing `instantiate_with_code`, `set_code`, and dry-run/query surfaces; `--determinism` missing; JSON mode ignored.
  - EVM: missing `create` / `create2`; `evm_call` likely drifts from pallet signature by omitting `authorization_list`; type-shape mismatches around gas/value; JSON mode ignored.
  - Safe-mode: `force-enter` appears to encode a spurious `duration` argument; documented `status` command does not exist; missing force/deposit management surfaces; JSON mode and important events not surfaced.
  - Drand: only `write-pulse` is exposed; signed origin likely conflicts with pallet `ensure_none`; payload/signature SCALE shape appears mismatched; missing read/query subcommands; JSON mode ignored.

## Notes & suggestions
- Scheduler-specific acceptance passed: docs mention all four scheduler subcommands and `tests/audit_scheduler.rs` compiles/runs parse coverage.
- Batch-level acceptance failed because the current branch lacks the other dependent workers’ `tests/audit_*.rs` files and several on-disk handoffs are error/unstructured.
- No source files were modified during verification; no verifier artifact commit was made.