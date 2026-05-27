<!-- orchestrate handoff
task: audit-completions-update
branch: orch/agcli-audit/audit-completions-update
agentId: bc-91326555-9c63-4de9-8f79-5469bebe6183
runId: run-9422a96b-2e4c-4fc4-a64c-a50fc678723e
resultStatus: error
finishedAt: 2026-05-27T11:35:19.589Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

Understood: I’ll audit the `completions`/`update` command surfaces end-to-end, only edit `docs/commands/utils.md` and `tests/audit_completions_update.rs`, and finish with the required handoff format. First I’m reading `.orchestrate/agcli-audit/discovery.md` and then tracing handlers from the CLI command enums into their implementations.I’ve loaded the discovery notes and now I’m mapping the exact `Completions` and `Update` command variants from `src/cli/mod.rs` / `src/cli/system_cmds.rs`, then I’ll trace each handler path and collect pallet/event/exit-code evidence for the docs and test updates.I’ve confirmed `Completions` is a single top-level command (`--shell` enum) and `Update` is a single top-level command with no args, both in `system_cmds.rs` with no subxt call path. Next I’m harvesting test patterns from the existing `tests/` suite so the new `audit_completions_update.rs` fits project conventions, including an ignored local-chain integration probe.