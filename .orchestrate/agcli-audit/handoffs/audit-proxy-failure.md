<!-- orchestrate failure handoff
task: audit-proxy
branch: orch/agcli-audit/audit-proxy
agentId: bc-5d4ea403-1ec8-4c41-9abf-f8ca2ce6aaf7
runId: run-b9c41160-88e1-4de1-97e7-8936f09a3e9b
failureMode: unknown
terminatedAt: 2026-05-27T11:34:50.206Z
-->

# audit-proxy failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-5d4ea403-1ec8-4c41-9abf-f8ca2ce6aaf7
Started: 2026-05-27T11:33:58.961Z
Terminated: 2026-05-27T11:34:50.206Z
Duration: 51245ms
Last activity: 2026-05-27T11:34:49.790Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-proxy
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
