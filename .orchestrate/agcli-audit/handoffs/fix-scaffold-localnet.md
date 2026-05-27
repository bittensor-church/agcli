<!-- orchestrate handoff
task: fix-scaffold-localnet
branch: orch/agcli-audit/fix-scaffold-localnet
agentId: bc-5e435f81-e175-44aa-8b45-d3a38c04a824
runId: run-c96a4e29-b0dc-4ecf-a3d4-07f7a3970916
resultStatus: finished
finishedAt: 2026-05-27T14:06:01.154Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-scaffold-localnet`

## What I did

**`src/scaffold.rs`** — 4 fixes:
- Added `Serialize` to all config types (`ScaffoldConfig`, `ChainConfig`, `SubnetConfig`, `NeuronConfig`) enabling TOML round-trip
- Added `#[serde(deny_unknown_fields)]` to all four types so unknown TOML keys produce a clear named-field error instead of silent discard
- Added public `serialize_config()` function as the symmetric counterpart to `load_config()`
- Added 6 new unit tests: 2 round-trip and 4 unknown-key rejection tests (all pass)

**`src/localnet.rs`** — 1 fix:
- Renamed `LocalnetStatus.uptime` → `started_at` (the field held the Docker `StartedAt` ISO 8601 timestamp, not a computed duration — the old name misled agents)

**`docs/commands/localnet.md`** — 2 fixes:
- Removed `seed` from the scaffold JSON output example (`NeuronResult.seed` has `#[serde(skip)]` so it never appears in output)
- Updated `localnet status` JSON schema comment from `uptime` → `started_at` with explanatory note

**Verification**: `cargo check --all-targets` ✓, `cargo build --bin agcli` ✓, 22/22 scaffold lib tests pass, 20/20 scaffold_config_test integration tests pass. Docker unavailable in VM so scaffold E2E is not-verified.