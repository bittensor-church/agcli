<!-- orchestrate failure handoff
task: audit-subscribe
branch: orch/agcli-audit/audit-subscribe
agentId: bc-2fba0828-9685-4dbd-91b9-b4ba7720c33a
runId: run-d3539e35-90cd-4fd0-be51-54b6d8243e52
failureMode: unknown
terminatedAt: 2026-05-27T11:16:52.296Z
-->

# audit-subscribe failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-2fba0828-9685-4dbd-91b9-b4ba7720c33a
Started: 2026-05-27T11:16:18.550Z
Terminated: 2026-05-27T11:16:52.296Z
Duration: 33746ms
Last activity: 2026-05-27T11:16:49.554Z
Last tool call: file_search
Branch: orch/agcli-audit/audit-subscribe
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
