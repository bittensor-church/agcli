<!-- orchestrate handoff
task: fix-chain-queries
branch: orch/agcli-audit/fix-chain-queries
agentId: bc-daaa9a14-269b-4bac-b6d0-1317f3403fe8
runId: run-48069901-b395-4051-9d4c-fc3e41ae2ee5
resultStatus: finished
finishedAt: 2026-05-27T14:03:21.187Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-chain-queries`

## What I did
- Updated `src/chain/queries.rs` to close read/query gaps:
  - Added explicit `get_*_at_block(block_hash)` variants (with existing `*_pinned` aliases preserved) for:
    - subnet info/hyperparams
    - delegate
    - subnet identity
    - proxies
    - coldkey swap announcement
    - child keys / pending child keys
  - Routed expensive historical reads through block-hash keyed cache paths:
    - `get_all_subnets_at_block`
    - `get_all_dynamic_info_at_block`
    - `get_neurons_lite_at_block`
  - Fixed crowdloan contributors storage key drift: `Contributors` → `Contributions`.
  - Hardened crowdloan storage decode to support both legacy tuple and current object/tuple layouts and normalized account/number extraction.
- Updated `src/queries/query_cache.rs`:
  - Added immutable at-block caches keyed by block hash (or block+netuid), separate from latest TTL caches.
- Updated `src/queries/disk_cache.rs`:
  - Added immutable-key handling for `atblock:` cache keys so pinned snapshots do not expire under latest-data TTL behavior.
- Updated `src/queries/metagraph.rs`:
  - Switched metagraph fetch to pinned block hash + `get_neurons_lite_at_block`.
  - Reused/saved exact block snapshots (`queries::cache::load_block`/`save`) to avoid stale joins.
- Added worker handoff doc:
  - `.orchestrate/agcli-audit/handoffs/fix-chain-queries.md` with fix mapping to upstream audit handoffs.

## Measurements
- `cargo check --all-targets exit code: 0 == 0`
- `cargo build --bin agcli exit code: 0 == 0`
- `git diff --stat (HEAD): 5 files changed, 625 insertions(+), 104 deletions(-)`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- Rust/Cargo in the VM was initially too old (Cargo 1.83) for current dependency requirements (`edition2024` crates). I upgraded toolchain to stable `rustc 1.95.0` / `cargo 1.95.0` before running required verification commands.
- Handoff Findings (fix → motivating audit handoff):
  - Crowdloan contributors key fix (`Contributors` → `Contributions`) → `.orchestrate/agcli-audit/handoffs/audit-crowdloan.md`
  - Crowdloan decode drift hardening (layout mismatch tolerance) → `.orchestrate/agcli-audit/handoffs/audit-crowdloan.md`
  - Pinned-read staleness fix in metagraph flow (block-consistent reads) → `.orchestrate/agcli-audit/handoffs/audit-audit-cmd.md`
  - Added explicit `get_*_at_block` parity on pinned-only reads for diff/at-block surface consistency → `.orchestrate/agcli-audit/handoffs/audit-block.md`, `.orchestrate/agcli-audit/handoffs/audit-diff.md` plus this worker scope
  - Cache isolation for at-block queries (no latest TTL bleed) → this worker’s cache acceptance criteria (implemented across `query_cache.rs` + `disk_cache.rs`)

## Suggested follow-ups
- Add targeted unit tests for:
  - crowdloan decode helper across legacy/current storage shapes
  - at-block cache key isolation and immutable disk behavior for `atblock:` keys
- Consider migrating remaining historical reads that still hit direct chain calls repeatedly to use the new block-keyed cache helpers when they are used in multi-read command paths.