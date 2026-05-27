<!-- orchestrate handoff
task: audit-swap-keys
branch: orch/agcli-audit/audit-swap-keys
agentId: bc-7ac271d0-07bb-4192-a227-c4726a9b5bc1
runId: run-7ff126d3-a54f-4b21-acba-335532c9c298
resultStatus: error
finishedAt: 2026-05-27T12:38:00.869Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

Understood — I’ll first read `.orchestrate/agcli-audit/discovery.md`, then trace every swap subcommand through handlers/extrinsics and pallet calls, update only `docs/commands/swap.md` and `tests/audit_swap_keys.rs`, and finish with compile verification plus a structured audit handoff.I’ve loaded the discovery context and identified the swap command entrypoints; next I’m tracing each subcommand from clap surface to handler and extrinsic encoding, then I’ll map them to subtensor dispatchables/events/storage before editing docs and adding the audit test.