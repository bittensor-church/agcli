<!-- orchestrate failure handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-2c8c9a3a-a338-4b1d-b72c-ed00d1c2716d
runId: run-c581318a-1445-4ca4-bf20-2f72b3bd841d
failureMode: unknown
terminatedAt: 2026-05-27T12:35:22.631Z
-->

# audit-completions-update failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-2c8c9a3a-a338-4b1d-b72c-ed00d1c2716d
Started: 2026-05-27T12:34:57.966Z
Terminated: 2026-05-27T12:35:22.631Z
Duration: 24665ms
Last activity: 2026-05-27T12:35:22.544Z - respawned by self-planner (was error; attempts=2)
Last tool call: read_file
Branch: orch/agcli-audit/audit-completions-update
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
