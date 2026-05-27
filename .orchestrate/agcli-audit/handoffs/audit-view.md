<!-- orchestrate handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-51349619-31d5-4adf-b645-a58b4c14f033
runId: run-71e3a518-1a52-45a6-bcf4-a45bc84731f0
resultStatus: error
finishedAt: 2026-05-27T12:35:07.992Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

I’m going to audit the `view` command surface end-to-end now: first I’ll read `.orchestrate/agcli-audit/discovery.md`, map every `ViewCommands` subcommand to its handler and chain call paths, then I’ll update `docs/commands/view.md` and add `tests/audit_view.rs` before running the required cargo checks.