<!-- orchestrate handoff
task: fix-explain-topics
branch: orch/agcli-audit/fix-explain-topics
agentId: bc-2e1e16b9-b7d8-433a-b457-df8431b01331
runId: run-386180aa-5f1f-4a1b-811b-c42e5c6a320b
resultStatus: finished
finishedAt: 2026-05-27T13:56:50.659Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-explain-topics`

## What I did
- Updated `src/utils/explain.rs` to expand built-in topic coverage and aliases:
  - Added canonical topics + content for:
    - `multisig`
    - `scheduler`
    - `drand`
    - `safe-mode`
    - `swap`
    - `evm` (EVM bridge concept)
    - `contracts`
    - `ss58-vs-evm-h160`
  - Added corresponding alias routing in `explain()` (including hyphen/underscore-normalized forms).
  - Added `dynamic` alias to AMM topic (`amm` / Dynamic TAO concept).
  - Updated `list_topics()` inline so these topics appear in `agcli explain` topic listings.
- Added/extended tests in `src/utils/explain.rs` for:
  - known-topic presence of all newly added topics
  - alias behavior (`evm-h160` alias for `ss58-vs-evm-h160`)
  - hyphen normalization checks for `safe-mode` and `ss58-vs-evm-h160`
- Verified `--full` doc resolution behavior against docs that exist:
  - Confirmed `agcli --output json explain --topic <topic> --full` resolves to `docs/commands/<topic>.md` for:
    - `multisig`, `scheduler`, `drand`, `safe-mode`, `swap`, `evm`, `contracts`

## Measurements
- `cargo check --all-targets (exit code): 0 == 0`
- `cargo build --bin agcli (exit code): 0 == 0`
- `explain --full resolution checks for new doc-backed topics: 7/7 == 7/7`

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- Handoff Findings (fix → motivating upstream audit handoff):
  - Added `multisig` concept topic → motivated by `audit-multisig`
  - Added `scheduler` concept topic → motivated by `audit-scheduler`
  - Added `drand` concept topic → motivated by `audit-drand`
  - Added `safe-mode` concept topic → motivated by `audit-safe-mode`
  - Added `swap` concept topic → motivated by `audit-swap-keys`
  - Added `evm` / bridge concept topic → motivated by `audit-evm`
  - Added `contracts` concept topic → motivated by `audit-contracts`
  - Added `ss58-vs-evm-h160` topic for address-domain guidance → motivated by cross-cutting findings in `audit-evm` + `audit-swap-keys`
  - Expanded AMM aliases (`dynamic`) to strengthen AMM/dynamic discoverability → motivated by `audit-view` + `audit-liquidity` context
- No files outside allowed scope were modified.
- Rust toolchain had to be upgraded in-session to pass current dependency requirements (`cargo/rustc 1.95.0`), because `1.83.0` failed on `edition2024` dependency metadata.

## Suggested follow-ups
- Update docs metadata that still says “32 topics” (`docs/commands/explain.md`, `docs/llm.txt`) to reflect the expanded built-in topic set.
- Consider centralizing topic-to-doc alias mapping so `--full` can reuse `explain()` canonicalization directly (single source of truth).
- Environment setup recommendation for future cloud agents (since toolchain upgrade was required):
  - Run an env setup agent from Cursor web and use prompt:
    - “Update this repo’s cloud agent environment to default to Rust stable 1.95+ (cargo with edition2024 support), verify `cargo check --all-targets` works without manual rustup updates, and persist that in startup image/config.”