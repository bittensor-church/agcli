<!-- orchestrate failure handoff
task: audit-weights
branch: orch/agcli-audit/audit-weights
agentId: bc-615c2871-2bec-45ed-866e-85e7d5767845
runId: run-9cc8ecd5-514e-43f9-927c-a03dc4a0ce85
failureMode: unknown
terminatedAt: 2026-05-27T11:34:30.190Z
-->

# audit-weights failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-615c2871-2bec-45ed-866e-85e7d5767845
Started: 2026-05-27T11:33:41.868Z
Terminated: 2026-05-27T11:34:30.190Z
Duration: 48322ms
Last activity: 2026-05-27T11:34:30.087Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-weights
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
