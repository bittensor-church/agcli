<!-- orchestrate failure handoff
task: audit-scheduler
branch: orch/agcli-audit/audit-scheduler
agentId: bc-a145e3f6-19c8-43df-9648-57bd5af584d8
runId: run-b02086e3-6e61-4934-84d2-dca81520faba
failureMode: unknown
terminatedAt: 2026-05-27T11:17:16.678Z
-->

# audit-scheduler failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-a145e3f6-19c8-43df-9648-57bd5af584d8
Started: 2026-05-27T11:16:24.462Z
Terminated: 2026-05-27T11:17:16.678Z
Duration: 52216ms
Last activity: 2026-05-27T11:17:16.605Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-scheduler
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
