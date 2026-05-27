<!-- orchestrate handoff
task: verify-readonly-batch
branch: orch/agcli-audit/audit-view
agentId: bc-1b15551f-8a08-4351-a4d4-cb6abc6b19c0
runId: run-8bf057b4-14d8-49c2-97f0-c04b21be2552
resultStatus: finished
finishedAt: 2026-05-27T13:41:35.416Z
-->

## Verification
verifier-failed

## Target
`audit-view` on branch `orch/agcli-audit/audit-view`

## Branch
`orch/agcli-audit/audit-view`

## Execution
- `cargo check --all-targets` → initial env failure on Cargo 1.83 / edition2024 dependency parsing.
- `rustup install stable && rustup default stable` → selected `rustc 1.95.0`, `cargo 1.95.0`.
- `cargo check --all-targets` → passed.
- `cargo build --bin agcli` → passed.
- `cargo test --no-run --workspace` → passed; compiled workspace tests present in this checkout.
- `cargo test --no-run --test audit_view && cargo test --test audit_view` → passed: `53 passed`, `0 failed`, `1 ignored`.
- `target/debug/agcli view --help` → exit 0; listed all 15 `view` subcommands.
- `rg "^## view |^## agcli audit" docs/commands/view.md` → headings found for all 15 `ViewCommands` subcommands plus top-level `agcli audit`.
- `rg "ViewCommands::...|Commands::Audit" tests/audit_view.rs` → parse assertions found for all 15 `ViewCommands` variants plus `Commands::Audit`.
- `Glob tests/audit_*.rs` → only `tests/audit_view.rs` exists on this branch.
- Read dependent handoffs under `.orchestrate/agcli-audit/handoffs/` → several required handoffs are raw error stubs, not structured success handoffs.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes: passed after updating the VM Rust toolchain.
- [x] `cargo build --bin agcli` passes: passed.
- [x] Tests compile: `cargo test --no-run --workspace` passed for tests present on this branch.
- [ ] Every dependent worker's `audit_*.rs` test file compiles: not met. Only `tests/audit_view.rs` exists; dependent audit test files are absent.
- [x] `docs/commands/view.md` mentions every `ViewCommands` subcommand: met.
- [x] `tests/audit_view.rs` parses every `ViewCommands` variant via `Cli::try_parse_from`: met; targeted test passed.
- [ ] Handoff Findings section exists for target: not met on disk. `.orchestrate/agcli-audit/handoffs/audit-view.md` has `resultStatus: error` and no structured `## Findings`.

Other findings:
- (high) This checkout is not the merged synthetic batch branch; it only contains the target view docs/test changes plus orchestrate metadata.
- (high) Required dependent handoffs for `audit-view`, `audit-block`, `audit-diff`, `audit-completions-update`, `audit-batch`, and `audit-utils-cli` are on-disk error stubs.
- (med) Target-only `audit-view` artifacts verify cleanly: all parse tests pass and CLI help/docs enumerate the view surface.
- (low) The localnet green path remains ignored; no live local-chain behavior was verified.

Findings rollup:
- `view`: JSON/CSV gaps in `neuron` and `nominations`; table/CSV column drift; validation exit-code mismatches; `--live` ordering pitfall.
- `block/diff`: finalized-vs-best block drift, missing header field, inconsistent timestamp/hash schemas, incomplete diff JSON and removed-neuron handling.
- `audit/doctor/explain/config/utils`: several commands accept CSV/JSON modes without structured output; error classification is inconsistent.
- `batch/admin/localnet`: missing/incorrect runtime dispatch surfaces, SCALE-shape mismatches, sparse JSON output, localnet readiness/log formatting issues.

## Notes & suggestions
- Treat this as a failed batch verification, not a failed target-only compile/test.
- I committed and pushed the verifier handoff artifact at `.orchestrate/agcli-audit/handoffs/verify-readonly-batch.md`.
- Rerun the verifier on the actual merged synthetic branch containing all dependent worker outputs.