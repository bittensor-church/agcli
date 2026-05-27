<!-- orchestrate handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-2c8c9a3a-a338-4b1d-b72c-ed00d1c2716d
runId: run-c581318a-1445-4ca4-bf20-2f72b3bd841d
resultStatus: error
finishedAt: 2026-05-27T12:35:22.631Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m going to start by reading `.orchestrate/agcli-audit/discovery.md` as required, then trace the completions/update command handlers and gather exactly what needs to be documented and tested before making the two allowed file edits.I’ve loaded the discovery notes and next I’m mapping the exact completions/update subcommands from clap + handlers so the docs and parse tests cover the full surface with no misses.