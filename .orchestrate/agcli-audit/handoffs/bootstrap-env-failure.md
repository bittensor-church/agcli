<!-- orchestrate failure handoff
task: bootstrap-env
branch: orch/agcli-audit/bootstrap-env
agentId: bc-ed7fca28-2515-4635-8cbd-ffea1c12fa85
runId: run-c067ec55-8752-4ff4-aa56-ec3d37ac81f6
failureMode: unknown
terminatedAt: 2026-05-27T11:34:19.135Z
-->

# bootstrap-env failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-ed7fca28-2515-4635-8cbd-ffea1c12fa85
Started: 2026-05-27T11:33:21.805Z
Terminated: 2026-05-27T11:34:19.135Z
Duration: 57330ms
Last activity: 2026-05-27T11:34:18.097Z - respawned by self-planner (was error; attempts=2)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/bootstrap-env
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
