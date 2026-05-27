<!-- orchestrate handoff
task: audit-serve
branch: orch/agcli-audit/audit-serve
agentId: bc-999a8cf2-3d14-4c06-804f-4d9a50a95a11
runId: run-7b59a8de-9a5b-47bb-8614-ccc58907363a
resultStatus: error
finishedAt: 2026-05-27T11:34:18.170Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading `.orchestrate/agcli-audit/discovery.md` first, then I’ll trace every `serve` subcommand from CLI parse surface through handlers to pallet dispatchables and events before updating only the allowed docs/test files.