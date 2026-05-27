<!-- orchestrate failure handoff
task: audit-identity
branch: orch/agcli-audit/audit-identity
agentId: bc-16c3b80c-ac3a-4709-a6fa-749967ea73f7
runId: run-b19f0f9e-7621-46ef-9971-d37743c86863
failureMode: unknown
terminatedAt: 2026-05-27T11:16:31.977Z
-->

# audit-identity failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-16c3b80c-ac3a-4709-a6fa-749967ea73f7
Started: 2026-05-27T11:16:06.096Z
Terminated: 2026-05-27T11:16:31.977Z
Duration: 25881ms
Last activity: 2026-05-27T11:16:30.930Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-identity
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
