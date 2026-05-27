<!-- orchestrate failure handoff
task: audit-batch
branch: orch/agcli-audit/audit-batch
agentId: bc-d1549f7b-6bdd-402b-a6af-3a996097019d
runId: run-dfd9e7e8-3a1b-4d7a-bce5-00d5d6095fa8
failureMode: unknown
terminatedAt: 2026-05-27T11:35:17.630Z
-->

# audit-batch failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-d1549f7b-6bdd-402b-a6af-3a996097019d
Started: 2026-05-27T11:34:48.504Z
Terminated: 2026-05-27T11:35:17.630Z
Duration: 29126ms
Last activity: 2026-05-27T11:35:17.554Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-batch
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
