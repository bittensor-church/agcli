<!-- orchestrate handoff
task: audit-delegate
branch: orch/agcli-audit/audit-delegate
agentId: bc-84699bd8-e9be-4d06-8a56-86318f88abb7
runId: run-c470d436-d743-4445-90bd-ecba7dd8b75e
resultStatus: finished
finishedAt: 2026-05-27T12:34:36.635Z
-->

Branch pushed successfully. Here is the handoff:

---

## Status
success

## Branch
`orch/agcli-audit/audit-delegate`

## What I did
- **`tests/audit_delegate.rs`** (new, 213 LOC): 15 parse-surface tests covering all 4 subcommands (`list`, `show`, `decrease-take`, `increase-take`), argument-encoding unit tests for the `u16` SCALE formula, `validate_delegate_take` boundary tests, error-classification tests for `DelegateTakeTooLow`, `DelegateTakeTooHigh`, `DelegateTxRateLimitExceeded`, and `NonAssociatedColdKey`. One `#[ignore]` green-path integration test using `Client::connect` against localnet.
- **`docs/commands/delegate.md`** (rewritten, 188 LOC): Full documentation for all 4 subcommands with clap flags + types, SCALE encoding formula, call indices (65/66), events (`TakeDecreased`, `TakeIncreased`), per-error exit codes, all relevant storage keys, admin-utils dispatchable gap, and source locations table.

## Measurements
- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_delegate`: compiled → compiled
- `cargo test --test audit_delegate`: 15 passed, 0 failed, 1 ignored

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings

1. **`delegate list` hard-caps at 50 delegates with no `--limit` flag.** `handle_delegate` calls `.into_iter().take(50)` unconditionally. On mainnet there are >200 registered delegates; callers cannot page through them or retrieve the full set without raw chain queries. `--limit` or `--all` flag is missing.

2. **`delegate show` ignores `--output` format flag.** All output is printed via `println!` instead of the `render_rows` / `ctx.output` path. Running `agcli delegate show --output json` produces the same human-readable text as the table view; `--output csv` is also silently ignored. Every other command in the group routes through `render_rows` or serialises to `ctx.output`.

3. **`DelegateTxRateLimitExceeded` has no explicit hint in `src/error.rs`.** The pallet error is produced whenever `increase_take` is called within the `TxDelegateTakeRateLimit` window (~300 blocks). `error::classify` returns `exit_code::CHAIN` (13) via the generic "Dispatch error:" branch, which is correct, but there is no human-readable hint message (no `hint(exit_code::CHAIN, "DelegateTxRateLimitExceeded")` entry), so the user sees the raw pallet error string with no guidance on when to retry.

4. **`validate_delegate_take` hardcodes the 18% upper bound but `MaxDelegateTake` is a mutable on-chain storage value.** `admin-utils` can call `sudo_set_min_delegate_take` to change the minimum; `MaxDelegateTake` defaults to `InitialDefaultDelegateTake` (18%) but can also be altered. If the chain admin lowers `MaxDelegateTake` below 18%, the CLI will happily accept the value, submit the extrinsic, and receive a `DelegateTakeTooHigh` chain rejection — the pre-flight gate would have already passed. The CLI should query `MaxDelegateTake` from the chain before validating, or at minimum document that the local 18% cap may lag the chain.

5. **No `agcli delegate` surface for `sudo_set_min_delegate_take` (admin-utils pallet).** The admin-utils pallet exposes `sudo_set_min_delegate_take(take: u16)` to set the chain-wide minimum take floor, but there is no dedicated `agcli admin set-min-delegate-take` subcommand. It is reachable only via the generic `agcli admin raw --call sudo_set_min_delegate_take`, which requires the caller to know the raw call name and construct the args JSON manually.

6. **`decrease_take` pallet does NOT enforce the rate limit; `increase_take` does.** The discovery doc and CLI comments do not call this out. The `change_take` function in `network_cmds.rs` shares a single `change_take` helper for both directions, but the rate-limit only applies on-chain to `increase_take`. This asymmetry is invisible to the CLI user and is not documented.

7. **`take_encoding_boundary_values` test reveals a rounding discrepancy at 10%.** `(10.0 / 100.0 * 65535.0).round()` = 6553.5 → rounds to 6554, not 6553. The CLI rounds and the pallet checks only `take > current` / `take < current` so no correctness issue exists, but the formula comment in `change_take` says "round" without specifying IEEE round-half-to-even vs round-half-up. Documented in the test.

## Suggested follow-ups
- **Add `--limit` flag to `delegate list`** (or remove the 50-cap entirely) to expose all registered delegates.
- **Route `delegate show` through `ctx.output`** to respect `--output json/csv`.
- **Add `hint(exit_code::CHAIN, "DelegateTxRateLimitExceeded")` to `src/error.rs`** with a message like "Rate limit exceeded for increase_take — wait ~300 blocks and retry."
- **Query `MaxDelegateTake` / `MinDelegateTake` from chain** at runtime instead of hardcoding in `validate_delegate_take`; or re-validate after chain rejection and surface the actual bound in the error message.
- **Add `agcli admin set-min-delegate-take --take <u16>`** as a named subcommand wrapping `sudo_set_min_delegate_take`.
- **Document the rate-limit asymmetry** between `decrease-take` (no limit) and `increase-take` (rate-limited) in help text and docs.