<!-- orchestrate handoff
task: fix-scaffold-localnet
branch: orch/agcli-audit/fix-scaffold-localnet
agentId: bc-72f0a8cb-80cd-4ac4-946a-7e4e9a071b84
runId: run-3292c4ea-6697-4b17-90bb-ad165e41a382
resultStatus: error
finishedAt: 2026-05-27T13:50:49.187Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by checking the required workflow guidance and then I’ll inspect the localnet/scaffold code paths plus upstream handoffs to implement the cross-cutting fixes in `src/scaffold.rs`, `src/localnet.rs`, and `examples/scaffold.toml`.I confirmed the repo is clean on `cursor/agcli-subtensor-audit-e2e7`. Next I’m creating the required worker branch, then I’ll inspect scaffold/localnet code and upstream handoff notes to apply targeted fixes.I installed Doppler and attempted secret lookup, but this VM session has no Doppler token, so secret reads are blocked. I’m now auditing `src/scaffold.rs`, `src/localnet.rs`, and `examples/scaffold.toml` directly against your 4 requirements.