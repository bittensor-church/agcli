<!-- orchestrate handoff
task: audit-admin
branch: orch/agcli-audit/audit-admin
agentId: bc-f85e68a1-ec09-4e9b-9e79-4f4154fd48ae
runId: run-8d11a630-ca8e-4555-9159-5ff5e57c36b9
resultStatus: finished
finishedAt: 2026-05-27T11:49:15.594Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-admin`

## What I did
- Updated `docs/commands/admin.md` into a full admin audit reference that now covers every `AdminCommands` subcommand (`set-*`, `raw`, `list`) with:
  - clap flags and types
  - shared/command output JSON schema
  - exit code mapping from `src/error.rs`
  - pallet dispatchable mapping
  - SCALE argument shape sent by agcli
  - storage key targets in `subtensor/pallets/subtensor`
  - emitted event notes (including when none are emitted)
- Added `tests/audit_admin.rs` with:
  - a parse-surface test that calls `agcli::cli::Cli::try_parse_from` for every admin subcommand
  - one `#[ignore]` local-chain green-path integration test (`set-default-take` via sudo) that validates `tx_hash` JSON output when run against a local node
- Ran required compile verification:
  - `SKIP_METADATA_FETCH=1 cargo check --all-targets`
  - `SKIP_METADATA_FETCH=1 cargo test --no-run --test audit_admin`
- Committed and pushed the worker branch changes.

## Measurements
- admin subcommands documented in `docs/commands/admin.md`: 12 → 33
- admin parse-surface cases in `tests/audit_admin.rs`: 0 → 33
- `cargo check --all-targets` exit code: 0 == 0
- `cargo test --no-run --test audit_admin` exit code: 0 == 0

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- **Concrete drift: `set-max-weight-limit` currently targets a missing runtime dispatchable.**
  - agcli calls `AdminUtils.sudo_set_max_weight_limit` (`src/admin.rs`), but this function is absent from `subtensor/pallets/admin-utils/src/lib.rs` at the pinned commit.
  - Result: metadata precheck fails before submission.
- **Concrete drift: `set-subnet-moving-alpha` argument encoding does not match pallet type.**
  - agcli encodes `alpha` as `u128` (`src/admin.rs`), while runtime dispatchable expects `I96F32` (`admin-utils/src/lib.rs`).
  - This can fail with dispatch/type mismatch depending on runtime metadata handling.
- **Concrete drift: `set-mechanism-emission-split` argument shape mismatch.**
  - runtime expects `Option<Vec<u16>>` (`admin-utils/src/lib.rs`)
  - agcli currently parses CSV to `Vec<u64>` and sends unnamed composite vec without explicit `Option::Some` wrapper (`src/admin.rs`)
  - likely dispatch/encoding failure path.
- **Concrete drift: `set-pow-registration` is surfaced but pallet implementation is hard-disabled.**
  - `sudo_set_network_pow_registration_allowed` returns `POWRegistrationDisabled` directly in current pallet implementation.
  - command is parse-valid but not green-path executable.
- **Coverage gap: `admin raw` is not a true escape hatch.**
  - `validate_admin_call_name` restricts `raw` to `known_params`.
  - admin-utils currently has far more `sudo_set_*` dispatchables than agcli’s allowlist, so many valid runtime calls remain unreachable from `agcli admin raw`.
- No panicking path was observed in the audited admin handlers during static tracing. Failures are returned as errors and mapped into exit codes.

## Suggested follow-ups
- Fix `set-max-weight-limit` by either:
  - removing/deprecating the command and stale known param, or
  - implementing a real runtime-backed setter path if the pallet restores this dispatchable.
- Fix `set-subnet-moving-alpha` SCALE encoding to match `I96F32` runtime type.
- Fix `set-mechanism-emission-split` encoding to send `Option::Some(Vec<u16>)` (or explicit `None`) and validate `u16` bounds.
- Rework `admin raw` allowlist strategy so it can cover all runtime `AdminUtils` calls (or clearly rename it to “allowlisted raw” and expose full dynamic mode separately).
- Decide command policy for `set-pow-registration` since runtime behavior is currently hard-disabled; either hide or gate it with explicit deprecation messaging.