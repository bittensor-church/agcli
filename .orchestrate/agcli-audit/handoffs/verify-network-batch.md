<!-- orchestrate handoff
task: verify-network-batch
branch: orch/agcli-audit/audit-identity
resultStatus: finished
finishedAt: 2026-05-27T13:32:00Z
-->

## Verification
verifier-failed

## Target
`audit-identity` on branch `orch/agcli-audit/audit-identity`

## Branch
`orch/agcli-audit/audit-identity`

## Execution
- `rustup install stable && rustup default stable && rustc --version && cargo --version` -> pass; selected rustc 1.95.0 and cargo 1.95.0.
- `cargo check --all-targets` -> pass; finished `dev` profile in 1m24s.
- `cargo build --bin agcli` -> pass; finished `dev` profile in 1m39s.
- `cargo test --no-run --workspace` -> pass; compiled workspace tests, including `tests/audit_identity.rs`.
- `cargo test --test audit_identity` -> pass; 20 passed, 0 failed, 1 ignored.
- `cargo test --no-run --test audit_identity` -> pass; compiled the identity audit target.
- `target/debug/agcli identity --help && target/debug/agcli serve --help && target/debug/agcli proxy --help && target/debug/agcli swap --help && target/debug/agcli subscribe --help && target/debug/agcli multisig --help` -> pass; CLI help surfaces were available for all six groups.
- Enum/docs/tests comparison over `IdentityCommands`, `ServeCommands`, `ProxyCommands`, `SwapCommands`, `SubscribeCommands`, and `MultisigCommands` -> fail: identity docs/test coverage met; serve docs miss `reset`, `batch-axon`, `axon-tls` and `tests/audit_serve.rs` is absent; proxy docs miss `remove-all`, `remove-announcement` and `tests/audit_proxy.rs` is absent; swap docs miss `evm-key` and `tests/audit_swap_keys.rs` is absent; subscribe docs enumerate both variants but `tests/audit_subscribe.rs` is absent; multisig docs enumerate all variants but `tests/audit_multisig.rs` is absent.
- Read on-disk handoffs under `.orchestrate/agcli-audit/handoffs/` -> fail for batch evidence: `audit-identity`, `audit-serve`, `audit-swap-keys`, `audit-subscribe`, and `audit-multisig` are raw `resultStatus: error` stubs without structured `## Findings`; only `audit-proxy` is structured on disk. The prompt supplied structured upstream findings for all six, consolidated below.

## Findings
Per acceptance criterion:
- [x] `cargo check --all-targets` passes on the checked-out branch: command exited 0. (met)
- [x] `cargo build --bin agcli` passes on the checked-out branch: command exited 0. (met)
- [x] `cargo test --no-run --workspace` passes on the checked-out branch: command exited 0. (met)
- [ ] Every dependent worker's `audit_*.rs` test file compiles: only `tests/audit_identity.rs` exists and compiles; `audit_serve`, `audit_proxy`, `audit_swap_keys`, `audit_subscribe`, and `audit_multisig` test files are absent from this checkout. (not met)
- [ ] Each docs/commands/<group>.md touched by dependent workers enumerates every enum subcommand: identity met; subscribe and multisig headings enumerate all variants; serve misses `reset`, `batch-axon`, `axon-tls`; proxy misses `remove-all`, `remove-announcement`; swap misses `evm-key`. (not met)
- [ ] Each `tests/audit_<group>.rs` parses every subcommand via `Cli::try_parse_from`: identity met by `cargo test --test audit_identity`; the other five audit test files are absent. (not met)
- [x] Verdict handoff consolidates dependent worker findings into one rollup. (met)
- [x] Status reflects actual verification result: `verifier-failed`. (met)

Other findings (severity-ordered):
- (high) This checkout is not the requested merged synthetic branch for `audit-identity`, `audit-serve`, `audit-proxy`, `audit-swap-keys`, `audit-subscribe`, and `audit-multisig`; it contains only the identity worker output.
- (high) The on-disk handoff files conflict with the prompt's structured upstream context: five of six are `resultStatus: error` raw-output stubs and cannot satisfy the "read each worker's handoff" requirement by themselves.
- (high) Batch acceptance cannot pass until the missing audit test files and refreshed docs from the other worker branches are present on the branch being verified.

Findings rollup:
- Identity: `identity set` and `identity clear` use wrong/missing Registry SCALE arguments; `identity set-subnet` calls the wrong Subtensor dispatchable and ignores `netuid`; identity JSON output is ignored; several Registry/SubnetIdentity fields have no CLI flags or decoded output.
- Serve: `serve reset` submits an invalid zero port; `axon-tls --cert` docs/validation do not match the pallet's 65-byte certificate format; IPv6 and placeholder fields are not exposed; write commands ignore `--output json`; no `serve show/status` read path exists.
- Proxy: announced proxy paths encode `AccountId32` without the expected `MultiAddress::Id` wrapper; `Proxy::proxy` and `Proxy::poke_deposit` have no CLI surface; most write commands ignore `--output json`; docs previously missed accepted proxy types and two subcommands.
- Swap keys: `swap coldkey` calls deprecated `schedule_swap_coldkey`; `swap evm-key` omits required `netuid` and uses `u32` for a `u64` block number; current coldkey rotation lifecycle dispatchables are missing; `swap hotkey` cannot scope by subnet; common write paths ignore JSON output.
- Subscribe: event filter variant lists drift from actual pallet events, including phantom swap/staking/crowdloan events and missing real events; `--netuid` filtering drops unnamed positional netuid fields; JSON uses `event` instead of `variant`; no transaction subscription surface exists.
- Multisig: `approve` cannot pass a required timepoint for non-first approvals; `submit` docs imply `as_multi` while implementation uses `approve_as_multi`; approval max weight is zero; `list` has no JSON output; storage-key parsing is brittle; multisig address hashing duplicates existing hashing logic.

## Notes & suggestions
- Do not mark `verify-network-batch` successful from this branch.
- Re-run verification on an actual synthetic merge containing all six worker outputs, then re-run `cargo check --all-targets`, `cargo build --bin agcli`, `cargo test --no-run --workspace`, the enum/docs comparison, and each `cargo test --no-run --test audit_<group>` target.
- The identity worker output itself compiles and its parse-surface tests pass, but that is insufficient for the batch verifier acceptance criteria.
