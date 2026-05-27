<!-- orchestrate failure handoff
task: audit-subnet
branch: orch/agcli-audit/audit-subnet
agentId: bc-3634a8bc-4bea-4727-b311-45c7d5cbb625
runId: run-6e8c03dd-d617-4629-8bd2-8934749992ec
failureMode: unknown
terminatedAt: 2026-05-27T11:16:56.769Z
-->

# audit-subnet failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-3634a8bc-4bea-4727-b311-45c7d5cbb625
Started: 2026-05-27T11:15:51.385Z
Terminated: 2026-05-27T11:16:56.769Z
Duration: 65384ms
Last activity: 2026-05-27T11:16:52.291Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-subnet
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
