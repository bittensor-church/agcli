<!-- orchestrate failure handoff
task: audit-swap-keys
branch: orch/agcli-audit/audit-swap-keys
agentId: bc-7ac271d0-07bb-4192-a227-c4726a9b5bc1
runId: run-7ff126d3-a54f-4b21-acba-335532c9c298
failureMode: unknown
terminatedAt: 2026-05-27T12:38:00.869Z
-->

# audit-swap-keys failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-7ac271d0-07bb-4192-a227-c4726a9b5bc1
Started: 2026-05-27T12:37:25.423Z
Terminated: 2026-05-27T12:38:00.869Z
Duration: 35446ms
Last activity: 2026-05-27T12:38:00.730Z - respawned by self-planner (was error; attempts=2)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-swap-keys
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
