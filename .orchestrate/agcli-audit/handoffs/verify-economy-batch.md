<!-- orchestrate handoff
task: verify-economy-batch
branch: orch/agcli-audit/audit-crowdloan
agentId: bc-39c932ff-3fdd-426a-9d09-87d0da782784
runId: run-061799af-de4a-44fa-be6b-4bd2ac0078e9
resultStatus: finished
finishedAt: 2026-05-27T12:10:07.214Z
-->

## Verification
verifier-failed

## Target
`audit-crowdloan` on branch `orch/agcli-audit/audit-crowdloan`

## Branch
`orch/agcli-audit/audit-crowdloan`

## Execution
- `rustup install stable && rustup default stable && git submodule update --init --depth=1 -- subtensor` → pass; rustc 1.95.0, subtensor `6844ee37f0b8cb02baf9ff8d3ca4319cfb33f361`.
- `cargo check --all-targets` → pass; finished in 1m27s.
- `cargo build --bin agcli` → pass; finished in 1m50s.
- `cargo test --no-run --workspace` → pass; compiled workspace tests, including `tests/audit_crowdloan.rs`; no liquidity/commitment audit targets present.
- `cargo test --no-run --test audit_crowdloan` → pass.
- `cargo test --no-run --test audit_liquidity` → fail: no test target named `audit_liquidity`.
- `cargo test --no-run --test audit_commitment` → fail: no test target named `audit_commitment`.
- `cargo test --test audit_crowdloan parse_surface_all_crowdloan_subcommands` → pass: 1 passed, 0 failed.
- Enum/docs/tests comparison → crowdloan covered all variants; commitment docs headings covered all variants but test file absent; liquidity docs/test coverage absent for `add`, `remove`, `modify`, `toggle`.
- Committed/pushed verifier artifact: `8f64286 Add economy batch verifier handoff`.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes: command exited 0. (met)
- [x] `cargo build --bin agcli` passes: command exited 0. (met)
- [ ] Every dependent worker’s `audit_*.rs` compiles: crowdloan compiles; liquidity and commitment test targets are missing. (not met)
- [ ] Docs enumerate every touched group’s enum variants: crowdloan met; commitment headings met; `docs/commands/swap.md` does not enumerate `LiquidityCommands` variants. (not met)
- [ ] Each audit test parses every subcommand via `Cli::try_parse_from`: crowdloan met; liquidity/commitment tests absent. (not met)
- [x] Verdict handoff consolidates dependent findings. (met)
- [x] Status reflects actual verification result: `verifier-failed`. (met)

Other findings (severity-ordered):
- (high) This checkout is not the requested synthetic merge of crowdloan/liquidity/commitment outputs; only crowdloan docs/tests are present.
- (high) `tests/audit_liquidity.rs` and `tests/audit_commitment.rs` are absent, so batch verification cannot pass.
- (med) `docs/commands/swap.md` still documents swap hotkey/coldkey, not liquidity subcommands.
- (med) Checked-out crowdloan/liquidity handoff files are `resultStatus: error` raw-output stubs, conflicting with the structured upstream handoff context supplied in the prompt.

Findings rollup:
- Crowdloan: contributors storage key drift (`Contributors` vs `Contributions`), crowdloan storage decode drift, missing `call` surface on `create`, inconsistent JSON/text output.
- Liquidity: `add_liquidity` disabled in pallet, `toggle` side effects/events commented out, JSON output ignored, missing `set_fee_rate`/`disable_lp` CLI surfaces, broad numeric SCALE encoding.
- Commitment: missing `set_max_space`/reveal/purge surfaces, limited/truncating `Data` encoding, inconsistent JSON schemas, hotkey vs coldkey terminology risk.

## Notes & suggestions
- Do not mark `verify-economy-batch` successful from this branch.
- Re-run verification on an actual synthetic merge containing all three worker branches.
- The verifier handoff was written to `.orchestrate/agcli-audit/handoffs/verify-economy-batch.md` and pushed.