<!-- orchestrate failure handoff
task: audit-wallet
branch: orch/agcli-audit/audit-wallet
agentId: bc-3c02dd3b-0841-4fe5-a2f0-3c7dfdc2764b
runId: run-a23dfb60-cce6-4b5b-91a8-2104f3bbc16a
failureMode: unknown
terminatedAt: 2026-05-27T11:16:41.532Z
-->

# audit-wallet failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-3c02dd3b-0841-4fe5-a2f0-3c7dfdc2764b
Started: 2026-05-27T11:15:41.798Z
Terminated: 2026-05-27T11:16:41.532Z
Duration: 59734ms
Last activity: 2026-05-27T11:16:41.313Z
Last tool call: grep_search
Branch: orch/agcli-audit/audit-wallet
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
