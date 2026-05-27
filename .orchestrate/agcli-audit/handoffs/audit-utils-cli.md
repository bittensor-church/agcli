<!-- orchestrate handoff
task: audit-utils-cli
branch: orch/agcli-audit/audit-utils-cli
agentId: bc-93ad6478-1220-4ffb-8a07-4aa82b0cea26
runId: run-6ddb7fc1-d344-400e-b020-ba1f8d93b495
resultStatus: error
finishedAt: 2026-05-27T11:18:38.223Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the orchestrator discovery doc and mapping the full `utils` command surface from CLI definitions through `handle_utils`, then I’ll trace each path into subxt/pallet calls before editing only the allowed docs and test file.