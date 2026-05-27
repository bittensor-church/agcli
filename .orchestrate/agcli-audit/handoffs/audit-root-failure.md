<!-- orchestrate failure handoff
task: audit-root
branch: orch/agcli-audit/audit-root
agentId: bc-a9b64a52-0750-4bdb-aacb-76bcde16a36b
runId: run-59176344-4221-4cc7-83ba-60cc03c2ce0a
failureMode: unknown
terminatedAt: 2026-05-27T11:34:41.035Z
-->

# audit-root failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-a9b64a52-0750-4bdb-aacb-76bcde16a36b
Started: 2026-05-27T11:33:44.849Z
Terminated: 2026-05-27T11:34:41.035Z
Duration: 56186ms
Last activity: 2026-05-27T11:34:40.684Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-root
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
