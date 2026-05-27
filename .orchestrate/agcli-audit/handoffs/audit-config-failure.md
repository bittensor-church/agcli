<!-- orchestrate failure handoff
task: audit-config
branch: orch/agcli-audit/audit-config
agentId: bc-fecd7651-0165-4e87-9bc0-9c7a6d8578ad
runId: run-f34d57df-96de-4f33-b47d-1bc1f6d83e94
failureMode: unknown
terminatedAt: 2026-05-27T11:35:31.047Z
-->

# audit-config failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-fecd7651-0165-4e87-9bc0-9c7a6d8578ad
Started: 2026-05-27T11:34:38.335Z
Terminated: 2026-05-27T11:35:31.047Z
Duration: 52712ms
Last activity: 2026-05-27T11:35:30.946Z - respawned by self-planner (was error; attempts=1)
Last tool call: read_file
Branch: orch/agcli-audit/audit-config
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
