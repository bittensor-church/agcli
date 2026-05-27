<!-- orchestrate failure handoff
task: audit-delegate
branch: orch/agcli-audit/audit-delegate
agentId: bc-3ec513cd-a5b6-4cd4-91c3-3641a30005a9
runId: run-633dbb5d-581e-412c-bf4e-d1c57aeb1977
failureMode: unknown
terminatedAt: 2026-05-27T11:34:46.742Z
-->

# audit-delegate failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-3ec513cd-a5b6-4cd4-91c3-3641a30005a9
Started: 2026-05-27T11:33:47.580Z
Terminated: 2026-05-27T11:34:46.742Z
Duration: 59162ms
Last activity: 2026-05-27T11:34:46.603Z - respawned by self-planner (was error; attempts=1)
Last tool call: read_file
Branch: orch/agcli-audit/audit-delegate
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
