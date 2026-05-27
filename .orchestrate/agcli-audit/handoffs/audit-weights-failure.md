<!-- orchestrate failure handoff
task: audit-weights
branch: orch/agcli-audit/audit-weights
agentId: bc-0ce96513-898e-4062-999a-5e1f4245c353
runId: run-4430ee73-1d49-44b1-bb97-8d28d679f99d
failureMode: unknown
terminatedAt: 2026-05-27T11:16:28.795Z
-->

# audit-weights failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-0ce96513-898e-4062-999a-5e1f4245c353
Started: 2026-05-27T11:15:54.263Z
Terminated: 2026-05-27T11:16:28.795Z
Duration: 34532ms
Last activity: 2026-05-27T11:16:28.672Z
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-weights
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
