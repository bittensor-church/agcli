<!-- orchestrate failure handoff
task: audit-root
branch: orch/agcli-audit/audit-root
agentId: bc-4d9d9c0d-c2e3-4b07-84b7-26596df28cfb
runId: run-9d9fd34d-4f52-4cc6-ae28-6d616165207f
failureMode: unknown
terminatedAt: 2026-05-27T11:16:44.032Z
-->

# audit-root failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-4d9d9c0d-c2e3-4b07-84b7-26596df28cfb
Started: 2026-05-27T11:15:57.672Z
Terminated: 2026-05-27T11:16:44.032Z
Duration: 46360ms
Last activity: 2026-05-27T11:16:43.934Z
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-root
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
