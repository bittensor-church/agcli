<!-- orchestrate handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-5a6bb3c1-3a19-4fe5-9b52-06d622ade613
runId: run-cdd99ec7-5593-443b-aa87-fbefcd423a09
resultStatus: finished
finishedAt: 2026-05-27T13:01:23.541Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-completions-update`

## What I did

- **`tests/audit_completions_update.rs`** (new, ~290 LOC): 23 parse-surface tests + 1 `#[ignore]` localnet stub covering:
  - `Commands::Completions`: all 4 shells (bash/zsh/fish/powershell) with variant field assertion, invalid shell rejection (clap value_parser rejects at parse time), missing `--shell` rejection, and global `--output json` interop.
  - `Commands::Update`: parse, extra-arg rejection, global `--network` flag interop.
  - `UtilsCommands::Convert`: RAO→TAO, TAO→RAO (`--to-rao`), TAO→Alpha (`--tao + --netuid`), Alpha→TAO (`--alpha + --netuid`), no-flags variant (runtime-only validation).
  - `UtilsCommands::Latency`: defaults (pings=5), `--extra` + `--pings` overrides.
  - Inner `completions_output` and `command_name_audit` submodules for organization.
  - One `#[tokio::test] #[ignore]` green-path stub (Docker unavailable in cloud-agent VM).

- **`docs/commands/utils.md`** (refresh, 54 → 182 lines): Added clap flag tables with types and required/optional status for all four subcommands; exit-code tables for each (`convert`, `latency`, `completions`, `update`); JSON output schema for all output paths; pallet reference notes (all no-chain-write); inline findings notes on latent bugs.

## Measurements

- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_completions_update`: compiles → compiles (exit 0)
- `cargo test --test audit_completions_update`: 23 passed, 0 failed, 1 ignored
- `LOC(tests/audit_completions_update.rs)`: 0 → ~290
- `LOC(docs/commands/utils.md)`: 54 → 182

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings (concrete audit observations)

1. **`completions` silent success on invalid shell (bypassing clap validation).** `generate_completions` has a `_ => { eprintln!("Unsupported shell: …"); return; }` arm that returns `()` without error. Since the command dispatch wraps it as `{ system_cmds::generate_completions(&shell); Ok(()) }`, any shell value that bypassed clap's `value_parser` would silently exit 0 with an error message on stderr only. Clap currently catches invalid values at parse time (exit 2), but the handler itself has no defensive `return Err(…)`. If the `value_parser` constraint were relaxed, this would become a silent no-op.

2. **`completions` produces no JSON output variant — ignores `--output json`.** `generate_completions` always writes raw shell script to stdout regardless of the global `--output` flag. An agent piping output with `--output json` will receive shell script, not JSON. There is no `{"completions": "…"}` wrapper. This is the same issue as `config show` (noted in the upstream `audit-config` handoff), but more expected for a completions command. Still, it should be documented (now done) and potentially detected by the agent layer.

3. **`update` exit code is always GENERIC (1) on failure — no exit-code differentiation.** `handle_update` uses `anyhow::bail!` for both "cargo not found" (IO/NETWORK class) and "cargo install exited non-zero" (could be NETWORK if GitHub is unreachable, or CHAIN-adjacent). `src/error.rs::classify` only sees the bare anyhow message which contains neither `"network"`, `"timeout"`, nor `"auth"` keywords, so all failures map to exit code 1. An agent scripting around `agcli update` cannot distinguish a network failure from a bad build.

4. **`utils convert` docs were wrong: `--tao` flag was undocumented.** The old `docs/commands/utils.md` only showed `agcli utils convert --tao 1.5` and `agcli utils convert --rao 1000000000` (using a non-existent `--rao` flag). The actual clap surface has `--amount` + `--to-rao` for TAO/RAO, and separate `--tao`/`--alpha` flags for subnet Alpha conversion. An agent reading the old docs would construct `agcli utils convert --rao 1000000000` and get a parse error.

5. **`utils convert --amount 0.0` (no flags, zero amount) produces output but gives ambiguous result.** When called with no flags and no `--amount`, the handler defaults to `amount = 0.0` and emits `0 RAO = 0.000000000 TAO`. This is not a crash but could confuse an agent that calls `agcli utils convert` expecting an error about missing required amount. The lack of a required-argument guard is a UX gap (now documented).

6. **No on-chain dispatchable gap:** `completions` and `update` have no subtensor pallet surface (by design — shell completions and GitHub self-update are not on-chain operations). `utils latency` and `utils convert` are read-only. There are no missing pallet dispatchables in scope for this command group.

## Suggested follow-ups

- **Fix `update` exit-code classification**: detect "Failed to run cargo install" as exit code 14 (IO) and `cargo install` network failures as 10 (NETWORK) in `src/error.rs::classify`.
- **Add `completions` JSON wrapper**: when `--output json` is set globally, emit `{"shell": "bash", "script": "…"}` instead of raw script to stdout.
- **Guard `generate_completions` with `Result`**: change signature to `fn generate_completions(shell: &str) -> Result<()>` so the `_` arm returns `Err(…)` rather than silently returning `()`.
- **Fix `utils convert` docs example `--rao` flag**: the old docs used `agcli utils convert --rao 1000000000` — the correct flag is `agcli utils convert --amount 1000000000` (RAO→TAO default). This is fixed in the refreshed docs but may persist in `docs/llm.txt` or other agent-facing references.