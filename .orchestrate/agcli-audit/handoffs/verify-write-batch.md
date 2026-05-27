<!-- orchestrate handoff
task: verify-write-batch
branch: orch/agcli-audit/audit-wallet
agentId: bc-7c47ddbe-9a89-4c75-9576-33fcf42ef94e
runId: run-69718f2f-5516-48a7-aa0c-84f7aadae103
resultStatus: finished
finishedAt: 2026-05-27T13:59:17.019Z
-->

## Verification
verifier-failed

## Target
`audit-wallet` on branch `orch/agcli-audit/audit-wallet`

## Branch
`orch/agcli-audit/audit-wallet`

## Execution
- `git status --short --branch` → on `orch/agcli-audit/audit-wallet`, clean.
- `cargo check --all-targets` → first failed before compile on Cargo 1.83 / Edition 2024 dependency parsing.
- `rustup default stable` → installed Rust/Cargo 1.95.0.
- `cargo check --all-targets` → passed: `Finished dev profile ... in 1m 27s`.
- `cargo build --bin agcli` → passed: `Finished dev profile ... in 1m 44s`.
- `cargo test --no-run --workspace` → passed; compiled workspace tests and only one dependent audit test target: `tests/audit_wallet.rs`.
- `cargo test --no-run --test audit_wallet && cargo test --test audit_wallet` → passed: 42 passed, 0 failed, 1 ignored.
- Checked dependent audit test files → `audit_wallet` present; `audit_balance_transfer`, `audit_stake`, `audit_subnet`, `audit_weights`, `audit_root`, `audit_delegate` missing.
- Inspected docs headings against `src/cli/mod.rs` enums:
  - wallet headings cover all wallet subcommands.
  - transfer docs are missing `balance`.
  - stake docs use `remove-stake-full-limit`, not the actual `remove-full-limit`.
  - subnet, weights, root, delegate headings appear to cover their enum variants.
- Read repo handoffs under `.orchestrate/agcli-audit/handoffs/` → six local handoff files are orchestrate error stubs rather than structured success handoffs; only `audit-subnet.md` is structured locally.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes: passed after switching to stable Rust 1.95.0. (met)
- [x] `cargo build --bin agcli` passes: passed. (met)
- [x] `cargo test --no-run --workspace` succeeds: passed. (met)
- [ ] Each touched docs file enumerates every subcommand: wallet met, but batch criterion not met because `docs/commands/transfer.md` omits `balance`, and `docs/commands/stake.md` does not exactly enumerate `stake remove-full-limit`. (not met)
- [ ] Each `tests/audit_<group>.rs` file parses every subcommand variant: wallet met and tests pass, but six dependent audit files are absent on this branch. (not met)
- [ ] Target handoff has structured Findings: the repo-local `.orchestrate/agcli-audit/handoffs/audit-wallet.md` is an error stub with no `## Findings`; the prompt-supplied upstream handoff does include findings, but the checked-out repo file does not. (not met)

Other findings, severity-ordered:
- (high) This checkout is not a merged synthetic branch containing all worker outputs. Evidence: only `tests/audit_wallet.rs` exists; the other six dependent audit test files are missing.
- (high) Repo-local handoffs conflict with the prompt-supplied upstream summaries. The local wallet, balance-transfer, stake, weights, root, and delegate handoffs are error stubs, so the repo itself does not contain the structured rollup inputs requested.
- (med) Findings rollup from supplied upstream context:
  - wallet: `--hotkey-name` collision; mnemonic CLI guard underdocumented; sign/verify ignore output flags; possible `associate_hotkey` encoding risk; no wallet alias for coldkey swap initiation.
  - balance/transfer: threshold ignored without watch; `--at-block` overrides watch; `transfer` uses `transfer_allow_death`; balance reports free balance only; no `force_transfer` surface.
  - stake: `move` hardcodes same hotkey; some alpha amounts are TAO-scaled; `process-claim` exits 0 on partial failures; write commands ignore JSON; wizard can panic without TTY.
  - subnet: lease call-shape mismatches; identity encoding missing `logo_url`; dissolve UX vs root-gated call drift; missing `register_limit` surface.
  - weights: no batch CLI surface; possible missing `commit_crv3_mechanism_weights`; raw dispatch for mechanism/timelocked paths; writes/status ignore JSON; commit-reveal silently falls back.
  - root: docs previously cited nonexistent `set_root_weights`; `root weights` hardcodes version key; output JSON ignored; root operations split across command groups.
  - delegate: list caps at 50; show ignores output format; take bounds are hardcoded; no named admin surface for min delegate take; increase/decrease rate-limit asymmetry.
- (low) Environment needs Rust newer than the VM default; stable 1.95.0 was sufficient.

## Notes & suggestions
- Overall status should not be marked success: compile/build/test compilation pass, but the merged-batch artifact criteria fail on this branch.
- If the planner expects a synthetic merged branch, rerun verification on that branch after merging/presenting all worker outputs.