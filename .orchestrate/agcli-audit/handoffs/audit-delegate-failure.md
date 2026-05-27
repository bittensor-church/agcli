<!-- orchestrate failure handoff
task: audit-delegate
branch: orch/agcli-audit/audit-delegate
agentId: bc-3977d1ee-f668-4367-af25-7a570d7527a5
runId: run-5185ee21-d4af-4415-8406-8b16d955c5dc
failureMode: unknown
terminatedAt: 2026-05-27T11:17:01.033Z
-->

# audit-delegate failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-3977d1ee-f668-4367-af25-7a570d7527a5
Started: 2026-05-27T11:16:00.462Z
Terminated: 2026-05-27T11:17:01.033Z
Duration: 60571ms
Last activity: 2026-05-27T11:17:00.947Z
Last tool call: grep_search
Branch: orch/agcli-audit/audit-delegate
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
