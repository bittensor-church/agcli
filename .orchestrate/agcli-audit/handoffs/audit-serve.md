<!-- orchestrate handoff
task: audit-serve
branch: orch/agcli-audit/audit-serve
agentId: bc-73bc8a38-f007-4add-b081-b2399b23ac86
runId: run-41894a69-254b-44fd-afc6-58f7f4ede960
resultStatus: error
finishedAt: 2026-05-27T11:17:04.292Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the required orchestration discovery document first, then I’ll map every `serve` subcommand to its handler and corresponding subtensor pallet dispatchables before touching docs/tests.I’ve loaded the discovery notes and next I’m tracing the `serve` command surface from clap definitions through `handle_serve` into chain extrinsics, then I’ll codify any drift in docs and tests only.I confirmed we’re on the expected starting branch; I’m now creating the required worker branch and then extracting the exact clap flag/type surface for all `serve` subcommands.I’ve mapped the CLI and extrinsic call sites; next I’m locating the exact subtensor pallet sources in the submodule to verify dispatchable signatures and events byte-for-byte.