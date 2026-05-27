<!-- orchestrate failure handoff
task: audit-identity
branch: orch/agcli-audit/audit-identity
agentId: bc-89aeffb2-8e21-470a-9b95-b1f7d078bc41
runId: run-fae891aa-c357-4336-9791-724f7a30c293
failureMode: unknown
terminatedAt: 2026-05-27T11:34:53.204Z
-->

# audit-identity failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-89aeffb2-8e21-470a-9b95-b1f7d078bc41
Started: 2026-05-27T11:33:53.181Z
Terminated: 2026-05-27T11:34:53.204Z
Duration: 60023ms
Last activity: 2026-05-27T11:34:53.129Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-identity
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
