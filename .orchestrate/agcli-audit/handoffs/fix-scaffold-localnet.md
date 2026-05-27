<!-- orchestrate handoff
task: fix-scaffold-localnet
branch: orch/agcli-audit/fix-scaffold-localnet
-->

## Status
success

## Branch
`orch/agcli-audit/fix-scaffold-localnet`

## What I did

### `src/scaffold.rs`
- Added `Serialize` to `ScaffoldConfig`, `ChainConfig`, `SubnetConfig`, and `NeuronConfig` — these previously only derived `Deserialize`, making the TOML round-trip impossible.
- Added `#[serde(deny_unknown_fields)]` to all four config types so that a TOML with unexpected keys (e.g. a typo) produces a clear `"unknown field 'xyz'"` error instead of silently ignoring it. (`default` and `deny_unknown_fields` are combined: missing fields get their defaults, extra fields are rejected.)
- Added public `serialize_config(config: &ScaffoldConfig) -> Result<String>` function (symmetric counterpart to `load_config`).
- Updated `load_config` error message to hint at the deny_unknown_fields behavior.
- Added 6 new unit tests:
  - `scaffold_config_toml_round_trip_defaults` — default config serializes → re-parses identically
  - `scaffold_config_toml_round_trip_with_explicit_config` — explicit config round-trips field-by-field
  - `scaffold_config_rejects_unknown_chain_key` — typo in `[chain]` produces error naming the field
  - `scaffold_config_rejects_unknown_subnet_key` — typo in `[[subnet]]` produces error
  - `scaffold_config_rejects_unknown_neuron_key` — typo in `[[subnet.neuron]]` produces error
  - `scaffold_config_rejects_unknown_top_level_key` — unknown top-level key is rejected
- All 22 scaffold unit tests pass.

### `src/localnet.rs`
- Renamed `LocalnetStatus.uptime: Option<String>` → `started_at: Option<String>`. The field held the Docker `StartedAt` ISO 8601 timestamp — not a computed uptime duration. The old name was misleading to agents expecting a "how long has this been running" value. A clarifying doc comment explains that callers compute wall-clock uptime by subtracting `started_at` from now.
- Updated both `status()` construction sites to use `started_at`.

### `docs/commands/localnet.md`
- Removed `"seed"` from the scaffold JSON output example. The `NeuronResult.seed` field has `#[serde(skip)]` so it was never emitted — the docs were aspirational rather than descriptive. Added an explanatory note about the seed formula (`//{name}_sn{netuid}`) so callers can still reconstruct it.
- Updated the `localnet status` JSON schema comment: `uptime` → `started_at`, with a clarifying sentence.

## Measurements
- `cargo check --all-targets`: pass → pass (16s incremental)
- `cargo build --bin agcli`: pass → pass (1m45s)
- `cargo test --lib scaffold`: 16 passing → 22 passing (6 new tests)
- `cargo test --test scaffold_config_test`: 20 passing → 20 passing (no regressions)
- `LOC(src/scaffold.rs)`: 818 → ~930
- `LOC(src/localnet.rs)`: 393 → 397 (field rename + doc comment)
- `LOC(docs/commands/localnet.md)`: 160 → 170

## Verification
unit-test-verified — 22 scaffold lib tests + 20 scaffold_config_test integration tests pass locally. Docker is not available in the cloud-agent VM (per `discovery.md`), so the `agcli localnet scaffold` E2E (3-neuron flow, VFS storage-driver) is **not-verified** via live chain; the structural fix (Serialize, deny_unknown_fields) is verified by unit test.

## Notes, concerns, deviations, findings, thoughts, feedback

### Findings

1. **TOML round-trip was broken (fixed)** — All four scaffold config types (`ScaffoldConfig`, `ChainConfig`, `SubnetConfig`, `NeuronConfig`) derived only `Deserialize`. `serialize_config` would not compile; the `load_config` ↔ `serialize_config` round-trip required adding `Serialize`. Motivated by acceptance criterion 3.

