<!-- orchestrate handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-7ba18182-3587-4680-a9aa-e5be0a33010c
runId: run-a9ae4339-0aee-4391-91fe-2167d6edb86a
resultStatus: finished
finishedAt: 2026-05-27T12:59:04.215Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-view`

## What I did

- **`tests/audit_view.rs`** (new, 680 LOC): 53 parse-surface tests covering all 15 `ViewCommands` subcommands (`Portfolio`, `Network`, `Dynamic`, `Neuron`, `Validators`, `History`, `Account`, `SubnetAnalytics`, `StakingAnalytics`, `SwapSim`, `Nominations`, `Metagraph`, `Axon`, `Health`, `Emissions`) plus the top-level `Audit` command. Includes TAO→RAO encoding unit tests, default-value assertions, required-arg failure tests, and one `#[ignore]`-gated integration test against a localnet node.
- **`docs/commands/view.md`** (rewritten, 550 LOC): Every subcommand documented with clap flags + types, read path, pallet storage keys accessed, JSON output schema, CSV column headers, exit codes, and inline audit findings. Includes the `agcli audit` top-level command section (backed by `view_cmds::handle_audit`).

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_view`: compiled → compiled
- `cargo test --test audit_view`: 53 passed, 0 failed, 1 ignored

## Verification

unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

The `--live` global flag is `Option<Option<u64>>` in clap. When passed as `["agcli", "--live", "view", "portfolio"]`, clap greedily consumes `view` as the optional value, causing parse failure. The tests place `--live` after the subcommand to avoid this. This is an actual usability quirk: users who write `agcli --live view portfolio` will get an error.

## Findings

1. **`view neuron` ignores `--output json/csv`**. The handler uses `println!` unconditionally and never checks `ctx.output`. Running `agcli --output json view neuron --netuid 1 --uid 0` produces human-readable text. Every other view subcommand routes through `ctx.output`. This is the most impactful drift — agents relying on JSON output from `view neuron` will receive unparseable text.

2. **`view dynamic` CSV/table column mismatch**. CSV header has 10 fields (`netuid,name,symbol,tempo,price,tao_in_rao,alpha_in,alpha_out,emission,volume`), but the table renderer uses 9 headers (`NetUID, Name, Symbol, Price (τ/α), TAO In, Alpha In, Alpha Out, Emission, Tempo`) — `volume` is present in CSV output but absent from table display. Column order also diverges (`tempo` is column 4 in CSV but the last table header).

3. **`view nominations` silently drops `--output csv`**. The handler checks `if output.is_json()` and returns, then falls through to `println!`-based human-readable output with no `render_rows` call. `--output csv` produces the same table text as `--output table`. The `json` path serializes the raw `Vec<DelegateInfo>` struct correctly, but CSV is unimplemented.

4. **`view history` CSV/table column ordering diverges**. CSV header: `block,hash,module,call,success,timestamp` (6 fields). Table headers: `["Block", "Module", "Call", "Success", "Hash"]` (5 fields — no `timestamp`; `hash` appears second in CSV but last in table). A consumer parsing the CSV output cannot assume columns match the table display.

5. **`view swap-sim` and `view axon` produce exit 1 (GENERIC) for missing-argument errors instead of exit 12 (VALIDATION)**. `handle_swap_sim` calls `anyhow::bail!("Specify either --tao or --alpha…")` and `handle_axon_lookup` calls `anyhow::bail!("Provide either --uid or --hotkey-address")`. Both are missing-argument validation errors that should produce exit 12, consistent with `validate_ss58` and `validate_netuid`.

6. **`--live` global flag has a clap ordering pitfall**. `pub live: Option<Option<u64>>` causes clap to consume the next positional token (e.g. the subcommand name `view`) as the optional value when `--live` appears before the subcommand. `agcli --live view portfolio` fails with "unrecognized subcommand 'portfolio'". Users must write `agcli view portfolio --live` instead. The docs advertise `agcli --live view dynamic` which does not parse correctly.

7. **`view history` depends on external Subscan API with no auth and no rate-limit handling**. When Subscan is unavailable or rate-limits the request, the command exits 0 with a note to stderr. There is no `--api-key` flag, no retry, and no offline fallback. Exit code should be NETWORK (10) when the HTTP request fails, not 0.

## Suggested follow-ups

- **Add JSON/CSV output to `view neuron`** — add a `print_json` branch matching the field set printed by `println!`.
- **Fix `view dynamic` CSV/table column alignment** — either add `volume` as a table column or remove it from the CSV header; make ordering consistent.
- **Add CSV path to `view nominations`** — route through `render_rows` like other subcommands.
- **Fix exit code for `view swap-sim` / `view axon` missing-argument errors** — use `anyhow::bail!` with a message matching the VALIDATION classifier (e.g. `"Invalid …: …"`), or add explicit message patterns to `error::classify`.
- **Fix `--live` ordering in docs** — change `agcli --live view dynamic` examples to `agcli view dynamic --live`; consider updating clap arg definition to use `require_equals` to make the optional-value semantics explicit.
- **Add exit NETWORK (10) for Subscan HTTP failure** in `handle_history` — wrap the `reqwest` error so it propagates through `classify`.