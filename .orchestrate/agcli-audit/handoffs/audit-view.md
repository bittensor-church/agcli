<!-- orchestrate handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-bba5f8c6-02c8-4e7b-a649-5fca070bba2a
runId: run-9d7ae039-de3d-4d3c-af97-18ae8f1ad2de
resultStatus: error
finishedAt: 2026-05-27T11:16:33.089Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m starting by reading the required discovery doc, then I’ll inventory every `view` subcommand from `src/cli/view_cmds.rs`, trace each handler to its chain query/runtime API, and use that to update `docs/commands/view.md` and create `tests/audit_view.rs` before running `cargo check --all-targets`.I’ve loaded the discovery notes and `view_cmds` implementation; next I’m mapping `ViewCommands` clap surfaces from `src/cli/mod.rs` and tracing each client query to its exact pallet storage/runtime API references so the docs and new audit test cover everything.