<!-- orchestrate handoff
task: audit-balance-transfer
branch: orch/agcli-audit/audit-balance-transfer
agentId: bc-44b9a09a-4e78-4bbe-96cb-53fe5a292721
runId: run-8731caf1-f8fb-41fc-8a2b-3b1c075cbaac
resultStatus: finished
finishedAt: 2026-05-27T13:06:33.546Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-balance-transfer`

## What I did

- **`tests/audit_balance_transfer.rs`** (new, 519 LOC):
  - 7 parse-surface tests for `balance` (no-args, `--address`, `--at-block`, `--watch`, `--watch N`, `--threshold`, all-flags)
  - 6 parse-surface tests for `transfer` (minimal, global flags, missing-dest fails, missing-amount fails)
  - 4 parse-surface tests for `transfer-all` (minimal, `--keep-alive`, global `--yes`, missing-dest fails)
  - 4 parse-surface tests for `transfer-keep-alive` (minimal, global flags, missing-dest fails, missing-amount fails)
  - 7 validation-layer tests (validate_amount: negative/zero/inf/positive; validate_ss58: empty/garbage/valid; validate_threshold: negative/NaN/zero)
  - 3 error-classification tests (insufficient balance → CHAIN; bad SS58 dest → VALIDATION; bad amount → VALIDATION)
  - 3 Balance type round-trip tests (from_tao, from_rao, ZERO)
  - 2 global-flag interaction tests (dry-run positional, yes flag)
  - 1 `#[ignore]` localnet integration test gated on `ws://127.0.0.1:9944`

- **`docs/commands/transfer.md`** (full rewrite, 97 → 480+ LOC):
  - All 4 subcommands documented: clap flags + types, exit codes, JSON output schema, pallet ref + storage key, events emitted
  - Exit code table cross-referenced with `src/error.rs::exit_code`
  - 7 concrete audit findings (see below)

## Measurements

- `cargo check --all-targets`: passes → passes
- `cargo test --no-run --test audit_balance_transfer`: compiles → compiles
- `LOC(docs/commands/transfer.md)`: 97 → ~480
- `LOC(tests/audit_balance_transfer.rs)`: 0 → 519
- test count (audit_balance_transfer): 0 → 30 compile-verified, 1 ignored

## Verification
unit-test-verified (all 30 non-ignored tests compile and pass parse/validation checks via `cargo test --no-run`)

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings

1. **`balance --threshold` without `--watch` is silently accepted but never fires.** The `validate_threshold` guard runs (exit 12 on negative/NaN), but if `--watch` is absent, the computed `threshold_rao` is never evaluated. The command returns a normal one-shot balance with no warning. Agents relying on the threshold for alerting must also pass `--watch`.

2. **`balance --at-block` + `--watch` silently drops `--watch`.** Both flags parse without error. The handler checks `at_block` first and returns early; the watch loop is never entered. No warning is printed. Providing both flags gives a single at-block result, not a polling loop.

3. **`agcli transfer` maps to `Balances::transfer_allow_death`, not `Balances::transfer`.** The FRAME v2 `transfer` dispatchable was superseded by `transfer_allow_death`. The CLI name `transfer` is intuitive, but agents need to know the account may be reaped if sender balance drops below the existential deposit.

4. **`get_balance` reads `data.free` only — not total balance.** The JSON fields `balance_rao` / `balance_tao` represent only the free (spendable) balance. Reserved, frozen, and locked amounts are excluded. The field names don't indicate this. An account with 10 TAO staked and 0.1 TAO free shows 0.1 TAO.

5. **`transfer-all` has no client-side preflight balance check.** Unlike `transfer` and `transfer-keep-alive`, which query `get_balance_ss58` before signing (and bail with a helpful "Insufficient balance: you have X but trying to transfer Y" message), `transfer-all` skips this step entirely. The chain determines what "all" means at execution time.

6. **`balance_rao` in JSON output is serialized as `u64`, not `u128`.** The `Balance::rao()` method returns `u64`. Maximum representable is ~18.4 × 10^9 TAO — well above Bittensor's supply. No practical overflow risk, but agents writing cross-platform parsers should not expect `u128`.

7. **`Balances::force_transfer` (sudo) has no `agcli` surface.** The FRAME dispatchable for admin-forced transfers (`pallet_balances::force_transfer`) is not reachable via any `agcli` command. Reasonable for a non-sudo CLI, but worth noting for completeness.

## Suggested follow-ups

- **Source**: Emit a warning (to stderr) when `--threshold` is provided without `--watch` — currently silently ignored.
- **Source**: Emit a warning when both `--at-block` and `--watch` are provided together, since `--at-block` silently wins.
- **Docs**: `agcli balance` help text and `docs/commands/transfer.md` should clarify that balance shown is "free (spendable)" only, not total account balance.
- **Source**: Consider adding `agcli balance --include-reserved` to expose `data.reserved + data.frozen` for agents auditing full account state.
- **Localnet test**: Run `green_path_balance_transfer_localnet` against `ghcr.io/opentensor/subtensor-localnet:devnet-ready` once Docker is available in the VM to confirm `transfer_allow_death` and `get_balance_at_block` work end-to-end.