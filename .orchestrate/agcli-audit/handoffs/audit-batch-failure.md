<!-- orchestrate failure handoff
task: audit-batch
branch: orch/agcli-audit/audit-batch
agentId: bc-1459ebe0-f0c3-4d8a-a0da-e47344cebca4
runId: run-9c951703-ed64-49d8-815d-0905a4ca1df7
failureMode: unknown
terminatedAt: 2026-05-27T11:18:06.981Z
-->

# audit-batch failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-1459ebe0-f0c3-4d8a-a0da-e47344cebca4
Started: 2026-05-27T11:17:23.313Z
Terminated: 2026-05-27T11:18:06.981Z
Duration: 43668ms
Last activity: 2026-05-27T11:18:06.899Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-batch
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
