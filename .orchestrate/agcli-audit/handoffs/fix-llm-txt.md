<!-- orchestrate handoff
task: fix-llm-txt
branch: orch/agcli-audit/fix-llm-txt
agentId: bc-8522d10c-2005-42f9-af14-b0c799c5a134
runId: run-6b618028-6528-4789-bc8a-6d040b261732
resultStatus: finished
finishedAt: 2026-05-27T14:09:07.072Z
-->

Branch pushed. No PR to open per task instructions (planner owns integration). Here is the handoff:

---

## Status
success

## Branch
`orch/agcli-audit/fix-llm-txt`

## What I did

**`docs/llm.txt`** (full rewrite, 380 → 491 lines):

- **`--live` ordering pitfall** (motivated by `audit-view`): Changed all examples from `agcli --live view dynamic` (fails — clap greedily consumes `view` as the optional seconds value) to `agcli view dynamic --live`. Added explicit note in Global Flags table.
- **Exit codes** (motivated by all audits): Replaced terse `0 = success, 1 = error` with a full 8-row exit code table covering codes 0, 1, 10–15.
- **`utils convert` flags** (motivated by `audit-utils-cli`, `audit-completions-update`): Removed non-existent `--rao` flag. Added four correct usage forms: `--tao X --netuid N`, `--alpha X --netuid N`, `--amount X` (RAO→TAO), `--amount X --to-rao` (TAO→RAO).
- **`utils latency`** (motivated by `audit-utils-cli`): Corrected `--count` (does not exist) to `--pings N`.
- **Wallet security guard** (motivated by `audit-wallet`): Changed `wallet import --mnemonic "..."` examples to use `AGCLI_MNEMONIC` env var; added note that `--mnemonic` on the command line is refused (visible in `ps` output).
- **`coldkeypub.txt` description** (motivated by `audit-wallet`): Corrected "SS58 address" to "64-hex-char raw pubkey (no `0x` prefix)".
- **Missing wallet subcommands** (motivated by `audit-wallet`): Added `wallet check-swap` (JSON: `new_coldkey_hash`, not `new_coldkey`) and `wallet associate-hotkey`.
- **Missing identity subcommand** (motivated by `audit-identity`): Added `identity clear`.
- **Missing serve subcommands** (motivated by `audit-serve`): Added `serve reset`, `serve batch-axon`, `serve prometheus`, `serve axon-tls` with cert-size caveat (≤65 bytes).
- **Expanded proxy list** (motivated by `audit-proxy`): All 11 subcommands listed, not 3.
- **Expanded multisig list** (motivated by `audit-multisig`): All 6 subcommands; added `NoTimepoint` caveat on `approve`.
- **Swap subcommands** (motivated by `audit-swap-keys`): Added `swap evm-key`; noted `swap coldkey` calls deprecated extrinsic.
- **Full crowdloan list** (motivated by `audit-crowdloan`): All 12 subcommands listed, not 3.
- **Config subcommands** (motivated by `audit-config`): Added `cache-clear`, `cache-info`; noted `config show` ignores `--output json`.
- **New command groups** (motivated by respective audits): Added `liquidity`, `scheduler`, `preimage`, `contracts`, `evm`, `safe-mode`, `drand`.
- **Admin section** (motivated by `audit-admin`): Expanded from 8 to 30 named subcommands; added complete list of AdminUtils calls reachable only via `admin raw`.
- **Balance notes** (motivated by `audit-balance-transfer`): Free-only caveat, `--threshold` without `--watch` silently ignored, `--at-block` + `--watch` silently drops `--watch`.
- **Stake encoding caveat** (motivated by `audit-stake`): Documented TAO-scale encoding for `remove-limit`, `recycle-alpha`, `burn-alpha`.
- **Completions** (motivated by `audit-completions-update`): Added `powershell` to the shell list.
- **Commands ignoring `--output json`** (motivated by multiple audits): Added summary note listing affected commands.

