<!-- orchestrate handoff
task: audit-block
branch: orch/agcli-audit/audit-block
agentId: bc-a3e5ec9c-89e6-406d-83f5-7c2842099444
runId: run-6bb18a8d-b2be-4e93-a4d7-785146e2e050
resultStatus: finished
finishedAt: 2026-05-27T13:18:29.657Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-block`

## What I did

- **`tests/audit_block.rs`** (new, ~460 LOC): 33 parse-surface tests covering all 3 `BlockCommands` subcommands (`info`, `latest`, `range`) and all 4 `DiffCommands` subcommands (`portfolio`, `subnet`, `network`, `metagraph`). Tests include: variant matching, required-flag rejection, u32 boundary checks (zero, max, overflow), unknown-flag rejection, global-flag threading (`--yes`, `--wallet`, `--network`, `--output`), range-guard arithmetic, a compile-time guard documenting the `get_block_header` 4-vs-5-field doc drift, and a documentation test recording the `block latest` best-vs-finalized head discrepancy. One `#[ignore]`-gated `green_path_block` tokio test for localnet (Docker unavailable in VM).

- **`docs/commands/block.md`** (full rewrite, ~250 LOC): Every subcommand documented with clap flags + types, exit codes cross-referenced to `src/error.rs`, output JSON schema, pallet/storage key refs, on-chain events (none — all read-only), a coverage table for all 7 subcommands, and all 6 findings with source citations.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_block`: exit 0 (compiles)
- `cargo test --test audit_block -- --skip green_path_block`: 33 passed, 0 failed, 1 filtered (ignored)

## Verification
`unit-test-verified` — 33 parse-surface tests pass. Localnet integration test is `#[ignore]`-gated (Docker unavailable).

## Findings

1. **`block latest` calls `get_block_number()` (best / non-finalized) but its clap doc-comment says "latest **finalized** block".** The handler should use `get_finalized_block_number()` (via `chain_getFinalizedHead`) to match the documented behavior. On mainnet the best head typically runs 2–4 blocks ahead of the finalized head, causing `block latest` to report a block that is not yet consensus-safe.

2. **`get_block_header` doc comment lists `extrinsics_root` as a returned field but the return type is `(u32, H256, H256, H256)` — 4 fields, not 5.** The doc comment (`/// Get block header (number, hash, parent_hash, extrinsics_root, state_root)`) is wrong; `extrinsics_root` is silently dropped and never surfaced in `block info` JSON output. The standard Substrate header includes this field and it is useful for Merkle verification.

3. **`block range` JSON output uses a locale-formatted timestamp string (`"%Y-%m-%d %H:%M:%S"`) while `block info` and `block latest` use RFC 3339 (`dt.to_rfc3339()`).** The inconsistency makes machine parsing unreliable — a consumer cannot deserialize `BlockRow.timestamp` as a `DateTime` using the same logic it uses for the other two commands.

4. **Missing timestamp serializes as `""` (empty string) in `block range` JSON rows, not `null`.** When `get_block_timestamp` returns `None`, the `BlockRow::timestamp` field is an empty string. Consumers cannot distinguish "no timestamp returned by node" from a formatting failure.

5. **Hash fields are serialized with `format!("{:?}", hash)` (Rust Debug format) rather than a stable display representation.** In all three `BlockCommands` handlers and in `BlockRow`, hashes are formatted via Debug. The output (`0x…` lowercase hex) is stable today in subxt, but `Debug` provides no stability guarantee across subxt major versions. `format!("{}", hash)` or `hex::encode(hash.0)` would be safer.

6. **"Block N not found" error (when `chain_getBlockHash` returns `None`) classifies as exit code 1 (GENERIC) instead of 12 (VALIDATION).** From a scripting perspective a non-existent block number is a user-input error (like an invalid address) and should map to VALIDATION. The pruned-block hint logic in `annotate_at_block_error` only fires on *storage* reads, not on the initial hash lookup.

## Notes, concerns, deviations, findings, thoughts, feedback

- The `block` command group is entirely read-only; there are no extrinsics to audit against SCALE encoding. The audit focused on read-path accuracy, output correctness, and documentation fidelity.
- The `DiffCommands` variants are handled in `src/cli/block_cmds.rs` (`handle_diff`) but are exposed as a separate top-level `diff` command. The discovery doc assigns both to the block worker, and all four diff subcommands are covered.
- Docker was unavailable in this VM; the `green_path_block` integration test is `#[ignore]`-gated accordingly.
- The subtensor submodule was not initialized, but the block commands use only standard Substrate system/timestamp pallets — no subtensor-specific storage — so pallet cross-referencing was straightforward.

## Suggested follow-ups

- **Planner:** Fix `BlockCommands::Latest` to call `get_finalized_block_number()` instead of `get_block_number()` in `src/cli/block_cmds.rs`. The method exists on `Client` and is documented as the correct choice when correctness matters.
- **Planner:** Fix `get_block_header` in `src/chain/queries.rs` to return `(u32, H256, H256, H256, H256)` with `extrinsics_root` as the fourth field and surface it in `block info` JSON output.
- **Planner:** Normalize `BlockRow::timestamp` in `src/cli/block_cmds.rs` to RFC 3339 (use `dt.to_rfc3339()`) and serialize missing timestamps as `null` via `Option<String>`.
- **Planner:** Replace `format!("{:?}", hash)` with `format!("{}", hash)` or `format!("0x{}", hex::encode(hash.0))` in all three `BlockCommands` output paths for hash stability.
- **Planner:** Add `"Block N not found"` pattern to `src/error.rs` classify function to return `exit_code::VALIDATION` (12) rather than GENERIC (1).