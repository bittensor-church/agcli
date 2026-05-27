<!-- orchestrate handoff
task: audit-multisig
branch: orch/agcli-audit/audit-multisig
agentId: bc-565b9883-d39b-4afc-aa70-b1e212b3ae3f
runId: run-3df6bba3-5a6e-4446-9823-91baf0d5c089
resultStatus: finished
finishedAt: 2026-05-27T13:32:35.887Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-multisig`

## What I did
- Read `.orchestrate/agcli-audit/discovery.md` for environment constraints and the pallet mapping table.
- Enumerated all 6 `MultisigCommands` variants (`Address`, `Submit`, `Approve`, `Execute`, `Cancel`, `List`) in `src/cli/mod.rs` (L1757–L1845) and traced each through `handle_multisig` in `src/cli/network_cmds.rs` (L409–L668).
- Traced every chain call to its subxt sink: `submit_multisig_call` → `approve_multisig` → `Multisig::approve_as_multi`; `execute_multisig` → `Multisig::as_multi`; `cancel_multisig` → `Multisig::cancel_as_multi`; `list_multisig_pending` → `Multisig::Multisigs` storage prefix scan.
- Cross-referenced with the subtensor runtime (`subtensor/runtime/src/lib.rs`): `pallet-multisig` from the opentensor polkadot-sdk fork at rev `7cc54bf`, pallet index 13, `MaxSignatories = 100`.
- **Created `tests/audit_multisig.rs`**: 28 non-ignored parse-surface tests covering all 6 variants, field extraction, global flags, and one targeted regression test (`approve_has_no_timepoint_flags`) that explicitly documents the `NoTimepoint` bug. One `#[ignore]` localnet integration test included.
- **Rewrote `docs/commands/multisig.md`**: every subcommand documented with clap flags + types, exit codes (from `src/error.rs`), output schema, pallet ref, storage keys, on-chain events, runtime config table, and 6 concrete audit findings.

## Measurements
- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_multisig`: compiles cleanly
- `cargo test --test audit_multisig`: 28 passed; 0 failed; 1 ignored
- `tests/audit_multisig.rs`: 0 LOC → 400 LOC
- `docs/commands/multisig.md`: 62 LOC → 270 LOC

## Verification
`unit-test-verified` — 28 parse-surface tests exercise every `MultisigCommands` variant via `Cli::try_parse_from`; `cargo test --test audit_multisig` passes in this VM. The `#[ignore]`d localnet test is not verified (Docker unavailable in cloud-agent VM per `discovery.md`).

## Findings

1. **`approve` is broken for non-first approvals — `NoTimepoint` pallet error.** `handle_multisig` → `approve_multisig` always passes `maybe_timepoint = None`. The `Multisig::approve_as_multi` pallet requires `maybe_timepoint: Some(Timepoint)` for any call that already has an existing `Multisigs` storage entry. The `Approve` clap variant has no `--timepoint-height` / `--timepoint-index` flags, so there is currently no way to issue a non-first approval without triggering a runtime `NoTimepoint` dispatch error. The test `approve_has_no_timepoint_flags` explicitly asserts this surface gap compiles (the flag is rejected by clap as unknown, confirming the missing surface).

2. **`submit` docstring says `as_multi` but the implementation calls `approve_as_multi`.** The clap variant comment reads `"Submit a multisig call (as_multi)"`. The actual dispatch is `approve_as_multi` (via `submit_multisig_call` → `approve_multisig`). For a 1-of-N threshold, `approve_as_multi` with `maybe_timepoint = None` stores a hash but does NOT execute the call. An agent reading the doc comment will expect execution semantics; the actual behavior is approval-only.

3. **`submit` and `approve` pass `max_weight = {ref_time: 0, proof_size: 0}` to `approve_as_multi`.** The `execute` path uses generous defaults (10B ref_time, 1MB proof_size), but `approve_multisig` hardcodes zero weight. If the pallet interprets the zero-weight `approve_as_multi` as a final-approval execution path (possible if caller is the last required signer and stored call data exists from a prior `as_multi`), the dispatch returns `MaxWeightTooLow`. This is a latent bug when threshold = 2 and both signers use `submit`/`approve` rather than the intended `submit` + `execute` flow.

4. **`list` output is plain text only — no JSON mode.** Every other agcli write/query command supports `--output json`. `multisig list` always emits multi-line human-readable output (`Call hash: ...`, `Timepoint: ...`, `Approvals: ...`, `Deposit: ... RAO`). Agent consumers must screen-scrape. The underlying `list_multisig_pending` returns a well-typed `Vec<(String, u32, u32, u32, u128)>` that maps trivially to JSON, but the handler never calls `print_json`.

5. **`list_multisig_pending` extracts call hash from the last 32 bytes of the raw storage key without validation.** The `Multisig::Multisigs` double-map uses `Blake2_128Concat` for both keys. The full key layout is: `prefix(16) + storage_hash(16) + blake2_128(account_id)(16) + account_id(32) + blake2_128(call_hash)(16) + call_hash(32)` = 128 bytes. The code takes `key_bytes[len-32..]` which is correct for this layout but falls back to `"unknown"` if `len < 32` with no log warning — a silent data-loss path. A subxt-version change altering the key layout would silently produce wrong hashes.

6. **`multisig address` uses `blake2::Blake2bVar` but `sp_core::hashing::blake2_256` used elsewhere is `Blake2b-256` — these are the same function, but the implementation is duplicated rather than calling the already-imported `sp_core::hashing::blake2_256`.** This creates a divergence risk: if either copy is updated, the two paths produce different addresses. The `execute_multisig` path computes the call hash with `sp_core::hashing::blake2_256(&encoded)` (in `submit_multisig_call`), while `address` uses the manual `Blake2bVar` path. Both should use a single canonical function.

## Suggested follow-ups
- Add `--timepoint-height` / `--timepoint-index` to `MultisigCommands::Approve` and thread them through `approve_multisig` as `Option<(u32, u32)>` for `maybe_timepoint`.
- Fix the `submit` clap docstring: change "Submit a multisig call (as_multi)" to "Submit a multisig call (approve_as_multi — first approval)".
- Set a non-zero default `max_weight` in `approve_multisig` to match the generous defaults in `execute_multisig` (or pass it as a parameter).
- Add JSON output to `multisig list`: emit `{"pending": [{"call_hash": "0x...", "timepoint_height": N, "timepoint_index": N, "approvals": N, "deposit_rao": N}]}` when `--output json`.
- Replace the manual `Blake2bVar` in `address` handler with `sp_core::hashing::blake2_256` to eliminate the duplication.
- Add a deposit-estimation subcommand (`multisig deposit-estimate --threshold N --signatories M`) using the `DepositBase + DepositFactor * (threshold - 1)` formula from the runtime config.