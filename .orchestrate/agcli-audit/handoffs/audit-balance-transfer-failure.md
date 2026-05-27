<!-- orchestrate failure handoff
task: audit-balance-transfer
branch: orch/agcli-audit/audit-balance-transfer
agentId: bc-c0076afd-23f9-4e0a-8492-9b26006db123
runId: run-0674eb98-2472-4158-860e-81620212f018
failureMode: unknown
terminatedAt: 2026-05-27T11:34:31.292Z
-->

# audit-balance-transfer failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-c0076afd-23f9-4e0a-8492-9b26006db123
Started: 2026-05-27T11:33:33.092Z
Terminated: 2026-05-27T11:34:31.292Z
Duration: 58200ms
Last activity: 2026-05-27T11:34:30.085Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-balance-transfer
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
