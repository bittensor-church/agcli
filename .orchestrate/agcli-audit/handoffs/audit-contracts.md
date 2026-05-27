<!-- orchestrate handoff
task: audit-contracts
branch: orch/agcli-audit/audit-contracts
agentId: bc-b69f55c7-9431-423e-b7c6-3ecd9cec82e9
runId: run-6e74f2b6-8204-4ac6-b932-fd32eb57256e
resultStatus: finished
finishedAt: 2026-05-27T12:35:09.609Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-contracts`

## What I did

- **`tests/audit_contracts.rs`** (new, 220 LOC): 15 parse-surface tests covering all 4 `ContractsCommands` subcommands (`upload`, `instantiate`, `call`, `remove-code`) including required-flag rejection tests, a `--help` smoke test, and a `#[ignore]`-gated `green_path_contracts` tokio test that connects to localnet, verifies pallet index 29 is present in runtime metadata, asserts the 4 dispatchables exist by name, and exercises `validate_wasm_file` locally.
- **`docs/commands/contracts.md`** (full rewrite, +415 lines): Every subcommand documented with clap flags + types, exit codes cross-referenced to `src/error.rs`, stdout output format, SCALE encoding notes (e.g. `Code::Existing(hash)` variant, `Weight { ref_time, proof_size }` struct), on-chain events emitted, pallet index 29, storage keys (`CodeInfoOf`, `PristineCode`, `ContractInfoOf`, `DeletionQueue`), and a Missing Surface section.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0 (no regression)
- `cargo test --no-run --test audit_contracts`: exit 0 (compiles)
- `cargo test --test audit_contracts -- --skip green_path_contracts`: 15 passed, 0 failed, 1 filtered (ignored)

## Verification
`unit-test-verified` — 15 parse-surface tests pass locally. The localnet integration test is `#[ignore]`-gated (Docker unavailable in VM).

## Findings

1. **No code hash in `upload` output.** `contracts upload` prints only the tx hash. The uploaded code hash (needed to call `instantiate`) is buried in the `Contracts::CodeStored` event. Agents must do a second RPC call to extract it. The output should print `code_hash:` from the event.

2. **No contract address in `instantiate` output.** `contracts instantiate` prints only the tx hash. The deployed contract's SS58 address (needed for `call`) is in the `Contracts::Instantiated` event. Same usability gap — the handler should subscribe to the event and surface the address.

3. **`instantiate_with_code` pallet dispatchable has no CLI surface.** The subtensor runtime exposes `Contracts::instantiate_with_code` (upload + instantiate in one extrinsic), but agcli only offers the two-step path. This is a missing subcommand for the common "deploy a new contract" workflow.

4. **`set_code` pallet dispatchable has no CLI surface.** Allows upgrading an existing contract's code hash in place. Not exposed in agcli.

5. **Dry-run RPC (`ContractsApi::call`) not exposed.** The runtime implements the `ContractsApi` RPC for off-chain call simulation (no gas/deposit consumed). agcli has no `contracts dry-run` or `contracts query` subcommand, so agents cannot estimate gas or read return values without submitting an extrinsic.

6. **`--determinism` flag missing from `upload`.** `Contracts::upload_code` accepts a `Determinism` enum (`Enforced` / `Unrestricted`). agcli hard-codes `Unrestricted`. Untrusted code should be uploaded as `Enforced`; there is no way to select this via the CLI.

7. **Output is plain text, not JSON.** All four handlers use `println!` with human-readable strings. There is no `--output json` support, so agents cannot reliably parse the tx hash or event data programmatically (inconsistent with other agcli commands that honour `--output json`).

8. **`validate_hex_data` is called on `code_hash` before `strip_prefix("0x")`.** The handler then strips `"0x"` a second time for decoding. This is redundant but harmless; however, `validate_hex_data` accepts bare hex without `0x`, while the clap help says `0x-prefixed` — a documentation/validation mismatch that could confuse agents.

## Suggested follow-ups

- Planner: create a task to make `contracts upload` subscribe to `Contracts::CodeStored` and print `code_hash: 0x...` (same pattern as other event-aware handlers in the codebase).
- Planner: create a task to make `contracts instantiate` print the deployed contract address from `Contracts::Instantiated`.
- Planner: add `contracts instantiate-with-code` subcommand wrapping `Contracts::instantiate_with_code`.
- Planner: add `contracts set-code` subcommand wrapping `Contracts::set_code`.
- Planner: add `contracts dry-run` subcommand using the `ContractsApi` runtime RPC.
- Planner: add `--determinism enforced|unrestricted` flag to `contracts upload`.
- Planner: wire `--output json` through all four contracts handlers (currently ignored).