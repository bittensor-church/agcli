<!-- orchestrate handoff
task: audit-stake
branch: orch/agcli-audit/audit-stake
agentId: bc-30e345f5-d3d5-4760-9de5-1c35bc012abb
runId: run-254b5e4f-e4cd-4707-b154-a619f2c833e7
resultStatus: error
finishedAt: 2026-05-27T11:16:29.962Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the required discovery doc and then mapping every `stake` subcommand from CLI parse surface to its handler and underlying subtensor dispatchable/event/storage references so I can update docs and build the dedicated audit test file accurately.I’ve loaded the discovery context and next I’ll inspect `StakeCommands` + `stake_cmds.rs` + relevant chain/extrinsic code to build a complete subcommand-to-dispatchable mapping before editing docs/tests.I found the repo currently doesn’t have a checked-out `subtensor/` tree, so I’m continuing by tracing stake handlers through `src/chain/*` and `src/error.rs` to verify dispatch names, SCALE argument order/types, and exit-code behavior directly from agcli’s runtime call layer.