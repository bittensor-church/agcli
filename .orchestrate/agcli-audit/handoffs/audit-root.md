<!-- orchestrate handoff
task: audit-root
branch: orch/agcli-audit/audit-root
agentId: bc-c25fbb51-73a3-48ea-bc37-05c9b4bb4342
runId: run-131602e4-b607-42d9-9ec0-fe78f8a5e2fd
resultStatus: finished
finishedAt: 2026-05-27T13:49:10.423Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-root`

## What I did

- **`tests/audit_root.rs`** (new, ~270 LOC):
  - 26 non-ignored tests + 1 `#[ignore]` localnet green-path covering all `RootCommands` variants: `Register` and `Weights`
  - Parse-surface tests: `root register` with no args, with global wallet flags, with `--dry-run`; `root weights` with single pair, multi-pair, max u16, `--output json`, all global flags
  - Rejection tests: missing `--weights` flag, no subcommand, unknown flags
  - `root_commands_variant_coverage`: exhaustive match that fails to compile if a new variant is added without updating this file
  - `parse_weight_pairs` unit tests: valid pairs, single pair, u16 boundary, overflow rejection, UID overflow, missing colon, extra colon, empty string, whitespace trimming, non-numeric inputs, 32-pair stress
  - `net_uid_root_is_zero`: asserts `NetUid::ROOT == 0`
  - 1 `#[ignore]` integration test `green_path_root_localnet` (Docker unavailable in CI)

- **`docs/commands/root.md`** (full rewrite, 22 substantive lines → 280+ lines):
  - Both subcommands documented with clap flags+types, relevant global flags, output format (noting JSON is silently ignored), exit code table (all 7 codes from `src/error.rs`), pallet ref + call index + implementation file, SCALE encoding table per argument, on-chain events emitted, storage key reference
  - Related pallet dispatches table showing `root_dissolve_network` and `claim_root` surface locations
  - 6 concrete findings section; suggested follow-ups section

## Measurements

- `cargo check --all-targets`: pass → pass
- `cargo test --no-run --test audit_root`: compiles
- `cargo test --test audit_root`: 26 passing, 1 ignored
- `LOC(tests/audit_root.rs)`: 0 → ~270
- `LOC(docs/commands/root.md)`: 22 substantive → ~280

## Verification
unit-test-verified (26 non-ignored tests pass; Docker unavailable so localnet test is `#[ignore]`)

## Notes, concerns, deviations, findings, thoughts, feedback

- `RootCommands` is minimal: only 2 variants. The `root`-pallet surface is actually spread across 3 command groups (`root`, `stake`, `subnet`), which makes the CLI non-obvious for agents.
- The subtensor submodule was initialized shallowly to cross-reference pallet dispatches. `SKIP_METADATA_FETCH=1` was required (no live finney RPC available).

## Findings

1. **Previous docs cited `set_root_weights` — a dispatch that does not exist.** `docs/commands/root.md` referenced `SubtensorModule::set_root_weights(origin, netuid, hotkey, dests, weights, version_key)`. No such dispatch exists. The actual on-chain call is `set_weights(origin, netuid=0, dests, weights, version_key)`. There is no dedicated root weight dispatch; root weight-setting is the standard `set_weights` with `netuid=0`.

2. **`--version-key` flag absent from `root weights`; hardcoded to `0`.** The `set_weights` pallet dispatch requires `version_key: u64`. The handler passes `0` unconditionally. If the root subnet's `WeightsVersionKey` storage advances (e.g., after a runtime upgrade that bumps it), all `root weights` submissions will fail with `VersionKeyMismatch` and there is no way to supply the correct key through the CLI.

3. **Neither `root register` nor `root weights` respects `--output json`.** Both handlers write exclusively with `println!` and never inspect `ctx.output`. Agents passing `--output json` receive human-readable text with exit `0`, with no machine-parseable transaction hash.

4. **`root_dissolve_network` uses `Value::u128` for a `NetUid` (u16 newtype) in `submit_raw_call`.** The pallet takes `netuid: NetUid` (newtype of `u16`). The raw call encodes it as `Value::u128(netuid.0 as u128)`. Subxt resolves this from runtime metadata at runtime, so it works in practice — but unlike the typed `api::tx()` path, there is no compile-time verification. A pallet rename or type change would surface only as a runtime `DispatchNotFound` or decode error.

5. **`claim_root` CLI limited to single subnet; pallet supports `BTreeSet<NetUid>` (multi-subnet batch).** `StakeCommands::ClaimRoot` exposes one `--netuid: u16`. The pallet `claim_root` (call index 121) accepts up to `MAX_SUBNET_CLAIMS` entries in a `BTreeSet<NetUid>`. An agent must make N separate transactions to claim dividends from N subnets, whereas the chain supports batching them in one. This is surfaced under `stake claim-root`, not `root`, but is a root-pallet capability gap.

6. **`RootCommands` has only 2 variants; two root-pallet dispatches are surfaced under foreign command groups.** `agcli root` covers `root_register` (call index 62) and `set_weights(netuid=0)`. `root_dissolve_network` (call index 120) lives under `agcli subnet root-dissolve` and `claim_root` (call index 121) lives under `agcli stake claim-root`. There is no `agcli root dissolve` or `agcli root claim` alias, so the `root` command group does not present a complete surface for root-network operations.

## Suggested follow-ups

- **Source** (`network_cmds.rs` / `mod.rs`): Add `--version-key` flag (type `u64`, default `0`) to `RootCommands::Weights` and thread it to `client.set_weights(..., version_key)`.
- **Source** (`network_cmds.rs`): Both `Register` and `Weights` handlers should emit JSON (`{"tx": "0x…", "hotkey": "5…"}`) when `ctx.output.is_json()`.
- **Source** (`extrinsics.rs`): Switch `root_dissolve_network` from `submit_raw_call` with `Value::u128` to the typed `api::tx().subtensor_module().root_dissolve_network(netuid.0)` path.
- **Source** (`stake_cmds.rs` / `mod.rs`): Extend `StakeCommands::ClaimRoot` to accept `--netuids` (comma-separated list) to allow multi-subnet batch claims matching the pallet `BTreeSet<NetUid>` API.
- **Docs** (`root.md`, `stake.md`, `subnet.md`): Add cross-links so agents can discover `agcli stake claim-root` and `agcli subnet root-dissolve` from the `root` docs.