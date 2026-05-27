<!-- orchestrate handoff
task: audit-proxy
branch: orch/agcli-audit/audit-proxy
agentId: bc-f4e2ec58-a1bf-4f3b-9f33-66e43dca0d4c
runId: run-3e980345-6fc6-444f-8191-bc99ab2f38f9
resultStatus: finished
finishedAt: 2026-05-27T12:37:19.338Z
-->

---

## Status
success

## Branch
`orch/agcli-audit/audit-proxy`

## What I did
- Read `.orchestrate/agcli-audit/discovery.md` for environment constraints and cross-reference pointers.
- Traced every `ProxyCommands` variant (11 subcommands) through `handle_proxy` in `src/cli/network_cmds.rs` → into each extrinsic in `src/chain/extrinsics.rs`, verifying pallet name, dispatchable name, and SCALE argument construction.
- Cross-referenced against the Substrate `pallet-proxy` dispatchable surface (11 CLI-mapped + 2 unmapped: `proxy` and `poke_deposit`).
- Ran `cargo check --all-targets` — passed in ~72 s.
- **Created `tests/audit_proxy.rs`**: 33 tests (32 parse-surface + 1 `#[ignore]` localnet integration test). Covers all 11 subcommands including `remove-all` and `remove-announcement` which had zero prior test coverage. All 32 non-ignored tests pass.
- **Rewrote `docs/commands/proxy.md`**: every subcommand documented with clap flags + types, exit codes (numeric values from `src/error.rs`), JSON output schema, pallet call + storage keys, events emitted, and 18-entry proxy type table (was 11).

## Measurements
- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo test --no-run --test audit_proxy`: compiles clean
- `cargo test --test audit_proxy`: 32 passed; 0 failed; 1 ignored
- `tests/audit_proxy.rs`: 0 LOC → 419 LOC
- `docs/commands/proxy.md`: 43 LOC → 275 LOC

## Verification
`unit-test-verified` — 32 parse-surface tests exercise every `ProxyCommands` variant via `Cli::try_parse_from` and assert correct field values. `cargo test --test audit_proxy` passes in the worker's VM.

## Notes, concerns, deviations, findings, thoughts, feedback

**Findings:**

1. **`proxy_announce` and `proxy_reject_announcement` pass `AccountId32` as raw bytes instead of `MultiAddress::Id`** — `src/chain/extrinsics.rs:1852` uses `Value::from_bytes(real_id.0)` for the `real` argument; `reject_announcement` (line 1904) uses the same pattern for `delegate`. Both pallet calls expect `AccountIdLookupOf<T>` = `MultiAddress<AccountId32, ()>`, which SCALE-encodes as variant index 0 + 32 bytes. Compare with `add_proxy` (line 850) and `remove_announcement` (line 2566) which correctly use `Value::unnamed_variant("Id", [Value::from_bytes(...)])`. These calls will likely fail with a SCALE decode error on the node side. Same issue applies to `proxy_announced` (lines 1883–1884): `delegate` and `real` both lack the `Id` wrapper.

2. **`Proxy::proxy` dispatchable has no CLI surface** — `src/chain/extrinsics.rs` exports `proxy_call()` which wraps `Proxy::proxy` (immediate proxy execution without a prior announcement), but no `ProxyCommands` variant routes to it. An agent cannot invoke an immediate proxy call via `agcli proxy`.

3. **`Proxy::poke_deposit` dispatchable has no CLI surface** — The pallet's `poke_deposit` call (update deposit base/factor for an existing proxy entry) is entirely absent from both the CLI enum and the extrinsics layer.

4. **Write subcommands produce plain text even with `--output json`** — `proxy add`, `proxy remove`, `proxy create-pure`, `proxy kill-pure`, `proxy announce`, `proxy remove-all`, `proxy remove-announcement`, `proxy reject-announcement`, and `proxy proxy-announced` all use `println!()` directly without consulting `output.is_json()`. Machine-readable JSON output (which `proxy list` and `proxy list-announcements` do implement correctly) is broken for all write operations.

5. **Proxy type table in old docs listed only 11 of 18 accepted types** — `triumvirate`, `non_fungible`, `sudo_unchecked_set_code`, `swap_hotkey`, `subnet_lease_beneficiary`, and `root_claim` were all accepted by `validate_proxy_type` and `parse_proxy_type` but absent from `docs/commands/proxy.md`. Updated.

6. **`remove-all` and `remove-announcement` subcommands were undocumented** — The old `proxy.md` had 9 subcommands. The actual CLI has 11. `remove-all` and `remove-announcement` had no doc entries and no parse-surface tests anywhere in the test suite.

7. **`delay` field type drift**: `proxy_op` encodes `delay` as `Value::u128(delay as u128)`. The pallet's `BlockNumberFor<T>` is `u32`. Subxt's dynamic encoder will accept this but the semantic mismatch could cause issues if the chain uses compact encoding for the field. `create_pure` and `kill_pure` encode `index` as `Value::u128(index as u128)` while the pallet type is `u16`. No runtime failure observed, but the type annotations in code comments say `compact<BlockNumber>` — subxt dynamic uses `Value::u128` for compact which is acceptable.

## Suggested follow-ups
- Fix `proxy_announce`, `proxy_announced`, and `proxy_reject_announcement` in `src/chain/extrinsics.rs` to wrap `AccountId32` args in `Value::unnamed_variant("Id", [...])` — these are likely-panicking paths on a real chain.
- Add `proxy exec` (or `proxy call`) CLI subcommand wiring to the existing `proxy_call()` extrinsic for immediate (no-delay) proxy execution.
- Add `proxy poke-deposit` subcommand + extrinsic for `Proxy::poke_deposit`.
- Add `--output json` support for all write operations in `handle_proxy` (parallel issue exists in other command groups).
- Add the 6 missing proxy types to `src/cli/mod.rs` help text for `--proxy-type` (it currently lists 15 but is still missing `triumvirate`, `non_fungible`, `sudo_unchecked_set_code`).