**`docs/hyperparameters.md`** (major expansion, 406 → 534 lines):

- Restructured into per-subnet and global tables, each with call index column.
- Added **35 previously undocumented dispatchables**: `default_take`, `tx_rate_limit`, `tx_delegate_take_rate_limit`, `min_delegate_take`, `subnet_owner_cut`, `network_rate_limit`, `total_issuance`, `network_immunity_period`, `network_min_lock_cost`, `subnet_limit`, `lock_reduction_interval`, `rao_recycled`, `stake_threshold`, `nominator_min_required_stake`, `alpha_values`, `alpha_sigmoid_steepness`, `ema_price_halving_period`, `dissolve_network_schedule_duration`, `evm_chain_id`, `toggle_transfer`, `recycle_or_burn`, `subnet_moving_alpha`, `subnet_owner_hotkey`, `subtoken_enabled`, `commit_reveal_version`, `owner_immune_neuron_limit`, `ck_burn`, `admin_freeze_window`, `owner_hparam_rate_limit`, `mechanism_count`, `mechanism_emission_split`, `max_mechanism_count`, `burn_half_life`, `burn_increase_mult`, `start_call_delay`, `coldkey_swap_announcement_delay`, `coldkey_swap_reannouncement_delay`, `tao_flow_cutoff`, `tao_flow_normalization_exponent`, `tao_flow_smoothing_factor`.
- Added encoding-drift notes for `subnet_moving_alpha` (u64 in CLI, I96F32 in pallet) and `mechanism_emission_split` (Vec\<u64\> vs Vec\<u16\>).
- Added owner rate-limiting interaction section, TAO flow parameters section, coldkey swap timing section.
- Cross-linked to `docs/commands/admin.md`, `docs/commands/subnet.md`, and AdminUtils source.

## Measurements
- `LOC(docs/llm.txt)`: 380 → 491
- `LOC(docs/hyperparameters.md)`: 406 → 534
- `cargo check --all-targets`: 0 errors → 0 errors
- `cargo build --bin agcli`: success → success
- `sudo_set_*` dispatchables documented in hyperparameters.md: ~33 → 73 (all dispatchables in admin-utils/src/lib.rs)

## Verification
type-check-only

## Notes, concerns, deviations, findings, thoughts, feedback

- No source files were modified; all changes are documentation only.
- The `admin raw` call allowlist (`known_params()` in `src/admin.rs`) does not include all of the dispatchables I added to the docs (e.g., `sudo_set_tao_flow_cutoff`, `sudo_set_burn_half_life`, etc.). Agents attempting those via `admin raw` will get a "call not in allowlist" error. The proper fix is to expand `known_params()` in source, but that is out of scope for this doc-only task.
- `sudo_set_subnet_moving_alpha` encoding drift (u64 CLI → I96F32 pallet) was flagged in the `audit-admin` handoff; the docs now note to use `admin raw` for correct encoding.
- `sudo_set_mechanism_emission_split` encoding drift (Vec\<u64\> without `Option::Some` wrapper) is noted in hyperparameters.md.
- `sudo_set_max_weight_limit` is absent from admin-utils/src/lib.rs at the current commit (finding from `audit-admin`); the CLI command `admin set-max-weight-limit` still parses and is in llm.txt, but will fail at runtime due to missing dispatchable.

## Suggested follow-ups
- Expand `known_params()` in `src/admin.rs` to cover all admin-utils dispatchables (especially the TAO flow, burn half-life, coldkey swap delay parameters).
- Fix `admin set-subnet-moving-alpha` to encode I96F32 correctly rather than u64.
- Fix `admin set-mechanism-emission-split` to encode `Option::Some(Vec<u16>)` as the pallet expects.
- Remove or stub `admin set-max-weight-limit` since the pallet dispatchable does not exist at the current commit.
- Correct `docs/llm.txt` entry for `agcli --live view dynamic` to `agcli view dynamic --live` in any place the docs were embedded in llm.txt (done), and propagate to README/tutorial files.