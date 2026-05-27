<!-- orchestrate failure handoff
task: audit-contracts
branch: orch/agcli-audit/audit-contracts
agentId: bc-90359b80-dc83-460b-b669-c12e2abb64c1
runId: run-114b8196-a496-461d-9e32-60461854be5b
failureMode: unknown
terminatedAt: 2026-05-27T11:35:04.555Z
-->

# audit-contracts failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-90359b80-dc83-460b-b669-c12e2abb64c1
Started: 2026-05-27T11:34:14.011Z
Terminated: 2026-05-27T11:35:04.555Z
Duration: 50544ms
Last activity: 2026-05-27T11:35:04.454Z - respawned by self-planner (was error; attempts=1)
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/audit-contracts
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
