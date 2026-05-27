<!-- orchestrate handoff
task: fix-error-mapping
branch: orch/agcli-audit/fix-error-mapping
agentId: bc-70ff6bc0-84b8-4ec0-925e-ef9c5a5ee28d
runId: run-b3d108d9-196d-4db5-a3ad-a64eae29f375
resultStatus: finished
finishedAt: 2026-05-27T14:02:20.982Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-error-mapping`

## What I did
- Updated `src/error.rs` to tighten and extend exit-code classification and hint coverage.
- Added dispatch-error pallet parsing helpers and a surfaced-pallet allowlist (`subtensor`, `admin-utils`, `commitments`, `crowdloan`, `proxy`, `swap`, `drand`, `utility`, `registry`, `shield` via runtime pallet names) so surfaced pallet dispatch errors map consistently to CHAIN (13).
- Implemented generic fallback behavior for unrecognized dispatch pallet errors: when a dispatch error includes runtime metadata pallet/variant but pallet is not recognized, `classify()` now returns GENERIC (1), preserving the metadata-decoded module+error in the message.
- Normalized validation mappings to exit 12 for the audited cases: batch JSON shape errors, invalid proxy SS58 label, missing diff portfolio address, block-not-found input, swap-sim/axon missing-argument forms, `--netuid is required`, and weight-vector shape/bounds messages.
- Moved client-side slippage guard (`"maximum allowed"`) to VALIDATION (12).
- Added timeout mapping for localnet readiness failures (`"did not become ready after ..."`) to TIMEOUT (15).
- Added/expanded human-readable hints for:
  - `DelegateTxRateLimitExceeded`
  - preimage errors (`AlreadyNoted`, `NotNoted`, `NotAuthorized`, `Requested`, `NotRequested`, `TooBig`)
  - scheduler errors (`Scheduler::NotFound`, `TargetBlockNumberInPast`, `Scheduler::Named`)
  - newly normalized validation cases.
- Expanded `src/error.rs` unit tests to cover the new mappings and fallback behavior (including surfaced vs unsurfaced runtime pallet dispatch handling).

## Measurements
- `cargo check --all-targets`: 101 → 0
- `cargo build --bin agcli`: 0 == 0
- `cargo test --lib error::tests::`: 151 passing, 0 failing == 151 passing, 0 failing

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback
- Rust toolchain in VM was initially too old for dependency resolution (edition 2024 crates). I upgraded to stable (`rustc 1.95.0`, `cargo 1.95.0`) before verification.
- Only `src/error.rs` was modified.

### Handoff Findings → Applied fixes
- **Batch validation errors returning GENERIC instead of VALIDATION** fixed by adding explicit batch-shape validation patterns to exit 12.  
  Source: `.orchestrate/agcli-audit/handoffs/audit-batch.md`
- **`diff portfolio` missing address returned GENERIC** fixed (`"No address provided and no wallet found"` → 12).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-diff.md`
- **`Block N not found` returned GENERIC** fixed (`block ... not found` → 12).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-block.md`
- **Invalid config proxy SS58 label not classified as validation** fixed (`proxy address` → 12).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-config.md`
- **`view swap-sim` / `view axon` missing-arg errors returned GENERIC** fixed (specific messages now 12).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-view.md`
- **`--netuid is required` and `Chain connection required` returned GENERIC** fixed (now 12).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-utils-cli.md`
- **Localnet readiness timeout returned GENERIC** fixed (`did not become ready after` → 15).  
  Source: `.orchestrate/agcli-audit/handoffs/audit-localnet.md`
- **Missing preimage pallet error mapping/hints** added for common preimage errors.  
  Source: `.orchestrate/agcli-audit/handoffs/audit-preimage.md`
- **Missing scheduler pallet error mapping/hints** added for `NotFound`, `TargetBlockNumberInPast`, `Named`.  
  Source: `.orchestrate/agcli-audit/handoffs/audit-scheduler.md`
- **Missing delegate rate-limit hint** added for `DelegateTxRateLimitExceeded`.  
  Source: `.orchestrate/agcli-audit/handoffs/audit-delegate.md`
- **Audit address guidance gap** addressed by adding `audit --address` validation hint path.  
  Source: `.orchestrate/agcli-audit/handoffs/audit-audit-cmd.md`

- Cross-cutting dispatch fallback requirement implemented: unknown runtime pallet dispatch errors now fall through to GENERIC (1) while preserving metadata-decoded `Pallet::Variant` in message text; surfaced pallets still map stably to CHAIN.

## Suggested follow-ups
- Add integration tests at CLI process level (not just `error.rs` unit tests) to assert real exit codes for representative commands (`view`, `diff`, `utils`, `localnet`, `batch`).
- Consider centralizing dispatch error normalization (currently split between `src/chain/mod.rs::format_dispatch_error` and `src/error.rs::classify`) to reduce drift risk.
- If strict clap parse failures must also be exit 12 end-to-end, adjust top-level parse flow to route clap errors through the same classifier path.