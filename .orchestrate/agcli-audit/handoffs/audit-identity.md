<!-- orchestrate handoff
task: audit-identity
branch: orch/agcli-audit/audit-identity
agentId: bc-14b494d8-3763-42b7-b630-46f5d74b1653
runId: run-36a39b86-6d27-4da9-9e4c-749a826fc20b
resultStatus: finished
finishedAt: 2026-05-27T13:12:15.157Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-identity`

## What I did

- **`tests/audit_identity.rs`** (new, ~400 LOC): 20 parse-surface tests covering all 4 `IdentityCommands` subcommands (`Show`, `Set`, `Clear`, `SetSubnet`) plus global-flag interaction tests (`--output json`, `--yes`, `--live` ordering), field-surface negative tests that assert absent flags (`--discord`, `--email`, `--logo-url`, etc.) cause parse failures, and 1 `#[ignore]`-gated localnet green-path placeholder with inline commentary explaining which steps will fail and why.
- **`docs/commands/identity.md`** (rewritten): All 4 subcommands documented with clap flags + types, exit codes mapped to `src/error.rs` constants, pallet refs + call indices (Registry call 0/1; SubtensorModule call 68/78), storage keys (`Registry.IdentityOf`, `SubtensorModule.SubnetIdentitiesV3`), on-chain events (`IdentitySet`, `IdentityDissolved`), and 10 concrete audit findings.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_identity`: compiled
- `cargo test --test audit_identity`: 20 passing, 0 failed, 1 ignored

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

The identity command group has the most severe bugs of any command group audited so far — two of the four subcommands (`set` and `clear`) will hard-fail at the chain level on every invocation due to wrong SCALE argument encoding, and a third (`set-subnet`) silently calls the completely wrong dispatchable. The `--live` ordering pitfall noted by the view audit worker also applies here.

## Findings

1. **`identity show` ignores `--output json/csv`** — `handle_identity` always calls `println!` unconditionally; `ctx.output` is never consulted. Agents relying on `--output json` receive unparseable human-readable text.

2. **`identity set` — wrong field names in SCALE composite** — `set_registry_identity` builds `Value::named_composite([("name",...), ("url",...), ("description",...), ("github_repo",...), ("image",...)])`. The Registry pallet's `IdentityInfo` has fields `display`, `web`, `riot`, `email`, `pgp_fingerprint`, `image`, `twitter`, `legal`, `additional`. The keys `name`, `description`, and `github_repo` do not exist in the pallet type; they will be silently dropped or cause a decode error depending on subxt's dynamic field matching.

3. **`identity show` hardcodes empty `github` and `description`** — `chain_identity_from_registration` sets `github: String::new()` and `description: String::new()`. The Registry pallet stores neither field by that name; the decoder never attempts to extract them from `additional` key-value pairs. The displayed output misrepresents what is stored on-chain.

4. **`identity set` — missing `identified` AccountId argument (critical)** — The Registry pallet's `set_identity` call signature is `(origin, identified: AccountId, info: Box<IdentityInfo>)`. `set_registry_identity` submits `vec![info]`, placing the `info` composite at the position of `identified`. Every invocation will fail with a SCALE decode error at the chain level.

5. **`identity clear` — missing `identified` AccountId argument (critical)** — `clear_registry_identity` submits `Registry::clear_identity` with `vec![]`. The pallet requires `(origin, identified: AccountId)`. Every invocation will fail with a SCALE decode error.

6. **`identity set-subnet` calls wrong dispatchable (critical)** — `Client::set_subnet_identity` calls `api::tx().subtensor_module().set_identity(...)` (call index 68, hotkey identity) instead of `SubtensorModule::set_subnet_identity` (call index 78). The `netuid` parameter is named `_netuid` and is entirely ignored. Calling `agcli identity set-subnet` therefore sets the **coldkey's own hotkey identity**, not the subnet's identity.

7. **`identity set-subnet` — `subnet_contact` passed as `image`** — Argument position 4 in the (wrong) `set_identity` call receives `identity.subnet_contact` where `image: Vec<u8>` is expected. No image URL is settable for subnets via the CLI.

8. **`SubnetIdentity` struct missing `logo_url`** — `SubnetIdentityV3` in the chain (subtensor/pallets/subtensor/src/lib.rs:317) has `pub logo_url: Vec<u8>`. `src/types/chain_data.rs::SubnetIdentity` does not have this field. Both the `--logo-url` CLI flag and the internal struct field are absent. This also means `get_subnet_identity` silently drops `logo_url` from the decoded result.

9. **`IdentityCommands::Set` missing flags for Registry pallet fields** — `IdentityInfo` supports `legal`, `riot` (Discord/Matrix handle), `email`, `twitter`, `pgp_fingerprint`, and arbitrary `additional` key-value pairs. The CLI only exposes `--name`, `--url`, `--github`, `--description`, `--image`. No `--discord`, `--email`, `--twitter`, `--legal` flags exist.

10. **`IdentityCommands::SetSubnet` missing flags** — `SubtensorModule::set_subnet_identity` accepts `discord`, `description`, `logo_url`, `subnet_contact`, `additional`. The CLI only exposes `--name`, `--github`, `--url`. Five settable fields have no CLI surface.

## Suggested follow-ups

- **Fix `identity set` / `identity clear` SCALE encoding** — prepend `identified` (coldkey AccountId) to the dynamic call args and align field names to `IdentityInfo` (`display`, `web`, `riot`, `email`, `image`, `twitter`).
- **Fix `identity set-subnet`** — change `Client::set_subnet_identity` to call `api::tx().subtensor_module().set_subnet_identity(netuid, ...)` and actually use the `netuid` parameter. Add `logo_url` to `SubnetIdentity` and pass it through.
- **Add JSON output to `identity show`** — route through `ctx.output` and serialize `ChainIdentity` as JSON; match the established pattern in other view subcommands.
- **Add missing flags** — `--discord` / `--email` / `--twitter` / `--legal` for `identity set`; `--discord` / `--description` / `--logo-url` / `--subnet-contact` for `identity set-subnet`.
- **Fix `chain_identity_from_registration`** — extract GitHub/description from `additional` key-value pairs rather than hardcoding empty strings.