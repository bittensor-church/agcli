<!-- orchestrate failure handoff
task: audit-subscribe
branch: orch/agcli-audit/audit-subscribe
agentId: bc-3e84313e-445e-4bbd-969a-145a7694c7cf
runId: run-6133fb68-323a-4c19-94b3-43810de65a44
failureMode: unknown
terminatedAt: 2026-05-27T11:35:40.507Z
-->

# audit-subscribe failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-3e84313e-445e-4bbd-969a-145a7694c7cf
Started: 2026-05-27T11:34:05.106Z
Terminated: 2026-05-27T11:35:40.507Z
Duration: 95401ms
Last activity: 2026-05-27T11:35:40.409Z - respawned by self-planner (was error; attempts=1)
Last tool call: read_file
Branch: orch/agcli-audit/audit-subscribe
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
