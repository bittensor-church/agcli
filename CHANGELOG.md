# Changelog

All notable user-facing changes to agcli.

## Unreleased

### Breaking changes

- **`safe-mode force-enter`**: removed `--duration`. Duration is set by chain config (`ForceEnterOrigin`), not CLI flags. SDK: use `safe_mode_force_enter(pair)`; deprecated `safe_mode_force_enter_with_duration(pair, _duration)`.
- **`swap coldkey`**: now calls `announce_coldkey_swap` (step 1). Add **`swap coldkey-exec --new-coldkey SS58`** for step 2 after the cooldown. Check status with `agcli wallet check-swap`.
- **`swap evm-key`**: **`--netuid` is required** (signatures are scoped per subnet). `--block-number` is now `u64`. SDK: `associate_evm_key(pair, netuid, …)`; deprecated `associate_evm_key_legacy` uses netuid 0.
- **`subnet register-leased`**: signs with **coldkey** (not hotkey). New **`--emissions-share 0-100`** (default 100). SDK: `register_leased_network(pair, emissions_share, end_block)`; deprecated `register_leased_network_with_hotkey`.
- **Alpha stake amounts**: SDK methods `remove_stake`, `move_stake`, `swap_stake`, `remove_stake_limit`, `swap_stake_limit`, `recycle_alpha`, and `burn_alpha` now take **`AlphaBalance`** (not `Balance`/TAO). CLI `--amount` for those commands is **alpha (α)** with client-side preflight on remove/move/swap/limit/recycle/burn.

### Deprecations (removed in 0.2.0)

- Query helpers `*_pinned` → use `*_at_block` equivalents.
- SDK `schedule_swap_coldkey` → use `announce_swap_coldkey`.

### Fixed

- Correct SCALE encoding for `SafeMode::force_enter`, coldkey swap announce/exec, `associate_evm_key` netuid, and `register_leased_network` emissions share.
- Query cache invalidation clears at-block disk entries.
- Crowdloan contributors: storage item (`Contributions` vs legacy `Contributors`) chosen from runtime metadata; **errors** if neither exists (no silent fallback).
- `wallet check-swap` hints at `swap coldkey-exec` step 2 with block countdown.
- `build.rs`: when building with `test-utils` and `metadata.scale` is missing, fetches metadata even if `SKIP_METADATA_FETCH` is set.

### Added

- `tests/extrinsic_encoding.rs` — encoding regression tests (`cargo test --features test-utils --test extrinsic_encoding`). **Metadata refresh:** after updating chain metadata in `build.rs`, re-run that test suite and update golden bytes in `safe_mode_force_enter_golden_call_data` if it fails.
- Release workflow smoke test: `./target/release/agcli --version`.
