<!-- orchestrate failure handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-b0077730-c33f-45a8-92b7-105a1b5691e0
runId: run-b5112d78-a609-44b9-87e4-c5a66bf1d963
failureMode: unknown
terminatedAt: 2026-05-27T11:34:16.007Z
-->

# audit-view failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-b0077730-c33f-45a8-92b7-105a1b5691e0
Started: 2026-05-27T11:33:50.337Z
Terminated: 2026-05-27T11:34:16.007Z
Duration: 25670ms
Last activity: 2026-05-27T11:34:15.892Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-view
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
