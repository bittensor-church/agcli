<!-- orchestrate failure handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-bba5f8c6-02c8-4e7b-a649-5fca070bba2a
runId: run-9d7ae039-de3d-4d3c-af97-18ae8f1ad2de
failureMode: unknown
terminatedAt: 2026-05-27T11:16:33.089Z
-->

# audit-view failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-bba5f8c6-02c8-4e7b-a649-5fca070bba2a
Started: 2026-05-27T11:16:03.290Z
Terminated: 2026-05-27T11:16:33.089Z
Duration: 29799ms
Last activity: 2026-05-27T11:16:33.020Z
Last tool call: read_file
Branch: orch/agcli-audit/audit-view
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
