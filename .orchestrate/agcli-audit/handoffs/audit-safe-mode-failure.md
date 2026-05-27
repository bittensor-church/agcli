<!-- orchestrate failure handoff
task: audit-safe-mode
branch: orch/agcli-audit/audit-safe-mode
agentId: bc-540fe24a-9cf4-41dd-b034-38598d0c53ae
runId: run-671c953c-5bc3-42c5-8d95-b6d6c61db2cd
failureMode: unknown
terminatedAt: 2026-05-27T11:18:17.234Z
-->

# audit-safe-mode failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-540fe24a-9cf4-41dd-b034-38598d0c53ae
Started: 2026-05-27T11:16:36.995Z
Terminated: 2026-05-27T11:18:17.234Z
Duration: 100239ms
Last activity: 2026-05-27T11:18:17.149Z
Last tool call: grep_search
Branch: orch/agcli-audit/audit-safe-mode
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