2. **Unknown TOML keys silently ignored (fixed)** — Without `deny_unknown_fields`, a typo like `timout = 60` in `[chain]` was silently ignored (config defaulted), leaving a user wondering why their timeout setting had no effect. Now produces `"unknown field 'timout', expected one of 'image', 'container', 'port', 'start', 'timeout'"`. Motivated by acceptance criterion 3.

3. **`LocalnetStatus.uptime` misnomer (fixed)** — The `uptime` JSON field held the Docker `StartedAt` ISO 8601 timestamp string (e.g. `"2026-05-27T13:22:11.123Z"`), not a duration. Agents consuming `agcli localnet status --output json` expecting a human-readable duration (e.g. `"3h 14m"`) would get an ISO timestamp. Renamed to `started_at` to accurately describe the value. Motivated by acceptance criterion 2 ("honestly reports the current chain state"). This is a JSON schema breaking change for consumers already parsing the `uptime` key — noted here for the planner.

4. **Docs showed `seed` in scaffold JSON output (fixed)** — `NeuronResult` has `#[serde(skip)]` on `seed`, so it was never in the JSON output. The docs example showed `"seed": "//validator1_sn1"` which misled agents into parsing a field that doesn't exist. Fixed with an explanatory note about how to derive the seed manually. Motivated by acceptance criterion 4 ("Default ChainConfig + SubnetConfig values match the docs").

5. **`admin::set_max_weight_limit` missing from runtime (pre-existing, not fixed here)** — `scaffold.rs` calls `admin::set_max_weight_limit(...)` inside `try_admin!`, but `audit-admin` confirmed this dispatchable is absent from the current subtensor runtime. The `try_admin!` macro already handles this gracefully (the `msg.contains("not found")` arm warns and continues), so scaffold does not hard-fail. The warning path is the correct behavior given the pallet state. A future fix should either remove the call or wait for the pallet to restore it.

6. **Scaffold E2E (3-neuron flow) is structurally correct** — The Docker-based E2E test in `tests/localnet_e2e_test.rs` exercises the full flow (start → register → fund → register neurons → assert UIDs). The VFS storage-driver requirement is documented in `discovery.md`. Since Docker is unavailable in this VM, I cannot run the E2E, but the fix path (adding `{"storage-driver":"vfs"}` to `/etc/docker/daemon.json` per discovery.md) is already documented and verified by the planner.

### Default values vs. docs

`ChainConfig::default()` and `SubnetConfig::default()` values match `docs/commands/localnet.md`:
- `ChainConfig`: `image=devnet-ready`, `port=9944`, `start=true`, `timeout=120` ✓
- `SubnetConfig`: `tempo=Some(100)`, `max_allowed_validators=Some(8)`, `min_allowed_weights=Some(1)`, `weights_rate_limit=Some(0)`, `commit_reveal=Some(false)` ✓
- Default neurons: `validator1` (1000 TAO), `miner1` (100 TAO), `miner2` (100 TAO), all `register=true` ✓

No discrepancy found between code defaults and documentation.

## Suggested follow-ups
- **Planner**: Remove `admin::set_max_weight_limit` from the scaffold hyperparams block once the pallet restores that dispatchable, or replace with the equivalent current runtime call.
- **Planner**: The `started_at` JSON key rename is a breaking change for consumers parsing `"uptime"`. If backward compatibility is needed, add `#[serde(alias = "uptime")]` on `started_at` or add a migration note in the changelog.
- **Planner**: The `agcli localnet scaffold` E2E (VFS Docker hack from `discovery.md`) should be run as a final verification once a Docker-capable environment is available.
- **Planner**: `localnet reset` could benefit from a `#[serde(alias = "uptime")]` addition to keep the field readable in contexts where "uptime" was the expected key from older docs.
