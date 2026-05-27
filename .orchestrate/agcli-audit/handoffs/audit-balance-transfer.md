<!-- orchestrate handoff
task: audit-balance-transfer
branch: orch/agcli-audit/audit-balance-transfer
agentId: bc-f437e611-308d-4bc5-9c59-02daf62d3a55
runId: run-077e301b-8227-4845-a178-2a184578e762
resultStatus: error
finishedAt: 2026-05-27T11:16:49.559Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the required discovery doc, then I’ll trace every balance/transfer-related CLI subcommand through its handler to the subxt call and compare it against the balances pallet expectations before editing docs and adding the scoped test file.