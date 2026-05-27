<!-- orchestrate handoff
task: audit-contracts
branch: orch/agcli-audit/audit-contracts
agentId: bc-d16b4960-5269-489b-bbdb-aab03b36a517
runId: run-555c618a-7fdd-418c-a3f7-340351e1b8dd
resultStatus: error
finishedAt: 2026-05-27T11:17:07.517Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’ll start by reading `.orchestrate/agcli-audit/discovery.md` exactly as requested, then map every `contracts` subcommand to its handler and on-chain dispatchable before editing only the allowed docs/test files.I’ve loaded the discovery notes and next I’m checking branch state plus the contracts command surfaces so I can trace each subcommand to its pallet dispatchable before making docs/tests updates.