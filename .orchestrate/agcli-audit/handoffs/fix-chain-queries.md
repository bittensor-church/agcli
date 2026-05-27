<!-- orchestrate handoff
task: fix-chain-queries
branch: orch/agcli-audit/fix-chain-queries
resultStatus: finished
-->

## Status
success

## Branch
`orch/agcli-audit/fix-chain-queries`

## What I did
- Updated `src/chain/queries.rs` to close read-surface and decode gaps:
  - added explicit `get_*_at_block(block_hash)` variants (with pinned aliases preserved) for subnet/delegate/identity/proxy/coldkey-swap/child-key read paths.
  - wired expensive at-block reads (`get_all_subnets_at_block`, `get_all_dynamic_info_at_block`, `get_neurons_lite_at_block`) through block-hash keyed cache entries instead of latest TTL entries.
  - fixed crowdloan contributor storage key from `Contributors` to `Contributions`.
  - replaced fragile crowdloan tuple decoding with metadata-value decoding that accepts both object and tuple layouts and normalizes account/number extraction.
- Updated `src/queries/query_cache.rs`:
  - added immutable block-hash keyed caches for subnets, dynamic info, and neurons-lite (`*_at_block` cache methods).
  - ensured at-block cache paths are isolated from latest TTL cache keys.
- Updated `src/queries/disk_cache.rs`:
  - treated `atblock:` keys as immutable snapshots so TTL expiry is skipped for pinned block data.
- Updated `src/queries/metagraph.rs`:
  - pinned to a single block hash and read neurons via `get_neurons_lite_at_block`.
  - reused/saved block-specific metagraph snapshots to avoid stale latest-cache joins.

## Handoff Findings
- **Fix: crowdloan contributors key drift (`Contributors` → `Contributions`)**  
  Motivated by `.orchestrate/agcli-audit/handoffs/audit-crowdloan.md`.
- **Fix: crowdloan decode drift (tuple layout mismatch) via robust dynamic value decoding**  
  Motivated by `.orchestrate/agcli-audit/handoffs/audit-crowdloan.md`.
- **Fix: stale pinned metagraph reads (pinned block number + latest cached neurons mismatch)**  
  Motivated by stale snapshot concerns in `.orchestrate/agcli-audit/handoffs/audit-audit-cmd.md` (mixed-block reads) and the cross-cutting at-block scope for this task.
- **Fix: missing explicit `get_*_at_block` coverage for pinned read methods**  
  Motivated by the worker scope requirement for full latest/at-block parity and by at-block/diff command audits (notably `.orchestrate/agcli-audit/handoffs/audit-block.md` and `.orchestrate/agcli-audit/handoffs/audit-diff.md`).
- **Fix: cache layer separation for pinned reads (no TTL/latest bleed into at-block paths)**  
  Motivated by this worker’s cache acceptance criteria and stale-data concerns raised across audit handoffs.

