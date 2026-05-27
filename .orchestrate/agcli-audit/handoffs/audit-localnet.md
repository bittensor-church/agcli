<!-- orchestrate handoff
task: audit-localnet
branch: orch/agcli-audit/audit-localnet
agentId: bc-eb9c21ad-602a-403e-b77b-26602e770c11
runId: run-ccd55fe4-b199-471f-aac4-c9cb8b49caff
resultStatus: finished
finishedAt: 2026-05-27T11:33:32.134Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-localnet`

## What I did
- Added `tests/audit_localnet.rs`:
  - Added parse-surface coverage using `agcli::cli::Cli::try_parse_from` for every `localnet` subcommand (`start`, `stop`, `status`, `reset`, `logs`, `scaffold`) with realistic arguments.
  - Added one `#[ignore]` async green-path integration test that starts a localnet container, checks status/block height, and stops it.
- Rewrote `docs/commands/localnet.md` to fully document all `localnet` subcommands with:
  - clap flags and concrete Rust types/default behavior
  - per-command exit-code behavior tied to `src/error.rs`
  - output JSON schemas (including nullable fields and non-JSON `logs` behavior)
  - pallet/storage/event references
  - scaffold call-path mapping through `SubtensorModule`, `Balances`, `AdminUtils`, and `Sudo`
  - explicit note that `seed` is intentionally not serialized in scaffold JSON output
- Audited handler + implementation flow end-to-end across:
  - `src/cli/localnet_cmds.rs`
  - `src/localnet.rs`
  - `src/scaffold.rs`
  - chain call sites in `src/chain/extrinsics.rs` and `src/admin.rs`
  - subtensor dispatch/events/storage references in `subtensor/pallets/**`

## Measurements
- `tests/audit_localnet.rs parse-surface tests: 0 → 6`
- `tests/audit_localnet.rs ignored localchain tests: 0 → 1`
- `cargo check --all-targets exit code: 0 == 0`
- `cargo test --no-run --test audit_localnet exit code: 0 == 0`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- **Finding 1 (runtime drift):** `scaffold` attempts `AdminUtils.sudo_set_max_weight_limit`, but this dispatchable is absent in current subtensor runtime metadata (`pallets/admin-utils/src/lib.rs` has no such call). Current behavior is metadata-guarded warning/skip, not panic.
- **Finding 2 (docs drift):** previous localnet docs showed scaffold JSON including `seed`; code uses `#[serde(skip)]` on `NeuronResult.seed`, so `seed` is not emitted in JSON.
- **Finding 3 (exit-code mismatch):** localnet readiness failure (`"did not become ready after ... seconds"`) currently classifies to generic exit `1`, not timeout `15`, because the message does not match timeout heuristics in `src/error.rs`.
- **Finding 4 (idempotency/exit behavior):** `localnet stop` on missing container returns generic error (`1`) rather than a clean no-op/specialized code.
- **Finding 5 (format drift):** `localnet logs` ignores output mode and always prints raw text; `--output json` does not produce JSON.
- **Finding 6 (validation split):** CLI helper accepts `u16` ports except `0`, but `localnet::start/reset` additionally reject `>=65534` due to dual-port mapping (`p` and `p+1`); this was undocumented before and is now documented.
- I did not execute the ignored Docker integration test in this VM; compilation-only verification was performed per the required commands.

## Suggested follow-ups
- Add/align `AdminUtils.sudo_set_max_weight_limit` handling: either implement against current runtime-supported call(s) or remove from scaffold attempt list to avoid recurring warning drift.
- Tighten `src/error.rs` classification for localnet-specific readiness timeout and missing-container stop/log paths so they map to explicit timeout/IO semantics instead of generic `1`.
- Decide and document whether `localnet logs --output json` should stay raw-text by design or return structured JSON payload for agent consumers.