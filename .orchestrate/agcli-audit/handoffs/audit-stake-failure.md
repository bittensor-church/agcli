<!-- orchestrate failure handoff
task: audit-stake
branch: orch/agcli-audit/audit-stake
agentId: bc-4ab7013e-6ea5-4286-8c23-79ad601d611c
runId: run-17cb88b7-14c0-45e0-88e9-0bfa43531dcc
failureMode: unknown
terminatedAt: 2026-05-27T11:34:06.992Z
-->

# audit-stake failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-4ab7013e-6ea5-4286-8c23-79ad601d611c
Started: 2026-05-27T11:33:36.283Z
Terminated: 2026-05-27T11:34:06.992Z
Duration: 30709ms
Last activity: 2026-05-27T11:34:06.910Z - respawned by self-planner (was error; attempts=1)
Last tool call: read_file
Branch: orch/agcli-audit/audit-stake
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
