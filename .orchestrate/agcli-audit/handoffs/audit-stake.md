<!-- orchestrate handoff
task: audit-stake
branch: orch/agcli-audit/audit-stake
agentId: bc-4ab7013e-6ea5-4286-8c23-79ad601d611c
runId: run-17cb88b7-14c0-45e0-88e9-0bfa43531dcc
resultStatus: error
finishedAt: 2026-05-27T11:34:06.992Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m going to do this as a strict docs+test audit only: first I’ll read `.orchestrate/agcli-audit/discovery.md` and map every `stake` subcommand/handler to the corresponding subtensor dispatchable + events/storage, then I’ll update `docs/commands/stake.md`, add `tests/audit_stake.rs`, and finally run the required compile checks.I’ve confirmed we’re on the expected starting branch; next I’m switching to `orch/agcli-audit/audit-stake`, then I’ll inspect `StakeCommands`/`stake_cmds.rs` and the subtensor dispatchables/events to build an exact command-to-chain mapping before editing docs/tests.