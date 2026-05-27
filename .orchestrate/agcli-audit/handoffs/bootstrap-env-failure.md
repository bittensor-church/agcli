<!-- orchestrate failure handoff
task: bootstrap-env
branch: orch/agcli-audit/bootstrap-env
agentId: bc-010abe85-8c77-48b3-bf6a-cc8c439faa7e
runId: run-036803b9-ef9e-4d59-8bba-fae26552752a
failureMode: unknown
terminatedAt: 2026-05-27T11:13:20.171Z
-->

# bootstrap-env failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-010abe85-8c77-48b3-bf6a-cc8c439faa7e
Started: 2026-05-27T11:12:27.365Z
Terminated: 2026-05-27T11:13:20.171Z
Duration: 52806ms
Last activity: 2026-05-27T11:13:20.063Z
Last tool call: run_terminal_cmd
Branch: orch/agcli-audit/bootstrap-env
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
