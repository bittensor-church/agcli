<!-- orchestrate failure handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-91326555-9c63-4de9-8f79-5469bebe6183
runId: run-9422a96b-2e4c-4fc4-a64c-a50fc678723e
failureMode: unknown
terminatedAt: 2026-05-27T11:35:19.589Z
-->

# audit-completions-update failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-91326555-9c63-4de9-8f79-5469bebe6183
Started: 2026-05-27T11:34:42.218Z
Terminated: 2026-05-27T11:35:19.589Z
Duration: 37371ms
Last activity: 2026-05-27T11:35:19.508Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-completions-update
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
