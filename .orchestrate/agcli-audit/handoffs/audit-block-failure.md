<!-- orchestrate failure handoff
task: audit-block
branch: orch/agcli-audit/audit-block
agentId: bc-7efacc27-b8d6-4f3a-af11-f18d275347e1
runId: run-e5ec2b03-a622-4e60-8645-68b8a0fa5699
failureMode: unknown
terminatedAt: 2026-05-27T11:18:20.008Z
-->

# audit-block failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-7efacc27-b8d6-4f3a-af11-f18d275347e1
Started: 2026-05-27T11:16:59.893Z
Terminated: 2026-05-27T11:18:20.008Z
Duration: 80115ms
Last activity: 2026-05-27T11:18:19.749Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-block
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
