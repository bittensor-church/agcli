## Verification
verifier-failed

## Target
`audit-crowdloan` on branch `orch/agcli-audit/audit-crowdloan`

## Branch
`orch/agcli-audit/audit-crowdloan`

## Execution
- `rustup install stable && rustup default stable && git submodule update --init --depth=1 -- subtensor` -> pass; installed/selected rustc 1.95.0 and checked out subtensor `6844ee37f0b8cb02baf9ff8d3ca4319cfb33f361`.
- `cargo check --all-targets` -> pass; finished dev profile in 1m27s.
- `cargo build --bin agcli` -> pass; finished dev profile in 1m50s.
- `cargo test --no-run --workspace` -> pass; compiled workspace test targets, including `tests/audit_crowdloan.rs`, but no `audit_liquidity` or `audit_commitment` targets were present.
- `cargo test --no-run --test audit_crowdloan` -> pass; compiled `tests/audit_crowdloan.rs`.
- `cargo test --no-run --test audit_liquidity` -> fail; Cargo reported `error: no test target named 'audit_liquidity'`.
- `cargo test --no-run --test audit_commitment` -> fail; Cargo reported `error: no test target named 'audit_commitment'`.
- `cargo test --test audit_crowdloan parse_surface_all_crowdloan_subcommands` -> pass; 1 passed, 0 failed, 0 ignored, 1 filtered out.
- Enum/docs/tests comparison script over `CrowdloanCommands`, `LiquidityCommands`, and `CommitmentCommands` -> crowdloan docs/test covered all variants; `docs/commands/swap.md` had no headings for `add`, `remove`, `modify`, or `toggle`; `tests/audit_liquidity.rs` and `tests/audit_commitment.rs` were absent; commitment docs headings covered `set`, `get`, and `list`.
- Read `.orchestrate/agcli-audit/handoffs/audit-crowdloan.md`, `audit-liquidity.md`, and `audit-commitment.md` -> the checked-out crowdloan and liquidity handoff files are `resultStatus: error` raw-output stubs with no structured Findings; the checked-out commitment handoff is structured success. The findings rollup below also incorporates the upstream structured handoff content supplied to this verifier.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes on the merged branch: command exited 0 on the actual checked-out branch. (met)
- [x] `cargo build --bin agcli` passes on the merged branch: command exited 0. (met)
- [ ] Every dependent worker's `audit_*.rs` test file compiles: `tests/audit_crowdloan.rs` compiled, but `cargo test --no-run --test audit_liquidity` and `cargo test --no-run --test audit_commitment` both failed because the test targets do not exist in this checkout. (not met)
- [ ] Each touched `docs/commands/<group>.md` enumerates every subcommand by matching `*Commands` variants against markdown headings: `docs/commands/crowdloan.md` covered all 12 crowdloan variants and `docs/commands/commitment.md` covered all 3 commitment variants; `docs/commands/swap.md` did not contain headings for the 4 `LiquidityCommands` variants (`add`, `remove`, `modify`, `toggle`). (not met)
- [ ] Each `tests/audit_<group>.rs` file at least parses every subcommand variant via `Cli::try_parse_from`: crowdloan passed via the targeted parse-surface test; liquidity and commitment could not be verified because their audit test files are absent. (not met)
- [x] Verdict handoff consolidates the dependent workers' Findings into a single rollup: see "Findings rollup" below. (met)
- [x] Verdict handoff Status reflects the actual pass/fail of the cargo checks: `verifier-failed`, because dependent-worker acceptance criteria failed despite cargo check/build passing. (met)

Other findings (severity-ordered):
- (high) The checkout is not the requested merged synthetic branch for `audit-crowdloan`, `audit-liquidity`, and `audit-commitment`: `git diff main...HEAD` shows only `docs/commands/crowdloan.md`, `tests/audit_crowdloan.rs`, and orchestrate metadata; `tests/audit_liquidity.rs` and `tests/audit_commitment.rs` are absent.
- (high) Liquidity and commitment audit test targets are missing, so the batch does not satisfy the verifier-specific requirement that every dependent worker's audit file compiles.
- (med) `docs/commands/swap.md` is still the swap hotkey/coldkey page in this checkout and does not enumerate `LiquidityCommands::{Add, Remove, Modify, Toggle}` as markdown headings.
- (med) The checked-out `.orchestrate` handoffs for `audit-crowdloan` and `audit-liquidity` are raw `resultStatus: error` stubs rather than structured worker handoffs. That conflicts with the upstream structured handoff context supplied to this verifier.
- (low) The ignored localnet green-path in `tests/audit_crowdloan.rs` was not executed; only its compilation and the parse-surface test were verified.

Findings rollup from dependent workers:
- Crowdloan:
  - `get_crowdloan_contributors` reportedly uses storage key `Contributors`, while the pallet storage is `Contributions`, so the contributors read path is likely drifted.
  - `list_crowdloans` / `get_crowdloan_info` reportedly decode `Crowdloan::Crowdloans` into a tuple shape that does not match the current `CrowdloanInfo` layout, including `funds_account` and `contributors_count`.
  - `crowdloan create` hardcodes `call = None`, so the pallet's optional `call` parameter is not exposed.
  - Crowdloan write subcommands and `info` have inconsistent output formatting and do not consistently honor `--output json`.
- Liquidity:
  - `agcli liquidity add` reportedly cannot currently succeed on-chain because `Swap::add_liquidity` returns `UserLiquidityDisabled` and its functional body is commented out.
  - `agcli liquidity toggle` reportedly dispatches but pallet state mutation and event emission are commented out, so a successful tx may have no observable state/event effect.
  - Liquidity command output reportedly ignores global JSON formatting and always prints plain text.
  - Swap pallet dispatchables `set_fee_rate` and `disable_lp` reportedly have no `LiquidityCommands` surface.
  - Agcli liquidity SCALE mapping reportedly uses broad dynamic numeric values for narrower pallet types.
- Commitment:
  - Commitment pallet functions `set_max_space`, `reveal_timelocked_commitments`, and `purge_netuid` are not exposed under `CommitmentCommands`.
  - `Client::set_commitment` only emits comma-split `RawN` data and silently truncates fields to 128 bytes, while pallet `Data` supports richer variants.
  - `commitment get --output json` can return incompatible object shapes, and `commitment list --output json` returns a bare array without `netuid` context.
  - CLI terminology can confuse hotkey vs signer semantics: `get` uses `--hotkey-address`, while `set` signs with the wallet coldkey in `handle_commitment`.

## Notes & suggestions
- Planner should not mark `verify-economy-batch` successful from this branch. The core Rust commands pass, but the branch lacks the liquidity and commitment worker outputs required for the batch.
- Re-run verification on an actual synthetic merge containing `orch/agcli-audit/audit-crowdloan`, `orch/agcli-audit/audit-liquidity`, and `orch/agcli-audit/audit-commitment`, then repeat the same cargo/test/docs checks.
- If the planner expects the repository handoff files to be authoritative, regenerate or replace the `audit-crowdloan` and `audit-liquidity` handoffs; the checked-out files do not contain the structured Findings sections present in the upstream verifier prompt.
