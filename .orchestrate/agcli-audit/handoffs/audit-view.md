<!-- orchestrate handoff
task: audit-view
branch: orch/agcli-audit/audit-view
agentId: bc-b0077730-c33f-45a8-92b7-105a1b5691e0
runId: run-b5112d78-a609-44b9-87e4-c5a66bf1d963
resultStatus: error
finishedAt: 2026-05-27T11:34:16.007Z
-->

> ⚠️ Run ended with `status=error`. No structured handoff produced — the content below is the worker's raw output up to the point of failure.

Starting the audit by reading the orchestrator discovery doc first, then I’ll enumerate every `view` subcommand/handler mapping and trace each one into chain queries before drafting docs and tests.I’ve loaded the discovery context and next I’m mapping `ViewCommands` to handlers and chain calls so the docs/test additions are complete and exact.