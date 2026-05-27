<!-- orchestrate handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-00778682-6d93-4cd5-808e-aca17b0934b2
runId: run-fdb55d13-0b31-401b-974e-663cc9abee93
resultStatus: error
finishedAt: 2026-05-27T11:17:42.475Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

Understood — I’ll start by reading `.orchestrate/agcli-audit/discovery.md`, then trace the completions/update command surfaces and their handlers before editing only `docs/commands/utils.md` and `tests/audit_completions_update.rs`. I’ll finish by running the required cargo checks and produce a single structured handoff.