<!-- orchestrate handoff
task: verify-xcut-batch
branch: orch/agcli-audit/fix-error-mapping
agentId: bc-068d011b-6cc8-442e-8623-ea1c6fbc2eb1
runId: run-d4eb1272-b770-4be3-8f85-777ab5639da9
resultStatus: finished
finishedAt: 2026-05-27T14:19:30.174Z
-->

## Verification
verifier-failed

## Target
`fix-error-mapping` on branch `orch/agcli-audit/fix-error-mapping`

## Branch
`orch/agcli-audit/fix-error-mapping`

## Execution
- `rustup toolchain install stable && rustup default stable` → upgraded verifier environment from Rust/Cargo 1.83.0 to Rust 1.95.0 / Cargo 1.95.0.
- `cargo check --all-targets` → passed; finished dev profile in 1m30s.
- `cargo build --bin agcli` → passed; finished dev profile in 1m40s.
- `cargo clippy --all-targets -- -D warnings` → failed with clippy diagnostics; output included 53 `error:` lines. One diagnostic was in target-touched `src/error.rs:1719` (`std::io::Error::new(std::io::ErrorKind::Other, ...)` should use `std::io::Error::other`), though that exact line does not appear in the branch diff against `main`.
- `cargo test --no-run --workspace` → passed; compiled all workspace test binaries.
- `cargo test --lib error::tests::` → passed: 151 passed, 0 failed.
- `Glob tests/audit_*.rs` / `Glob **/audit_*.rs` → found 0 files, so the requested 5-file `tests/audit_*.rs` spot-check could not be performed literally.
- `cargo test parse_audit` → passed: 12 audit parse-surface tests passed in `tests/cli_test.rs`.
- `target/debug/agcli utils convert --tao 1` → exit code 12 with validation message `--netuid is required for TAO↔Alpha conversion`.
- `target/debug/agcli view axon` → exit code 2 from clap for missing `--netuid`, not exit code 12.
- `target/debug/agcli diff portfolio` → exit code 2 from clap for missing `--block1/--block2`, not exit code 12.
- `git status --short --branch` → clean working tree on `orch/agcli-audit/fix-error-mapping`.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes: command exited 0. (met)
- [x] `cargo build --bin agcli` passes: command exited 0. (met)
- [ ] `cargo clippy --all-targets -- -D warnings` passes for cross-cutting touched files, best-effort: command exited 101. Most diagnostics appear broad/pre-existing, but clippy did report `src/error.rs:1719`, and the verifier-specific cargo invocation did not pass. (not met)
- [x] `cargo test --no-run --workspace` passes: command exited 0 and compiled all listed test binaries. (met)
- [ ] Spot-check 5 random `tests/audit_*.rs` files for parse-surface coverage: no `tests/audit_*.rs` files exist in this checkout. Closest available audit parse tests were embedded in `tests/cli_test_modules`, and `cargo test parse_audit` passed 12/12. (not met literally / substitute passed)
- [x] Re-run tests exercising `src/error.rs` exit-code classification: `cargo test --lib error::tests::` passed 151/151. (met)
- [ ] Validation errors consistently use exit code 12 end-to-end: non-clap validation path `utils convert --tao 1` returned 12, but clap missing-argument paths still returned 2 for `view axon` and `diff portfolio`. (not met)

Other findings:
- (high) End-to-end clap parse failures still return clap’s exit code 2, despite the scoped requirement that clap parse validation errors use exit code 12. Evidence: `target/debug/agcli view axon` and `target/debug/agcli diff portfolio` both exited 2 before reaching the classifier.
- (high) Required clippy invocation fails under `-D warnings`; this prevents success under the verifier-specific acceptance criteria.
- (med) No literal `tests/audit_*.rs` files exist, so the requested 5-file audit spot-check cannot be satisfied as written. Existing audit parse coverage is in `tests/cli_test_modules/*` and passed via `cargo test parse_audit`.
- (low) Cross-cutting handoff rollup:
  - `fix-error-mapping`: added/expanded `src/error.rs` classification for surfaced pallet dispatch errors, validation/network/timeout codes, generic fallback, and hints; targeted unit tests passed, but end-to-end clap parse exit code remains 2.
  - `fix-events-pretty`: corrected event filter variant sets, subtensor pretty coverage, JSON variant fields, netuid extraction, and integer serialization.
  - `fix-explain-topics`: added built-in explain topics/aliases for multisig, scheduler, drand, safe-mode, swap, evm, contracts, and SS58/H160 guidance.
  - `fix-llm-txt`: updated `docs/llm.txt` and hyperparameter docs for command coverage, exit code docs, wallet safety, admin/raw coverage, and CLI caveats.
  - `fix-chain-queries`: added at-block query variants/cache isolation, fixed crowdloan storage key/decode drift, and pinned metagraph reads.
  - `fix-chain-extrinsics`: fixed identity/proxy/safe-mode/lease/swap/EVM extrinsic call shapes and sudo finalization behavior.
  - `fix-scaffold-localnet`: added scaffold config serialization/unknown-key rejection, renamed localnet status timestamp field, and corrected localnet docs.

## Notes & suggestions
- Verdict is `verifier-failed` because not all required checks passed.
- The `src/error.rs` unit-level classifier behavior is well covered, but clap parse errors appear to bypass the classifier entirely. Follow-up should route clap errors through the same exit-code layer or explicitly configure top-level parse handling to exit 12.
- Future cloud agents should start with Rust stable 1.95+; Rust/Cargo 1.83.0 was too old for this workspace’s dependency graph.