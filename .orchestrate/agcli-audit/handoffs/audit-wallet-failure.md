<!-- orchestrate failure handoff
task: audit-wallet
branch: orch/agcli-audit/audit-wallet
agentId: bc-9b380bd6-7b10-456f-8921-d351f2c5522a
runId: run-dbb80bce-4055-42a5-8acc-21b56decb98d
failureMode: unknown
terminatedAt: 2026-05-27T11:34:12.235Z
-->

# audit-wallet failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-9b380bd6-7b10-456f-8921-d351f2c5522a
Started: 2026-05-27T11:33:25.383Z
Terminated: 2026-05-27T11:34:12.235Z
Duration: 46852ms
Last activity: 2026-05-27T11:34:12.147Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-wallet
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
