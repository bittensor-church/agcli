<!-- orchestrate handoff
task: audit-weights
branch: orch/agcli-audit/audit-weights
agentId: bc-9c79c56b-5a27-43da-be75-5398af15189c
runId: run-bf29cb0f-d0da-4ea8-8d32-0e0bc8f195aa
resultStatus: finished
finishedAt: 2026-05-27T13:36:48.659Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-weights`

## What I did

- **`tests/audit_weights.rs`** (new, 620 LOC):
  - 86 non-ignored parse-surface, validation, and unit tests covering all 10 `WeightCommands` variants: `Show`, `Set`, `Commit`, `Reveal`, `Status`, `CommitTimelocked`, `CommitReveal`, `SetMechanism`, `CommitMechanism`, `RevealMechanism`
  - Variant field inspection tests (dry-run on root Cli not local field; `CommitTimelocked` has `round: u64`; `CommitMechanism` carries precomputed `hash` string; `RevealMechanism` salt is required)
  - Commit-hash unit tests: determinism, salt sensitivity, uid/weight sensitivity, 32-byte length guarantee
  - Salt u16 encoding tests: even-length, odd-length, single-byte padding, empty, round-trip consistency vs raw bytes
  - Reveal-window timing math unit tests (block calculation, open/closed boundary, minutes/seconds display)
  - 500-pair stress parse (no hang/panic)
  - 1 `#[ignore]` localnet green-path test gated on `ws://127.0.0.1:9944`

- **`docs/commands/weights.md`** (full rewrite, 138 → 440+ LOC):
  - All 10 subcommands documented with clap flags+types, exit codes (full table with numeric codes), JSON output schema, pallet ref + storage keys, on-chain events emitted
  - New sections: batch operations (SDK-only, no CLI surface), exit code reference table, findings (10 concrete observations)

## Measurements

- `cargo check --all-targets`: pass → pass
- `cargo test --no-run --test audit_weights`: compiles
- `cargo test --test audit_weights`: 86 passing, 1 ignored
- `LOC(tests/audit_weights.rs)`: 0 → 620
- `LOC(docs/commands/weights.md)`: 138 (original lines before submodule removal, effectively ~287 substantive) → 440+

## Verification
unit-test-verified (86 non-ignored tests pass via `cargo test --test audit_weights`; Docker unavailable so localnet integration test is `#[ignore]`)

## Notes, concerns, deviations, findings, thoughts, feedback

The subtensor submodule was not initialized at start; initialized shallowly for `cargo check`. The `SKIP_METADATA_FETCH=1` flag was required since no live finney RPC is available in this VM. All 10 WeightCommands subcommands were traced from clap definition through handler to extrinsic dispatch.

## Findings

1. **Batch weight operations (SDK functions, no CLI surface).** `batch_set_weights`, `batch_commit_weights`, `batch_reveal_weights` are fully implemented in `src/chain/extrinsics.rs` with typed pallet API calls and length-validation guards. No `WeightCommands` variants exist for any of them. Agents cannot invoke batch weight operations through `agcli weights`. The pallet exposes all three dispatches.

2. **`commit_crv3_mechanism_weights` entirely absent.** The task scope lists this as a pallet dispatchable in scope. It does not appear anywhere in `src/` — neither in `extrinsics.rs` nor in the CLI enum. If it is a distinct dispatch from `commit_mechanism_weights`, it has no agcli surface at all.

3. **Dispatch name mismatch between docs and typed subxt API.** The previous docs cited the on-chain functions as `commit_crv3_weights` and `reveal_crv3_weights`. The actual subxt typed calls are `api::tx().subtensor_module().commit_weights(...)` and `reveal_weights(...)`. These are the names generated from the chain metadata. The pallet source may use `_crv3_` internally, but the wire-level dispatch (and what agents must call) uses the shorter names.

