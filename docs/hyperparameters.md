# Subtensor Hyperparameters — Complete Reference

Every subnet on Bittensor has a set of tunable hyperparameters stored on-chain. Some can be changed by the **subnet owner** via `agcli subnet set-param`; others require the **chain sudo key** (root/governance) via `agcli admin`. This guide explains every parameter, what it actually does, and why you'd change it.

Cross-links: [admin.md](commands/admin.md) · [subnet.md](commands/subnet.md) · [AdminUtils pallet](../subtensor/pallets/admin-utils/src/lib.rs)

---

## Quick Reference Table

### Per-Subnet Parameters

| Parameter | Type | Who Sets | agcli Command | Call Index |
|-----------|------|----------|---------------|------------|
| [tempo](#tempo) | u16 | sudo | `admin set-tempo` | 30 |
| [rho](#rho-ρ) | u16 | sudo | `admin set-rho` | 23 |
| [kappa](#kappa-κ) | u16 | sudo | `admin set-kappa` | 22 |
| [immunity_period](#immunity_period) | u16 | owner/sudo | `admin set-immunity-period` | 10 |
| [min_allowed_weights](#min_allowed_weights) | u16 | owner/sudo | `admin set-min-weights` | 11 |
| [max_allowed_uids](#max_allowed_uids) | u16 | sudo | `admin set-max-uids` | 12 |
| [max_allowed_validators](#max_allowed_validators) | u16 | sudo | `admin set-max-validators` | 27 |
| [max_weight_limit](#max_weight_limit) | u16 | owner/sudo | `admin set-max-weight-limit` | — |
| [min_difficulty](#min_difficulty) | u64 | sudo | `admin set-min-difficulty` | 4 |
| [max_difficulty](#max_difficulty) | u64 | owner/sudo | `admin set-max-difficulty` | 5 |
| [difficulty](#difficulty) | u64 | sudo | `admin set-difficulty` | 26 |
| [weights_version_key](#weights_version) | u64 | owner/sudo | `admin raw --call sudo_set_weights_version_key` | 6 |
| [weights_rate_limit](#weights_rate_limit) | u64 | sudo | `admin set-weights-rate-limit` | 7 |
| [adjustment_interval](#adjustment_interval) | u16 | sudo | `admin set-adjustment-interval` | 8 |
| [adjustment_alpha](#adjustment_alpha) | u64 | owner/sudo | `admin set-adjustment-alpha` | 9 |
| [activity_cutoff](#activity_cutoff) | u16 | owner/sudo | `admin set-activity-cutoff` | 19 |
| [registration_allowed](#registration_allowed) | bool | sudo | `admin set-network-registration` | 20 |
| [pow_registration_allowed](#pow_registration_allowed) | bool | sudo | `admin set-pow-registration` | 21 |
| [target_regs_per_interval](#target_regs_per_interval) | u16 | sudo | `admin raw --call sudo_set_target_registrations_per_interval` | 18 |
| [min_burn](#min_burn) | u64 | sudo | `admin set-min-burn` | 24 |
| [max_burn](#max_burn) | u64 | owner/sudo | `admin set-max-burn` | 25 |
| [bonds_moving_average](#bonds_moving_average) | u64 | owner/sudo | `admin raw --call sudo_set_bonds_moving_average` | 28 |
| [bonds_penalty](#bonds_penalty) | u16 | owner/sudo | `admin set-bonds-penalty` | 29 |
| [max_regs_per_block](#max_regs_per_block) | u16 | sudo | `admin raw --call sudo_set_max_registrations_per_block` | — |
| [serving_rate_limit](#serving_rate_limit) | u64 | owner/sudo | `admin raw --call sudo_set_serving_rate_limit` | 3 |
| [commit_reveal_weights_enabled](#commit_reveal_weights_enabled) | bool | owner/sudo | `admin set-commit-reveal` | 49 |
| [commit_reveal_weights_interval](#commit_reveal_weights_interval) | u64 | owner/sudo | `admin raw --call sudo_set_commit_reveal_weights_interval` | 57 |
| [liquid_alpha_enabled](#liquid_alpha_enabled) | bool | owner/sudo | `admin set-liquid-alpha` | 50 |
| [alpha_values](#alpha_values) | u16×2 | owner/sudo | `admin set-alpha-values` | 51 |
| [alpha_sigmoid_steepness](#alpha_sigmoid_steepness) | i16 | owner/sudo | `admin raw --call sudo_set_alpha_sigmoid_steepness` | 68 |
| [ema_price_halving_period](#ema_price_halving_period) | u64 | sudo | `admin raw --call sudo_set_ema_price_halving_period` | 65 |
| [yuma3_enabled](#yuma3_enabled) | bool | owner/sudo | `admin set-yuma3` | 69 |
| [bonds_reset_enabled](#bonds_reset_enabled) | bool | owner/sudo | `admin raw --call sudo_set_bonds_reset_enabled` | 70 |
| [toggle_transfer](#toggle_transfer) | bool | owner/sudo | `admin raw --call sudo_set_toggle_transfer` | 61 |
| [recycle_or_burn](#recycle_or_burn) | enum | owner/sudo | `admin raw --call sudo_set_recycle_or_burn` | 80 |
| [owner_immune_neuron_limit](#owner_immune_neuron_limit) | u16 | owner/sudo | `admin raw --call sudo_set_owner_immune_neuron_limit` | 72 |
| [mechanism_count](#mechanism_count) | u16 | owner/sudo | `admin set-mechanism-count` | 76 |
| [mechanism_emission_split](#mechanism_emission_split) | Vec\<u16\> | owner/sudo | `admin set-mechanism-emission-split` | 77 |
| [min_allowed_uids](#min_allowed_uids) | u16 | sudo | `admin raw --call sudo_set_min_allowed_uids` | 79 |
| [min_non_immune_uids](#min_non_immune_uids) | u16 | sudo | `admin raw --call sudo_set_min_non_immune_uids` | 84 |
| [rao_recycled](#rao_recycled) | u64 | sudo | `admin raw --call sudo_set_rao_recycled` | 39 |
| [subnet_owner_hotkey](#subnet_owner_hotkey) | AccountId | owner/sudo | `admin raw --call sudo_set_sn_owner_hotkey` | 67 |
| [burn_half_life](#burn_half_life) | u16 | owner/sudo | `admin raw --call sudo_set_burn_half_life` | 89 |
| [burn_increase_mult](#burn_increase_mult) | U64F64 | owner/sudo | `admin raw --call sudo_set_burn_increase_mult` | 90 |
| [subtoken_enabled](#subtoken_enabled) | bool | sudo | `admin raw --call sudo_set_subtoken_enabled` | 66 |

### Global Parameters (no netuid)

| Parameter | Type | Who Sets | agcli Command | Call Index |
|-----------|------|----------|---------------|------------|
| [default_take](#default_take) | u16 | sudo | `admin set-default-take` | 1 |
| [tx_rate_limit](#tx_rate_limit) | u64 | sudo | `admin set-tx-rate-limit` | 2 |
| [subnet_owner_cut](#subnet_owner_cut) | u16 | sudo | `admin raw --call sudo_set_subnet_owner_cut` | 28 |
| [network_rate_limit](#network_rate_limit) | u64 | sudo | `admin raw --call sudo_set_network_rate_limit` | 29 |
| [total_issuance](#total_issuance) | u64 | sudo | `admin raw --call sudo_set_total_issuance` | 33 |
| [network_immunity_period](#network_immunity_period) | u64 | sudo | `admin raw --call sudo_set_network_immunity_period` | 35 |
| [network_min_lock_cost](#network_min_lock_cost) | u64 | sudo | `admin raw --call sudo_set_network_min_lock_cost` | 36 |
| [subnet_limit](#subnet_limit) | u16 | sudo | `admin raw --call sudo_set_subnet_limit` | 37 |
| [lock_reduction_interval](#lock_reduction_interval) | u64 | sudo | `admin raw --call sudo_set_lock_reduction_interval` | 38 |
| [stake_threshold](#stake_threshold) | u64 | sudo | `admin set-stake-threshold` | 42 |
| [nominator_min_required_stake](#nominator_min_required_stake) | u64 | sudo | `admin set-nominator-min-stake` | 43 |
| [tx_delegate_take_rate_limit](#tx_delegate_take_rate_limit) | u64 | sudo | `admin raw --call sudo_set_tx_delegate_take_rate_limit` | 45 |
| [min_delegate_take](#min_delegate_take) | u16 | sudo | `admin raw --call sudo_set_min_delegate_take` | 46 |
| [subnet_moving_alpha](#subnet_moving_alpha) | I96F32 | sudo | `admin set-subnet-moving-alpha` (or `admin raw`) | 63 |
| [dissolve_network_schedule_duration](#dissolve_network_schedule_duration) | BlockNumber | sudo | `admin raw --call sudo_set_dissolve_network_schedule_duration` | 55 |
| [evm_chain_id](#evm_chain_id) | u64 | sudo | `admin raw --call sudo_set_evm_chain_id` | 58 |
| [commit_reveal_version](#commit_reveal_version) | u16 | sudo | `admin raw --call sudo_set_commit_reveal_version` | 71 |
| [ck_burn](#ck_burn) | u64 | sudo | `admin raw --call sudo_set_ck_burn` | 73 |
| [admin_freeze_window](#admin_freeze_window) | u16 | sudo | `admin raw --call sudo_set_admin_freeze_window` | 74 |
| [owner_hparam_rate_limit](#owner_hparam_rate_limit) | u16 | sudo | `admin raw --call sudo_set_owner_hparam_rate_limit` | 75 |
| [max_mechanism_count](#max_mechanism_count) | u16 | sudo | `admin raw --call sudo_set_max_mechanism_count` | 88 |
| [tao_flow_cutoff](#tao_flow_cutoff) | I64F64 | sudo | `admin raw --call sudo_set_tao_flow_cutoff` | 81 |
| [tao_flow_normalization_exponent](#tao_flow_normalization_exponent) | U64F64 | sudo | `admin raw --call sudo_set_tao_flow_normalization_exponent` | 82 |
| [tao_flow_smoothing_factor](#tao_flow_smoothing_factor) | u64 | sudo | `admin raw --call sudo_set_tao_flow_smoothing_factor` | 83 |
| [start_call_delay](#start_call_delay) | u64 | sudo | `admin raw --call sudo_set_start_call_delay` | 85 |
| [coldkey_swap_announcement_delay](#coldkey_swap_announcement_delay) | BlockNumber | sudo | `admin raw --call sudo_set_coldkey_swap_announcement_delay` | 86 |
| [coldkey_swap_reannouncement_delay](#coldkey_swap_reannouncement_delay) | BlockNumber | sudo | `admin raw --call sudo_set_coldkey_swap_reannouncement_delay` | 87 |

---

## Epoch & Timing

### tempo
**Blocks per epoch.** An epoch is one complete evaluation cycle: validators set weights, consensus runs, emissions are distributed.

- **Default**: 360 blocks (~72 minutes at 12s/block)
- **Range**: 1–65535
- **Storage**: `SubtensorModule.Tempo[netuid]`
- **Effect**: Lower tempo = faster evaluation cycles = more frequent emission distribution. Higher tempo = less chain overhead but slower responsiveness.
- **Why change it**: Short tempo for fast-iterating subnets (e.g., real-time inference). Long tempo for subnets where evaluation is expensive (e.g., training runs).
- **Gotcha**: Tempo acts as a rate-limit multiplier — subnet owners can only change hyperparams every `tempo × OwnerHyperparamRateLimit` blocks.
- **agcli**: `agcli admin set-tempo --netuid N --tempo T --sudo-key //Alice`

### activity_cutoff
**Blocks of inactivity before a neuron becomes deregistration-eligible.**

- **Default**: 5000 blocks (~16.6 hours)
- **Range**: 1–65535
- **Storage**: `SubtensorModule.ActivityCutoff[netuid]`
- **Effect**: If a validator hasn't set weights (or a miner hasn't been scored) within this window, the neuron is flagged inactive and can be replaced by new registrants.
- **Why change it**: Increase for subnets with long evaluation cycles. Decrease to aggressively prune idle neurons.
- **agcli**: `agcli admin set-activity-cutoff --netuid N --cutoff C --sudo-key //Alice`

### immunity_period
**Blocks of protection after a neuron registers.**

- **Default**: 4096 blocks (~13.6 hours)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.ImmunityPeriod[netuid]`
- **Effect**: Newly registered neurons can't be deregistered during this window regardless of performance.
- **Why change it**: Increase if your subnet needs significant setup time. Decrease if you want a competitive "prove yourself fast" environment.
- **Gotcha**: If `immunity_period > activity_cutoff`, neurons could be immune but flagged inactive simultaneously — immunity takes precedence.
- **agcli**: `agcli admin set-immunity-period --netuid N --period P --sudo-key //Alice`

---

## Consensus Parameters

### rho (ρ)
**Inflation/emission adjustment parameter.**

- **Default**: 10
- **Range**: 0–65535
- **Storage**: `SubtensorModule.Rho[netuid]`
- **Effect**: Used in the Yuma consensus emission formula. Controls how aggressively consensus rewards shift toward high-performing validators.
- **Why change it**: Rarely changed. Higher values make consensus more aggressive. Deep protocol parameter — change with extreme caution.
- **agcli**: `agcli admin set-rho --netuid N --rho R --sudo-key //Alice`

### kappa (κ)
**Consensus majority threshold.**

- **Default**: 32767 (≈50% of u16 max)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.Kappa[netuid]`
- **Effect**: Determines how much agreement validators need before a weight vector "wins" in Yuma consensus. 65535 = 100%, 32767 ≈ 50%.
- **Why change it**: Increase to require stronger validator agreement. Decrease to make consensus more permissive.
- **agcli**: `agcli admin set-kappa --netuid N --kappa K --sudo-key //Alice`

### yuma3_enabled
**Enable Yuma3 consensus variant for a subnet.**

- **Default**: false
- **Storage**: `SubtensorModule.Yuma3Enabled[netuid]`
- **Effect**: Enables the Yuma3 consensus algorithm variant (enhanced version of Yuma with improved properties).
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-yuma3 --netuid N --enabled true --sudo-key //Alice`

---

## Weight Parameters

### min_allowed_weights
**Minimum number of UIDs a validator must include when setting weights.**

- **Default**: varies (often 1–16)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.MinAllowedWeights[netuid]`
- **Effect**: Forces validators to evaluate at least N miners per weight-set call. Prevents validators from only rating one or two miners.
- **Why change it**: Increase to force broader evaluation. Set to 1 for small subnets.
- **Gotcha**: If set higher than active miners, validators can't set weights at all.
- **agcli**: `agcli admin set-min-weights --netuid N --min M --sudo-key //Alice`

### max_weight_limit
**Maximum weight value per UID in the weight vector.**

- **Default**: 65535 (no effective limit)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.MaxWeightsLimit[netuid]`
- **Effect**: Caps the normalized weight any single miner can receive. Lower values force more even distribution.
- **Why change it**: Decrease to prevent validators from concentrating all weight on a single miner.
- **agcli**: `agcli admin set-max-weight-limit --netuid N --limit L --sudo-key //Alice`

### weights_version_key
**Expected weights version key.**

- **Default**: 0
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.WeightsVersionKey[netuid]`
- **Effect**: When set, validators must pass this version number when calling `set_weights`. Mismatched versions are rejected.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_weights_version_key --args '[N, K]' --sudo-key //Alice`

### weights_rate_limit
**Minimum blocks between weight-set calls.**

- **Default**: varies (e.g., 100)
- **Range**: 0–u64::MAX (0 = unlimited)
- **Storage**: `SubtensorModule.WeightsSetRateLimit[netuid]`
- **Effect**: Rate-limits how often a validator can update weights.
- **Why change it**: Increase for subnets where evaluation is slow. Set to 0 for testing or fast-feedback subnets.
- **agcli**: `agcli admin set-weights-rate-limit --netuid N --limit L --sudo-key //Alice`

---

## Commit-Reveal

Commit-reveal is a two-phase weight submission protocol that prevents validators from copying each other's weights.

### commit_reveal_weights_enabled
**Toggle commit-reveal for weight submission.**

- **Default**: varies
- **Storage**: `SubtensorModule.CommitRevealWeightsEnabled[netuid]`
- **Effect**: When true, validators must use the two-phase commit-reveal protocol. `set_weights` is rejected.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-commit-reveal --netuid N --enabled true --sudo-key //Alice`

### commit_reveal_weights_interval
**Epochs between commit and reveal phases.**

- **Default**: varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.CommitRevealWeightsInterval[netuid]` / `RevealPeriodEpochs[netuid]`
- **Effect**: After committing, validators wait this many epochs before revealing.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_commit_reveal_weights_interval --args '[N, I]' --sudo-key //Alice`

### commit_reveal_version
**Global commit-reveal protocol version (all subnets).**

- **Default**: 0
- **Range**: 0–65535 (u16)
- **Storage**: `SubtensorModule.CommitRevealWeightsVersion`
- **Effect**: Must match between commit and reveal calls across all subnets globally. Used to upgrade the commit-reveal algorithm.
- **Who can set**: Sudo only (global)
- **agcli**: `agcli admin raw --call sudo_set_commit_reveal_version --args '[V]' --sudo-key //Alice`

---

## Registration & Difficulty

### registration_allowed
**Master switch for new registrations (burn and PoW).**

- **Default**: true
- **Storage**: `SubtensorModule.NetworkRegistrationAllowed[netuid]`
- **Effect**: When false, no new neurons can register on this subnet.
- **agcli**: `agcli admin set-network-registration --netuid N --allowed false --sudo-key //Alice`

### pow_registration_allowed
**Allow proof-of-work registration specifically.**

- **Default**: varies
- **Storage**: `SubtensorModule.NetworkPowRegistrationAllowed[netuid]`
- **Effect**: When false, only burn (TAO) registration works. PoW is disabled.
- **Gotcha**: Pallet currently hard-disables PoW (`POWRegistrationDisabled`). This call will always fail on current runtime.
- **agcli**: `agcli admin set-pow-registration --netuid N --allowed true --sudo-key //Alice`

### difficulty
**Current proof-of-work difficulty.**

- **Default**: dynamically adjusted
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.Difficulty[netuid]`
- **Effect**: The hash difficulty target for PoW registration.
- **agcli**: `agcli admin set-difficulty --netuid N --difficulty D --sudo-key //Alice`

### min_difficulty / max_difficulty
**Floor and ceiling for the auto-adjusted PoW difficulty.**

- **Defaults**: min ~10^18, max varies
- **Storage**: `SubtensorModule.MinDifficulty[netuid]` / `SubtensorModule.MaxDifficulty[netuid]`
- **agcli**: `agcli admin set-min-difficulty --netuid N --difficulty D --sudo-key //Alice`
- **Who sets max_difficulty**: Subnet owner or sudo

### target_regs_per_interval
**Target number of registrations per adjustment interval.**

- **Default**: 1–3 (varies)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.TargetRegistrationsPerInterval[netuid]`
- **Effect**: The difficulty auto-adjuster tries to hit this target.
- **agcli**: `agcli admin raw --call sudo_set_target_registrations_per_interval --args '[N, T]' --sudo-key //Alice`

### adjustment_interval
**Blocks between difficulty/burn auto-adjustments.**

- **Default**: varies (e.g., 112)
- **Range**: 1–65535
- **Storage**: `SubtensorModule.AdjustmentInterval[netuid]`
- **agcli**: `agcli admin set-adjustment-interval --netuid N --interval I --sudo-key //Alice`

### adjustment_alpha
**EMA smoothing factor for difficulty adjustment.**

- **Default**: varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.AdjustmentAlpha[netuid]`
- **Effect**: Controls how aggressively difficulty changes. High alpha = slow, smooth adjustments.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-adjustment-alpha --netuid N --alpha A --sudo-key //Alice`

### min_burn / max_burn
**Floor and ceiling for TAO burn registration cost (in RAO).**

- **Defaults**: min ~1 TAO (10^9 RAO), max varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.MinBurn[netuid]` / `SubtensorModule.MaxBurn[netuid]`
- **Gotcha**: Values are in RAO (1 TAO = 10^9 RAO).
- **Who sets max_burn**: Subnet owner or sudo
- **agcli**: `agcli admin set-min-burn --netuid N --burn B --sudo-key //Alice`

### max_regs_per_block
**Maximum registrations allowed per block.**

- **Default**: 1
- **Range**: 0–65535
- **Storage**: `SubtensorModule.MaxRegistrationsPerBlock[netuid]`
- **agcli**: `agcli admin raw --call sudo_set_max_registrations_per_block --args '[N, M]' --sudo-key //Alice`

### burn_half_life
**Number of epochs for burn registration cost to halve.**

- **Default**: varies
- **Range**: 1–MaxBurnHalfLife
- **Storage**: `SubtensorModule.BurnHalfLife[netuid]`
- **Effect**: Controls how quickly registration burn cost decays toward `min_burn` when registration pressure is low. Shorter half-life = faster cost decay.
- **Who can set**: Subnet owner or sudo; not permitted on root subnet
- **agcli**: `agcli admin raw --call sudo_set_burn_half_life --args '[N, H]' --sudo-key //Alice`

### burn_increase_mult
**Multiplier for registration burn cost increase (1.0–3.0).**

- **Default**: varies
- **Range**: 1.0–3.0 (U64F64 fixed-point)
- **Storage**: `SubtensorModule.BurnIncreaseMult[netuid]`
- **Effect**: When registration pressure exceeds target, burn cost is multiplied by this factor per interval. Higher value = more aggressive cost increases.
- **Who can set**: Subnet owner or sudo; not permitted on root subnet
- **agcli**: `agcli admin raw --call sudo_set_burn_increase_mult --args '[N, M]' --sudo-key //Alice`

---

## Network Size Limits

### max_allowed_uids
**Maximum total neurons (UIDs) allowed on the subnet.**

- **Default**: 256 (varies by subnet, root subnet is 64)
- **Range**: 1–65535
- **Storage**: `SubtensorModule.MaxAllowedUids[netuid]`
- **Effect**: Hard cap on subnet size. When full, new registrations must replace existing neurons.
- **agcli**: `agcli admin set-max-uids --netuid N --max M --sudo-key //Alice`

### min_allowed_uids
**Minimum neurons the subnet must maintain.**

- **Default**: varies
- **Range**: 0–65535 (must be < max_allowed_uids and < current neuron count)
- **Storage**: `SubtensorModule.MinAllowedUids[netuid]`
- **Effect**: Prevents shrinking the subnet below this threshold via deregistration.
- **agcli**: `agcli admin raw --call sudo_set_min_allowed_uids --args '[N, M]' --sudo-key //Alice`

### max_allowed_validators
**Maximum validators with validator permits.**

- **Default**: 128 (subnet 1), varies
- **Range**: 1–65535
- **Storage**: `SubtensorModule.MaxAllowedValidators[netuid]`
- **Effect**: Limits how many neurons can act as validators.
- **agcli**: `agcli admin set-max-validators --netuid N --max M --sudo-key //Alice`

### min_non_immune_uids
**Minimum number of neurons that are NOT immune.**

- **Default**: varies
- **Range**: 0–65535
- **Storage**: `SubtensorModule.MinNonImmuneUids[netuid]`
- **Effect**: Ensures there are always N neurons eligible for deregistration.
- **agcli**: `agcli admin raw --call sudo_set_min_non_immune_uids --args '[N, M]' --sudo-key //Alice`

### owner_immune_neuron_limit
**Maximum neurons the subnet owner can protect from deregistration.**

- **Default**: varies
- **Range**: 0–65535
- **Storage**: `SubtensorModule.OwnerImmuneNeuronLimit[netuid]`
- **Effect**: Caps how many owner-registered neurons get permanent immunity. Prevents the owner from blocking all competitive churn.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_owner_immune_neuron_limit --args '[N, L]' --sudo-key //Alice`

---

## Bonds & Dividends

### bonds_moving_average
**Smoothing factor for bond calculations.**

- **Default**: 900000
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.BondsMovingAverage[netuid]`
- **Effect**: Higher values = bonds change slowly (more historical weight). Lower values = bonds respond quickly to new weight vectors.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_bonds_moving_average --args '[N, 900000]' --sudo-key //Alice`

### bonds_penalty
**Penalty factor applied to bond calculations.**

- **Default**: 0
- **Range**: 0–65535
- **Storage**: `SubtensorModule.BondsPenalty[netuid]`
- **Effect**: Penalizes validators whose weight vectors deviate too strongly from consensus.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-bonds-penalty --netuid N --penalty P --sudo-key //Alice`

### bonds_reset_enabled
**Allow periodic bond resets.**

- **Default**: false
- **Storage**: `SubtensorModule.BondsResetEnabled[netuid]`
- **Effect**: When true, bonds can be reset (zeroed) periodically, preventing entrenched validators from permanently dominating dividends.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_bonds_reset_enabled --args '[N, true]' --sudo-key //Alice`

### liquid_alpha_enabled
**Enable liquid alpha (dynamic dividend distribution).**

- **Default**: false
- **Storage**: `SubtensorModule.LiquidAlphaOn[netuid]`
- **Effect**: When enabled, the alpha parameter in bond calculations becomes dynamic instead of fixed.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-liquid-alpha --netuid N --enabled true --sudo-key //Alice`

### alpha_values
**Liquid alpha bounds (alpha_low, alpha_high) — both u16 in [0, 65535].**

- **Default**: alpha_low=0, alpha_high=65535
- **Storage**: `SubtensorModule.AlphaValues[netuid]` (stored as (u16, u16) tuple)
- **Effect**: When liquid alpha is enabled, the actual alpha value is clamped to [alpha_low/65535, alpha_high/65535]. Controls the range of adaptive bond updating.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-alpha-values --netuid N --alpha-low L --alpha-high H --sudo-key //Alice`

### alpha_sigmoid_steepness
**Steepness of the sigmoid function for alpha scaling.**

- **Default**: 0
- **Range**: 0–i16::MAX (non-negative from owner; root can set negative values)
- **Storage**: `SubtensorModule.AlphaSigmoidSteepness[netuid]`
- **Effect**: Controls how sharply the alpha sigmoid transitions. Higher steepness = more binary behavior (bonds either update fast or slow). Negative values reserved for future use.
- **Who can set**: Subnet owner (non-negative only) or sudo
- **agcli**: `agcli admin raw --call sudo_set_alpha_sigmoid_steepness --args '[N, S]' --sudo-key //Alice`

### ema_price_halving_period
**Number of blocks for the EMA alpha token price to halve.**

- **Default**: varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.EMAPriceHalvingBlocks[netuid]`
- **Effect**: Controls the exponential moving average decay of the alpha token price. Shorter period = price reacts faster to AMM activity.
- **Who can set**: Sudo only
- **agcli**: `agcli admin raw --call sudo_set_ema_price_halving_period --args '[N, P]' --sudo-key //Alice`

---

## Serving (Axon)

### serving_rate_limit
**Minimum blocks between `serve_axon` calls.**

- **Default**: 50
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.ServingRateLimit[netuid]`
- **Effect**: Rate-limits how often a neuron can update its on-chain IP/port/protocol info.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_serving_rate_limit --args '[N, L]' --sudo-key //Alice`

---

## Subnet Economy

### toggle_transfer
**Enable/disable alpha token transfers for a subnet.**

- **Default**: varies
- **Storage**: `SubtensorModule.TransferEnabled[netuid]`
- **Effect**: When false, alpha tokens on this subnet cannot be transferred between accounts.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_toggle_transfer --args '[N, true]' --sudo-key //Alice`

### recycle_or_burn
**Behavior of "burn UID" emissions.**

- **Default**: Burn
- **Type**: `RecycleOrBurnEnum { Burn, Recycle }`
- **Storage**: `SubtensorModule.RecycleOrBurn[netuid]`
- **Effect**: When set to `Recycle`, miner emissions sent to the burn UID slot are recycled back into the subnet pool instead of being burned.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin raw --call sudo_set_recycle_or_burn --args '[N, {"Recycle": null}]' --sudo-key //Alice`

### subnet_owner_hotkey / sn_owner_hotkey
**Hotkey associated with the subnet owner.**

- **Storage**: `SubtensorModule.SubnetOwnerHotkey[netuid]`
- **Effect**: Sets the hotkey that represents the subnet owner on-chain for staking/childkey operations. Rate-limited to one change per configured interval.
- **Who can set**: Subnet owner or sudo (call index 67: `sudo_set_sn_owner_hotkey`; call index 64: `sudo_set_subnet_owner_hotkey` is identical)
- **agcli**: `agcli admin raw --call sudo_set_sn_owner_hotkey --args '[N, "5SS58..."]' --sudo-key //Alice`

### subtoken_enabled
**Enable/disable subtoken trading for a subnet.**

- **Default**: varies
- **Storage**: `SubtensorModule.SubtokenEnabled[netuid]`
- **Effect**: When true, the subnet's alpha token participates in the AMM swap system.
- **Who can set**: Sudo only
- **agcli**: `agcli admin raw --call sudo_set_subtoken_enabled --args '[N, true]' --sudo-key //Alice`

### rao_recycled
**Amount of RAO recycled for emissions on this subnet.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.RAORecycledForRegistration[netuid]`
- **Effect**: Directly sets the recycled RAO counter used in emission calculations. Primarily used for genesis/migration.
- **Who can set**: Sudo only
- **agcli**: `agcli admin raw --call sudo_set_rao_recycled --args '[N, R]' --sudo-key //Alice`

### mechanism_count
**Number of active mechanisms in a subnet.**

- **Default**: 1
- **Range**: 0–max_mechanism_count
- **Storage**: `SubtensorModule.MechanismCountCurrent[netuid]`
- **Effect**: Controls how many emission mechanisms operate simultaneously in the subnet. Each mechanism can have separate weights, validators, miners.
- **Who can set**: Subnet owner or sudo
- **agcli**: `agcli admin set-mechanism-count --netuid N --count C --sudo-key //Alice`

### mechanism_emission_split
**Emission weight split across mechanisms.**

- **Default**: None (equal split)
- **Type**: `Option<Vec<u16>>` — proportional weights summing to 65535
- **Storage**: `SubtensorModule.MechanismEmissionSplit[netuid]`
- **Effect**: When set, emissions are proportionally distributed to mechanisms per these weights. `None` resets to equal split.
- **Who can set**: Subnet owner or sudo
- **Note (drift)**: CLI encodes as `Vec<u64>` (not `Vec<u16>`) without explicit `Option::Some` wrapper — may cause encoding issues
- **agcli**: `agcli admin set-mechanism-emission-split --netuid N --weights "50,50" --sudo-key //Alice`

### burn_half_life
See [Registration & Difficulty](#registration--difficulty).

### burn_increase_mult
See [Registration & Difficulty](#registration--difficulty).

---

## Global Network Parameters

### default_take
**Global default delegate take rate for new delegations.**

- **Default**: ~18% (11796 in u16 scale; 18/100 × 65535)
- **Range**: 0–65535 (u16 where 65535 = 100%)
- **Storage**: `SubtensorModule.MaxDelegateTake`
- **Effect**: Sets the take rate used when a delegate does not explicitly set their own take. Also used as the maximum cap for `increase_take`.
- **agcli**: `agcli admin set-default-take --take T --sudo-key //Alice`

### tx_rate_limit
**Global per-account transaction rate limit (blocks between identical calls).**

- **Default**: 1000
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.TxRateLimit`
- **Effect**: An account can only submit the same type of extrinsic once per this many blocks. Prevents spam.
- **agcli**: `agcli admin set-tx-rate-limit --limit L --sudo-key //Alice`

### tx_delegate_take_rate_limit
**Global rate limit for `increase_take` transactions.**

- **Default**: ~300 blocks
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.TxDelegateTakeRateLimit`
- **Effect**: An account can only call `increase_take` once per this many blocks. `decrease_take` is not rate-limited.
- **agcli**: `agcli admin raw --call sudo_set_tx_delegate_take_rate_limit --args '[L]' --sudo-key //Alice`

### min_delegate_take
**Global minimum floor for delegate take rates.**

- **Default**: 0
- **Range**: 0–65535 (u16)
- **Storage**: `SubtensorModule.MinDelegateTake`
- **Effect**: Delegates cannot set their take below this floor. If the chain admin raises it, existing delegates below the floor cannot lower their take further.
- **agcli**: `agcli admin raw --call sudo_set_min_delegate_take --args '[T]' --sudo-key //Alice`

### subnet_owner_cut
**Global protocol cut taken from subnet emissions before owner distribution.**

- **Default**: varies
- **Range**: 0–65535 (u16)
- **Storage**: `SubtensorModule.SubnetOwnerCut`
- **Effect**: Protocol fee on subnet emissions (as fraction of u16::MAX).
- **agcli**: `agcli admin raw --call sudo_set_subnet_owner_cut --args '[C]' --sudo-key //Alice`

### network_rate_limit
**Minimum blocks between subnet creation events (global).**

- **Default**: varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.NetworkRateLimit`
- **Effect**: Prevents creation of multiple subnets within a short time window.
- **agcli**: `agcli admin raw --call sudo_set_network_rate_limit --args '[R]' --sudo-key //Alice`

### total_issuance
**Override total TAO issuance (DANGEROUS — breaks economic invariants).**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.TotalIssuance`
- **Effect**: Directly sets the total TAO supply counter. Used only for genesis or emergency correction. Incorrect values will corrupt inflation accounting.
- **agcli**: `agcli admin raw --call sudo_set_total_issuance --args '[I]' --sudo-key //Alice`

### network_immunity_period
**Blocks of immunity for newly created subnets.**

- **Default**: varies
- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.NetworkImmunityPeriod`
- **Effect**: A newly created subnet cannot be dissolved during this window. Protects subnet owners who just registered.
- **agcli**: `agcli admin raw --call sudo_set_network_immunity_period --args '[P]' --sudo-key //Alice`

### network_min_lock_cost
**Minimum TAO required to lock when creating a subnet.**

- **Range**: 0–u64::MAX (RAO)
- **Storage**: `SubtensorModule.NetworkMinLockCost`
- **Effect**: Floor on the subnet creation lock. Prevents subnets from being created for free if the auto-adjustment would otherwise drop the cost to zero.
- **agcli**: `agcli admin raw --call sudo_set_network_min_lock_cost --args '[C]' --sudo-key //Alice`

### subnet_limit
**Maximum number of subnets that can exist simultaneously.**

- **Default**: varies (currently 64 on mainnet)
- **Range**: 0–65535
- **Storage**: `SubtensorModule.SubnetLimit` / `MaxSubnets`
- **agcli**: `agcli admin raw --call sudo_set_subnet_limit --args '[M]' --sudo-key //Alice`

### lock_reduction_interval
**Blocks over which the subnet creation lock cost decays.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.LockReductionInterval`
- **Effect**: The lock cost for creating a subnet is reduced by half every `lock_reduction_interval` blocks since the last subnet was created.
- **agcli**: `agcli admin raw --call sudo_set_lock_reduction_interval --args '[I]' --sudo-key //Alice`

### stake_threshold
**Minimum stake (RAO) required for a validator permit.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.StakeThreshold`
- **Effect**: Validators below this stake floor cannot receive permits regardless of their rank.
- **agcli**: `agcli admin set-stake-threshold --threshold T --sudo-key //Alice`

### nominator_min_required_stake
**Minimum stake (RAO) required to be a nominator.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.NominatorMinRequiredStake`
- **Effect**: Nominators below this threshold are automatically cleared. Raising this value triggers immediate cleanup of small nominations.
- **agcli**: `agcli admin set-nominator-min-stake --stake S --sudo-key //Alice`

### subnet_moving_alpha
**Global EMA smoothing factor for subnet emission share calculations.**

- **Type**: I96F32 fixed-point
- **Storage**: `SubtensorModule.SubnetMovingAlpha`
- **Effect**: Controls how quickly each subnet's share of global emissions adjusts to changes in staking. Lower alpha = faster adjustment; higher = more inertia.
- **Note (drift)**: CLI `admin set-subnet-moving-alpha` encodes as u64 but pallet expects I96F32. Use `admin raw` for correct encoding.
- **agcli**: `agcli admin raw --call sudo_set_subnet_moving_alpha --args '[ALPHA_I96F32]' --sudo-key //Alice`

### dissolve_network_schedule_duration
**Blocks a subnet dissolution is delayed after being scheduled.**

- **Range**: 0–u64::MAX (BlockNumber)
- **Storage**: `SubtensorModule.DissolveNetworkScheduleDuration`
- **Effect**: When a subnet owner calls `dissolve_network`, the actual removal is delayed by this many blocks, allowing time to dispute or cancel.
- **agcli**: `agcli admin raw --call sudo_set_dissolve_network_schedule_duration --args '[D]' --sudo-key //Alice`

### evm_chain_id
**EVM chain ID used by the EVM precompiles.**

- **Range**: 0–u64::MAX
- **Storage**: `ChainId` (admin-utils pallet)
- **Effect**: Sets the chain ID returned by EVM `CHAINID` opcode and used in EIP-155 transaction signing.
- **agcli**: `agcli admin raw --call sudo_set_evm_chain_id --args '[ID]' --sudo-key //Alice`

### ck_burn
**Childkey burn amount (RAO) required to create a childkey.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.CKBurn`
- **agcli**: `agcli admin raw --call sudo_set_ck_burn --args '[B]' --sudo-key //Alice`

### admin_freeze_window
**Blocks at the end of each epoch during which hyperparameter changes are blocked.**

- **Range**: 0–65535
- **Storage**: `SubtensorModule.AdminFreezeWindow`
- **Effect**: During the last N blocks of a tempo epoch, subnet owners cannot change hyperparameters. Prevents last-minute manipulation before epoch evaluation.
- **agcli**: `agcli admin raw --call sudo_set_admin_freeze_window --args '[W]' --sudo-key //Alice`

### owner_hparam_rate_limit
**Global multiplier (in epochs) for how often a subnet owner can change hyperparameters.**

- **Range**: 0–65535 (epochs)
- **Storage**: `SubtensorModule.OwnerHyperparamRateLimit`
- **Effect**: Subnet owners must wait `tempo × owner_hparam_rate_limit` blocks between hyperparameter changes. Higher value = less frequent owner changes.
- **agcli**: `agcli admin raw --call sudo_set_owner_hparam_rate_limit --args '[E]' --sudo-key //Alice`

### max_mechanism_count
**Global maximum number of mechanisms any subnet can have.**

- **Range**: 0–u16::MAX
- **Storage**: `SubtensorModule.MaxMechanismCount`
- **Effect**: Upper bound on `mechanism_count` for all subnets.
- **agcli**: `agcli admin raw --call sudo_set_max_mechanism_count --args '[M]' --sudo-key //Alice`

### commit_reveal_version
See [Commit-Reveal](#commit_reveal_version).

---

## TAO Flow Parameters (Dynamic Emission Routing)

These global parameters control how TAO emissions are distributed across subnets based on staking flows.

### tao_flow_cutoff
**Threshold (I64F64 fixed-point) below which a subnet's TAO flow is treated as zero.**

- **Storage**: `SubtensorModule.TaoFlowCutoff`
- **Effect**: Subnets with very small TAO inflows/outflows below this threshold are excluded from flow-based emission adjustments, reducing noise.
- **agcli**: `agcli admin raw --call sudo_set_tao_flow_cutoff --args '[A]' --sudo-key //Alice`

### tao_flow_normalization_exponent
**Exponent (U64F64, range 1.0–2.0) for normalizing TAO flows across subnets.**

- **Range**: 1.0–2.0
- **Storage**: `SubtensorModule.TaoFlowNormalizationExponent`
- **Effect**: Controls the power-law normalization applied to subnet TAO flows. Higher exponent concentrates emissions toward subnets with higher flows.
- **agcli**: `agcli admin raw --call sudo_set_tao_flow_normalization_exponent --args '[E]' --sudo-key //Alice`

### tao_flow_smoothing_factor
**EMA smoothing factor (u64) for TAO flow calculations.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.TaoFlowSmoothingFactor`
- **Effect**: Controls how quickly the moving average of subnet TAO flows adapts to new data.
- **agcli**: `agcli admin raw --call sudo_set_tao_flow_smoothing_factor --args '[F]' --sudo-key //Alice`

---

## Coldkey Swap Timing

### coldkey_swap_announcement_delay
**Blocks between announcing a coldkey swap and the swap becoming executable.**

- **Range**: 0–u64::MAX (BlockNumber)
- **Storage**: `SubtensorModule.ColdkeySwapAnnouncementDelay`
- **Effect**: Security delay that gives the community time to dispute fraudulent coldkey swaps. Longer delay = more security; shorter = faster recovery.
- **agcli**: `agcli admin raw --call sudo_set_coldkey_swap_announcement_delay --args '[D]' --sudo-key //Alice`

### coldkey_swap_reannouncement_delay
**Blocks required between consecutive coldkey swap reannouncements.**

- **Range**: 0–u64::MAX (BlockNumber)
- **Storage**: `SubtensorModule.ColdkeySwapReannouncementDelay`
- **Effect**: Rate-limits reannouncements to prevent spam during the announcement period.
- **agcli**: `agcli admin raw --call sudo_set_coldkey_swap_reannouncement_delay --args '[D]' --sudo-key //Alice`

---

## Subnet Lifecycle

### start_call_delay
**Blocks a subnet must wait after creation before calling `start`.**

- **Range**: 0–u64::MAX
- **Storage**: `SubtensorModule.StartCallDelay`
- **Effect**: Prevents owners from immediately starting emission by requiring a stabilization period after subnet creation.
- **agcli**: `agcli admin raw --call sudo_set_start_call_delay --args '[D]' --sudo-key //Alice`

---

## How Parameters Interact

### Registration cost loop
`target_regs_per_interval` + `adjustment_interval` + `adjustment_alpha` + `min_burn`/`max_burn` + `min_difficulty`/`max_difficulty` form a feedback loop:
1. Every `adjustment_interval` blocks, the chain checks how many neurons registered
2. If registrations > target → difficulty and burn cost go up
3. If registrations < target → difficulty and burn cost go down
4. `adjustment_alpha` controls the speed of change
5. `min_*`/`max_*` prevent extremes
6. `burn_half_life` and `burn_increase_mult` additionally modulate the burn cost curve

### Deregistration eligibility
A neuron can be deregistered if ALL of these hold:
1. They've been registered longer than `immunity_period`
2. They've been inactive for longer than `activity_cutoff`
3. The subnet is full (`max_allowed_uids` reached)
4. At least `min_non_immune_uids` neurons remain non-immune
5. A new registrant is competing for the slot

### Consensus → Emissions pipeline
1. Validators set weights (subject to `min_allowed_weights`, `max_weight_limit`, `weights_rate_limit`, `commit_reveal_*`)
2. Every `tempo` blocks, Yuma/Yuma3 consensus runs using `rho` and `kappa`
3. Bonds update using `bonds_moving_average`, `bonds_penalty`, and optionally `liquid_alpha_enabled` + `alpha_values`
4. Emissions split between validators (dividends via bonds) and miners (incentive via rank)
5. Subnet emission share weighted by `subnet_moving_alpha` and TAO flow parameters

### Owner rate limiting
Subnet owners changing hyperparameters are subject to:
- `owner_hparam_rate_limit` (global, in epochs): minimum epochs between any owner change
- `admin_freeze_window` (global, in blocks): no changes in last N blocks of each epoch
- Per-hyperparameter rate limits tracked in `LastRateLimitedBlock`

---

## Setting Parameters

### As subnet owner
```bash
# List all settable params
agcli subnet set-param --netuid 1 --param list

# Set a specific param
agcli subnet set-param --netuid 1 --param immunity_period --value 2000

# View current values
agcli subnet hyperparams --netuid 1
```

### As sudo (chain root)
```bash
# Typed commands (common params)
agcli admin set-tempo --netuid 1 --tempo 100 --sudo-key //Alice --network local

# Raw command (any AdminUtils call)
agcli admin raw --call sudo_set_bonds_moving_average --args '[1, 900000]' --sudo-key //Alice --network local

# List all known admin params
agcli admin list
```

### Via scaffold (automated setup)
```toml
# In scaffold.toml
[hyperparams]
tempo = 100
immunity_period = 500
max_allowed_validators = 8
min_allowed_weights = 1
weights_rate_limit = 0
commit_reveal_weights_enabled = false
```

---

## Source References

- **agcli admin commands**: `src/cli/admin_cmds.rs`, `src/admin.rs`
- **agcli subnet set-param**: `src/cli/subnet_cmds.rs` (SUBNET_PARAMS constant)
- **On-chain storage**: `SubtensorModule` pallet in subtensor `pallets/subtensor/src/`
- **AdminUtils pallet**: `subtensor/pallets/admin-utils/src/lib.rs` — all `sudo_set_*` extrinsics
- **Yuma consensus**: `subtensor/pallets/subtensor/src/epoch/` — the weight→emission pipeline
