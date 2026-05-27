<!-- orchestrate handoff
task: fix-chain-extrinsics
branch: orch/agcli-audit/fix-chain-extrinsics
agentId: bc-45dcf513-ce1e-4bae-a1d0-c1246bb1b063
runId: run-f5c64366-cc29-4f11-b660-6994c87289fb
resultStatus: finished
finishedAt: 2026-05-27T14:02:57.914Z
-->

## Status
success

## Branch
`orch/agcli-audit/fix-chain-extrinsics`

## What I did
- Updated `src/chain/extrinsics.rs` to fix cross-cutting submitter issues in the extrinsics layer:
  - **Identity submitters**
    - Fixed `set_subnet_identity` to call `SubtensorModule::set_subnet_identity` (not `set_identity`), include `netuid`, and include `logo_url` (empty fallback).
    - Fixed `register_network_with_identity` to use typed `SubnetIdentityV3` including `logo_url`.
    - Fixed `set_registry_identity` encoding for `Registry::set_identity(identified, info)`:
      - prepends `identified` account arg,
      - uses correct `IdentityInfo` field names (`display`, `web`, etc.),
      - maps `github`/`description` into `additional` kv entries.
    - Fixed `clear_registry_identity` to include required `identified` argument.
  - **Proxy submitters**
    - Fixed `proxy_announce`, `proxy_announced`, and `proxy_reject_announcement` to encode `AccountIdLookup` args as `MultiAddress::Id` variants.
    - Fixed `proxy_announced` to pass runtime call value (`RuntimeCall`) instead of encoded bytes for the `call` field.
  - **Safe-mode / sudo / finalization handling**
    - Fixed `safe_mode_force_enter` to stop encoding deprecated `duration` arg and route through checked sudo submit.
    - Switched `safe_mode_force_exit` to checked sudo submit.
    - Updated `submit_sudo_raw_call_checked` to:
      - acquire tx lock (nonce safety),
      - honor global finalization timeout via `tokio::time::timeout`.
  - **Lease / swap / evm submitters**
    - Changed `schedule_swap_coldkey` to use `announce_coldkey_swap` with `Blake2` hash, avoiding deprecated `schedule_swap_coldkey` dispatch.
    - Fixed `register_leased_network` call shape to current runtime signature `(emissions_share, end_block)`.
    - Fixed `terminate_lease` call shape by resolving `lease_id` from `SubnetUidToLeaseId`, loading lease, then submitting `(lease_id, lease.hotkey)`.
    - Fixed `associate_evm_key` call shape to include `netuid` and `u64 block_number` via typed call.

## Measurements
- `SKIP_METADATA_FETCH=1 cargo check --all-targets`: fail (compile error in updated extrinsics during first pass) → pass
- `SKIP_METADATA_FETCH=1 cargo build --bin agcli`: not run after failing check → pass

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback
- ### Handoff Findings
  - **Fixed identity call-shape bugs** motivated by:
    - `audit-identity` (critical wrong `Registry::set_identity`/`clear_identity` args; wrong `set-subnet` dispatchable),
    - `audit-subnet` (missing `logo_url` in subnet identity encoding).
  - **Fixed proxy announcement account encoding** motivated by:
    - `audit-proxy` (missing `MultiAddress::Id` wrapping for announce/reject/proxy-announced paths).
  - **Fixed safe-mode force-enter arg shape + sudo finalization behavior** motivated by:
    - `audit-safe-mode` (force-enter had spurious arg),
    - cross-cutting requirement to honor global finalization timeout for sudo-wrapped checked submits.
  - **Fixed lease extrinsic shapes** motivated by:
    - `audit-subnet` (register/terminate lease signature drift).
  - **Fixed coldkey swap path away from deprecated dispatch** motivated by:
    - `audit-swap-keys` (deprecated `schedule_swap_coldkey` usage).
  - **Fixed EVM associate call shape** motivated by:
    - `audit-swap-keys` / `audit-evm` (missing netuid + block number width mismatch).
- I had to install/update Rust toolchain to `stable 1.95.0` in this cloud environment to satisfy the lockfile/dependency edition requirements before checks could pass.
- No UI/interactive bug scope here; no screen recording artifact applicable.

## Suggested follow-ups
- Add CLI surface for newly required runtime args to remove current fallbacks:
  - `swap evm-key --netuid`,
  - `subnet register-leased --emissions-share`,
  - `subnet terminate-lease --lease-id --hotkey` (or explicit resolver UX).
- Add explicit two-phase coldkey swap CLI flow:
  - `announce-coldkey-swap` + `swap-coldkey-announced`.
- Consider replacing remaining fragile dynamic submits with typed `api::tx()` where metadata already supports them.
- Environment setup recommendation (for future cloud workers): set default toolchain to Rust stable 1.95+ in cloud agent image/startup config to avoid edition-2024 cargo parse failures.