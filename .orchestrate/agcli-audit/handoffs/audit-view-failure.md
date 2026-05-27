<!-- orchestrate failure handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-51349619-31d5-4adf-b645-a58b4c14f033
runId: run-71e3a518-1a52-45a6-bcf4-a45bc84731f0
failureMode: unknown
terminatedAt: 2026-05-27T12:35:07.992Z
-->

# audit-view failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-51349619-31d5-4adf-b645-a58b4c14f033
Started: 2026-05-27T12:34:43.998Z
Terminated: 2026-05-27T12:35:07.992Z
Duration: 23994ms
Last activity: 2026-05-27T12:35:07.855Z - respawned by self-planner (was error; attempts=2)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-view
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
