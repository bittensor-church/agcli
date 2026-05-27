<!-- orchestrate failure handoff
task: audit-block
branch: orch/agcli-audit/audit-block
agentId: bc-363b84fe-639f-45f1-9be1-86c6737981ab
runId: run-443b85fe-9ca4-4946-a6b3-fd3a684790c8
failureMode: unknown
terminatedAt: 2026-05-27T11:35:10.551Z
-->

# audit-block failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-363b84fe-639f-45f1-9be1-86c6737981ab
Started: 2026-05-27T11:34:29.150Z
Terminated: 2026-05-27T11:35:10.551Z
Duration: 41401ms
Last activity: 2026-05-27T11:35:10.403Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-block
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
