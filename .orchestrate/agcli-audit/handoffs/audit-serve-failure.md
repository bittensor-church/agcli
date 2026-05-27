<!-- orchestrate failure handoff
task: audit-serve
branch: orch/agcli-audit/audit-serve
agentId: bc-999a8cf2-3d14-4c06-804f-4d9a50a95a11
runId: run-7b59a8de-9a5b-47bb-8614-ccc58907363a
failureMode: unknown
terminatedAt: 2026-05-27T11:34:18.170Z
-->

# audit-serve failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-999a8cf2-3d14-4c06-804f-4d9a50a95a11
Started: 2026-05-27T11:33:55.817Z
Terminated: 2026-05-27T11:34:18.170Z
Duration: 22353ms
Last activity: 2026-05-27T11:34:18.095Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-serve
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
