<!-- orchestrate handoff
task: audit-batch
branch: orch/agcli-audit/audit-batch
agentId: bc-d1549f7b-6bdd-402b-a6af-3a996097019d
runId: run-dfd9e7e8-3a1b-4d7a-bce5-00d5d6095fa8
resultStatus: error
finishedAt: 2026-05-27T11:35:17.630Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the required discovery file first, then I’ll map every `batch` subcommand to its handler and chain dispatch details before editing docs/tests.I’ve read the discovery notes and next I’m tracing the `batch` command surface in `src/cli` and its handler wiring to identify exact subcommands and extrinsic calls.