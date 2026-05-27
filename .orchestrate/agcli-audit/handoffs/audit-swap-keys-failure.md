<!-- orchestrate failure handoff
task: audit-swap-keys
branch: orch/agcli-audit/audit-swap-keys
agentId: bc-ace70aa8-04f5-4a02-830b-c95cf986de4d
runId: run-7068d543-90ca-4f12-926b-6cdd9dc0cd19
failureMode: unknown
terminatedAt: 2026-05-27T11:39:37.593Z
-->

# audit-swap-keys failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-ace70aa8-04f5-4a02-830b-c95cf986de4d
Started: 2026-05-27T11:34:02.097Z
Terminated: 2026-05-27T11:39:37.593Z
Duration: 335496ms
Last activity: 2026-05-27T11:39:37.460Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-swap-keys
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
