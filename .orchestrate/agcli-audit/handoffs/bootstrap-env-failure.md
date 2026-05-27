<!-- orchestrate failure handoff
task: bootstrap-env
branch: orch/agcli-audit/bootstrap-env
agentId: bc-05e45c90-5c21-4902-b6b9-13281d5bade4
runId: run-98d0d654-8d69-4721-8667-9df1bb6a0e38
failureMode: unknown
terminatedAt: 2026-05-27T11:16:25.775Z
-->

# bootstrap-env failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-05e45c90-5c21-4902-b6b9-13281d5bade4
Started: 2026-05-27T11:15:38.024Z
Terminated: 2026-05-27T11:16:25.775Z
Duration: 47751ms
Last activity: 2026-05-27T11:16:25.689Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/bootstrap-env
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
