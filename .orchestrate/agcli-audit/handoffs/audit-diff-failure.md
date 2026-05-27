<!-- orchestrate failure handoff
task: audit-diff
branch: orch/agcli-audit/audit-diff
agentId: bc-02a44011-4228-4457-bfb5-bbe0d3745142
runId: run-1e7dae5b-c3c9-49b9-96a0-69b48fafd9ef
failureMode: unknown
terminatedAt: 2026-05-27T11:35:46.668Z
-->

# audit-diff failure handoff

Status: error (cloud agent terminated without writing a handoff)
Failure mode: unknown
Cloud agent: bc-02a44011-4228-4457-bfb5-bbe0d3745142
Started: 2026-05-27T11:34:33.083Z
Terminated: 2026-05-27T11:35:46.668Z
Duration: 73585ms
Last activity: 2026-05-27T11:35:46.548Z - respawned by self-planner (was error; attempts=1)
Last tool call: grep_search
Branch: orch/agcli-audit/audit-diff
SDK error: (none recorded)

## Suggested next steps
- Retry as-is (treat as transient)
- Retry with smaller scope if this repeats
- Retry with different model if the same tool keeps failing
- Abandon: skip task, replan around it