4. **Mechanism + timelocked weight extrinsics use unverified raw string dispatch.** `set_mechanism_weights`, `commit_mechanism_weights`, `reveal_mechanism_weights`, and `commit_timelocked_weights` all call `submit_raw_call` with string literals for pallet and dispatchable names. The typed `api::tx()` path (used by `set_weights`, `commit_weights`, `reveal_weights`, all batch functions) gets compile-time verification from the generated metadata. A pallet rename or removal on the four raw-call dispatches would only surface at runtime as `DispatchNotFound`, not at build time.

5. **All write commands silently ignore `--output json`.** `weights set` (live path), `weights commit`, `weights reveal`, `weights commit-timelocked`, `weights set-mechanism`, `weights commit-mechanism`, `weights reveal-mechanism` never read `ctx.output`. Only the dry-run path of `weights set` and `weights set-mechanism`, plus the `--wait` path of `weights commit-reveal`, emit any JSON. Agents expecting machine-parseable output from write commands receive human text with exit 0.

6. **`weights status` has no JSON output mode.** The output contains structured data (commit hash, block numbers, reveal window, phase) that would be natural JSON. `ctx.output` is never read; `--output json` is silently ignored.

7. **`weights commit-reveal` falls back to `set_weights` silently.** When commit-reveal is disabled on the subnet, the command prints a warning to stderr and falls back to direct `set_weights`, exiting 0 either way. An agent running automation expecting a two-phase flow will silently get a single-phase set with no distinguishable exit code.

8. **API asymmetry between `commit` and `commit-mechanism`.** `weights commit` accepts `--weights` + optional `--salt` and computes the hash internally. `weights commit-mechanism` requires a precomputed `--hash` and has no `--weights` or `--salt` flag. Agents using the mechanism commit-reveal flow must compute the hash out-of-band, while the global flow is end-to-end. This asymmetry is not documented in help text.

9. **No parse tests for 4 subcommands in existing test files.** Prior to this audit, `tests/cli_weights.rs` had zero tests for `commit-timelocked`, `set-mechanism`, `commit-mechanism`, `reveal-mechanism`. The new `tests/audit_weights.rs` adds full parse-surface and error-path coverage for all four.

10. **`validate_weights_args` is a no-op for `WeightCommands::Show`.** `validate_weights_args` matches `Show { .. } => {}` and does nothing. All Show-path validation (netuid, SS58, limit) happens inside `handle_weights` at runtime. This is functionally correct but inconsistent with other subcommands that get early validation in `validate_weights_args`.

## Suggested follow-ups

- **Source** (`weights_cmds.rs`/`mod.rs`): Add `WeightCommands::BatchSet`, `BatchCommit`, `BatchReveal` wired to the existing SDK functions in `extrinsics.rs`.
- **Source** (`extrinsics.rs`): Investigate `commit_crv3_mechanism_weights` — if it's a distinct pallet dispatch, add SDK function + CLI surface.
- **Source** (`extrinsics.rs`): Switch `set_mechanism_weights`, `commit_mechanism_weights`, `reveal_mechanism_weights`, `commit_timelocked_weights` from `submit_raw_call` to typed `api::tx()` bindings.
- **Source** (`weights_cmds.rs`): All write commands should check `ctx.output.is_json()` and emit `{"tx": "<hash>", "netuid": N, ...}` for agent consumption.
- **Source** (`weights_cmds.rs`): `weights status` should emit a JSON object when `--output json` is passed.
- **Source** (`weights_cmds.rs`): Add `--no-fallback` flag to `weights commit-reveal` to fail (exit 12) instead of silently falling back to direct set when commit-reveal is disabled.
- **Source** (`weights_cmds.rs`/`mod.rs`): Add `--weights` + `--salt` path to `weights commit-mechanism` to match the ergonomics of `weights commit`, or add a `agcli utils hash-weights` command for offline hash computation.
- **Localnet test**: Run `green_path_weights_localnet` against `ghcr.io/opentensor/subtensor-localnet:devnet-ready` once Docker is available to confirm `weights show` and `weights set --dry-run` work end-to-end.