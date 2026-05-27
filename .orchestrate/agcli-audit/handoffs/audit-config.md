<!-- orchestrate handoff
task: audit-config
branch: orch/agcli-audit/audit-config
agentId: bc-940890ac-6805-465a-b274-99916e8a6232
runId: run-273e55d1-f273-459b-b9e1-f682aa89baa3
resultStatus: finished
finishedAt: 2026-05-27T12:34:52.457Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-config`

## What I did

- **`tests/audit_config.rs`** (new, 280 LOC): 28 parse-surface + handler tests covering all 6 `ConfigCommands` variants (`Show`, `Set` × 7 keys, `Unset` × 2, `Path`, `CacheClear`, `CacheInfo`); clap required-arg failure tests; `validate_config_network` / `validate_spending_limit` unit tests; `Config` TOML round-trip green-path; disk-cache no-panic probes; global-flag interaction tests; one `#[ignore]` localnet integration stub. 28 pass, 1 ignored.
- **`docs/commands/config.md`** (refresh): Added `cache-clear` and `cache-info` subcommand entries (previously absent); added exit-code tables for every subcommand; documented `finalization_timeout` and `mortality_blocks` as TOML-only (no `config set` key); noted `config show` ignores `--output json`; added pallet-reference section clarifying no on-chain interaction.

## Measurements

- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_config`: compiles → compiles
- `cargo test --test audit_config`: 28 passed, 0 failed, 1 ignored → same
- `LOC(tests/audit_config.rs)`: 0 → 280
- `LOC(docs/commands/config.md)`: 69 → 225

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings (concrete audit observations)

1. **`cache-clear` and `cache-info` were not documented.** Both subcommands have been in `ConfigCommands` since at least the current commit, but `docs/commands/config.md` had no entry for either. Agents querying `agcli explain --topic config --full` would never know these subcommands exist.

2. **`finalization_timeout` and `mortality_blocks` have no `config set` surface.** `Config` struct carries both fields and they survive TOML round-trips. The `handle_config` Set/Unset match arms have no branch for `"finalization_timeout"` or `"mortality_blocks"`, so agents cannot set them via `agcli config set`. The only path is manual TOML editing. This creates a silent capability gap.

3. **`config show` ignores `--output json`.** The handler calls `toml::to_string_pretty(&cfg)` and prints it unconditionally regardless of the global `--output` flag. An agent running with `--output json` will receive TOML text, not a JSON object, breaking any downstream JSON parsing. Every other command group respects `output.is_json()`.

4. **`validate_config_network` does not accept custom URLs.** A user who sets `endpoint` via `config set --key endpoint --value wss://...` correctly uses the `endpoint` key, but if they mistakenly try `config set --key network --value wss://...` they get a misleading error ("Unknown network …"). The docs previously only mentioned finney/test/local/archive without explaining that custom endpoints must use the `endpoint` key. This is now clarified in the docs.

5. **No `config set` key for `proxy` validation message mismatch.** `validate_ss58` is called with label `"proxy address"`, but the error classify heuristic in `src/error.rs` matches `"invalid destination"` and `"stake list --address"` — it does NOT match `"proxy address"`, so a bad `--key proxy --value <bad-ss58>` call returns exit code `1` (GENERIC) rather than `12` (VALIDATION). This is a minor scripting gotcha but not a blocker.

### Environment notes
- Docker not available in cloud-agent VM; `#[ignore]` integration test left as stub per discovery.md guidance.
- Rust 1.95.0 (stable) installed via `rustup`; was 1.83 at start.

## Suggested follow-ups

- **Fix `config show` to respect `--output json`**: wrap output in `{"config": { ... }}` JSON when `output.is_json()`, using `serde_json::to_value(&cfg)` rather than `toml::to_string_pretty`.
- **Add `config set` arms for `finalization_timeout` and `mortality_blocks`**: both are typed `u64`, same pattern as `live_interval`.
- **Fix exit code for bad `proxy` value in `config set`**: add `"proxy address"` to the VALIDATION heuristic in `src/error.rs::classify`.
- **`config unset` unknown-key error** is a bare `anyhow::bail!("Unknown config key '{}'")` without a hint listing valid keys — inconsistent with the `config set` error which does list valid keys. Low severity but worth fixing for agent UX.