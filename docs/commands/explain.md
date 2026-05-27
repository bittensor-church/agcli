# explain - concept reference and full-doc loader

`agcli explain` is an offline documentation command. It has no chain writes and does not build a `Client`.

## Clap surface

`Commands::Explain` is defined in `src/cli/mod.rs`:

- `--topic <String>`: optional concept key or alias.
- `--full`: optional bool flag. Switches from embedded summaries to `docs/commands/*.md` file loading.

Global flags still apply, especially:

- `--output {table|json|csv}` (`OutputFormat`; practical values here are table/json).
- `--pretty` (`bool`) only affects JSON formatting.

## Execution modes (all surfaces under `explain`)

`src/cli/commands.rs` dispatches `Commands::Explain` directly to `system_cmds::handle_explain(...)`.

| Mode | Example | Behavior |
|---|---|---|
| Topic index | `agcli explain` | Prints canonical topics from `utils::explain::list_topics()` |
| Topic summary | `agcli explain --topic tempo` | Prints built-in constant text from `utils::explain::explain(topic)` |
| Full-doc index | `agcli explain --full` | Lists `docs/commands/*.md` files discovered by `find_docs_dir()` |
| Full-doc topic | `agcli explain --topic weights --full` | Loads and prints `docs/commands/weights.md` (with alias fallbacks) |

## Subxt and SCALE audit result

`explain` does not call subxt, does not encode SCALE arguments, and does not submit extrinsics.

- No `connect(...)` call in the `Commands::Explain` match arm.
- `handle_explain(...)` only reads in-memory topic constants or markdown files on disk.
- Pallet/dispatchable/encoding checks are therefore not applicable to command execution itself.

## Exit codes

Command exits are classified through `src/error.rs` and `src/main.rs`.

| Case | Exit code |
|---|---|
| Success paths (`explain`, `--topic`, `--full`) | `0` |
| Unknown topic (`Unknown topic '...'`) | `12` (`VALIDATION`) |
| Missing docs directory in `--full` mode | `1` (`GENERIC`) |
| Invalid `--topic` path traversal form in `--full` mode | `1` (`GENERIC`) |
| Missing doc file for `--topic ... --full` | `1` (`GENERIC`) |
| Clap parse failure | `2` (clap default) |

## Output JSON schemas

### `agcli explain --output json`

```json
[
  {
    "topic": "tempo",
    "description": "Block cadence for subnet weight evaluation"
  }
]
```

### `agcli explain --topic <topic> --output json`

```json
{
  "topic": "tempo",
  "content": "TEMPO\n=====\n..."
}
```

### `agcli explain --topic <topic> --full --output json`

```json
{
  "topic": "weights",
  "source": "docs/commands/weights.md",
  "content": "# weights ..."
}
```

### `agcli explain --full --output json`

```json
["admin", "balance", "block", "doctor", "weights"]
```

### Unknown topic payload (`stderr`, JSON mode)

```json
{
  "error": true,
  "message": "Unknown topic 'xyz'",
  "available_topics": [
    {
      "topic": "tempo",
      "description": "Block cadence for subnet weight evaluation"
    }
  ]
}
```

## Topic catalog (canonical keys in `list_topics()`)

`list_topics()` currently returns 32 canonical topics.

| Canonical topic | Aliases accepted by `explain()` | Pallet + storage reference | On-chain events reference |
|---|---|---|---|
| `tempo` | `tempo` | `subtensor::Tempo` (`pallets/subtensor/src/lib.rs`) | `TempoSet` |
| `commit-reveal` | `commitreveal`, `cr` | `subtensor::CommitRevealWeightsEnabled`, `RevealPeriodEpochs`, `WeightCommits`, `CRV3WeightCommitsV2` | `CommitRevealEnabled`, `WeightsCommitted`, `WeightsRevealed`, `CRV3WeightsCommitted`, `CRV3WeightsRevealed` |
| `yuma` | `yuma`, `yumaconsensus` | `subtensor::Consensus`, `Incentive`, `Dividends`, `ValidatorTrust` | `WeightsSet`, `IncentiveAlphaEmittedToMiners` |
| `rate-limits` | `ratelimit`, `ratelimits`, `weightsratelimit` | `subtensor::WeightsSetRateLimit`, `TxRateLimit`, `TxDelegateTakeRateLimit`, `TxChildkeyTakeRateLimit` | `WeightsSetRateLimitSet`, `TxRateLimitSet`, `TxDelegateTakeRateLimitSet`, `TxChildKeyTakeRateLimitSet` |
| `weights` | `weights`, `settingweights`, `setweights`, `weightsetting` | `subtensor::Weights`, `WeightCommits`, `TimelockedWeightCommits` | `WeightsSet`, `WeightsCommitted`, `WeightsRevealed`, `TimelockedWeightsCommitted`, `TimelockedWeightsRevealed` |
| `stake-weight` | `stakeweight`, `stakeweightminimum`, `1000` | `subtensor::StakeThreshold`, `StakeWeight` | `StakeThresholdSet` |
| `amm` | `amm`, `dynamictao`, `dtao`, `pool` | `subtensor::SubnetTAO`, `SubnetAlphaIn`, `SubnetAlphaOut`, `SubnetMovingPrice`; `swap::Positions` | `StakeAdded`, `StakeRemoved`, `StakeSwapped`, `LiquidityAdded`, `LiquidityRemoved`, `LiquidityModified` |
| `bootstrap` | `bootstrap` | `subtensor::SubnetOwner`, `NetworkMinLockCost`, `NetworkRegistrationAllowed`, `Tempo` | `NetworkAdded`, `SubnetIdentitySet`, `RegistrationAllowed`, `TempoSet` |
| `alpha` | `alpha`, `alphatoken` | `subtensor::Alpha`, `AlphaV2`, `SubnetAlphaIn`, `SubnetAlphaOut` | `StakeAdded`, `StakeRemoved`, `AlphaRecycled`, `AlphaBurned` |
| `emission` | `emission`, `emissions` | `subtensor::BlockEmission`, `Emission`, `PendingValidatorEmission`, `PendingServerEmission` | `IncentiveAlphaEmittedToMiners`, `AutoStakeAdded` |
| `registration` | `registration`, `register` | `subtensor::Difficulty`, `Burn`, `UsedWork`, `Uids`, `RegistrationsThisBlock` | `NeuronRegistered`, `BulkNeuronsRegistered`, `PowRegistrationAllowed`, `DifficultySet` |
| `subnets` | `subnet`, `subnets` | `subtensor::TotalNetworks`, `SubnetLimit`, `SubnetOwner`, `NetworksAdded` | `NetworkAdded`, `NetworkRemoved`, `SubnetLimitSet` |
| `validators` | `validator`, `validators` | `subtensor::ValidatorPermit`, `ValidatorTrust`, `Delegates` | `WeightsSet`, `DelegateAdded`, `TakeIncreased`, `TakeDecreased` |
| `miners` | `miner`, `miners` | `subtensor::Uids`, `Axons`, `Incentive`, `Emission` | `NeuronRegistered`, `AxonServed`, `IncentiveAlphaEmittedToMiners` |
| `immunity` | `immunity`, `immunityperiod` | `subtensor::ImmunityPeriod`, `BlockAtRegistration`, `MinNonImmuneUids` | `ImmunityPeriodSet`, `MinNonImmuneUidsSet` |
| `delegation` | `delegate`, `delegation`, `nominate` | `subtensor::Delegates`, `TotalHotkeyShares`, `TotalHotkeyAlpha` | `DelegateAdded`, `TakeIncreased`, `TakeDecreased` |
| `childkeys` | `childkey`, `childkeys` | `subtensor::ChildKeys`, `ParentKeys`, `PendingChildKeys`, `ChildkeyTake` | `SetChildrenScheduled`, `SetChildren`, `ChildKeyTakeSet` |
| `root` | `root`, `rootnetwork` | `subtensor::RootProp`, `RootClaimable`, `RootClaimType` | `RootClaimed`, `RootClaimTypeSet` |
| `proxy` | `proxy` | `proxy::Proxies`, `proxy::Announcements`, `proxy::RealPaysFee` (`pallets/proxy/src/lib.rs`) | `ProxyAdded`, `ProxyRemoved`, `Announced`, `ProxyExecuted`, `RealPaysFeeSet` |
| `coldkey-swap` | `coldkeyswap`, `coldkey`, `ckswap` | `subtensor::ColdkeySwapAnnouncements`, `ColdkeySwapDisputes`, `ColdkeySwapAnnouncementDelay`, `ColdkeySwapReannouncementDelay` | `ColdkeySwapAnnounced`, `ColdkeySwapped`, `ColdkeySwapDisputed`, `ColdkeySwapCleared` |
| `governance` | `governance`, `gov`, `proposals` | `subtensor::VotingPower`, `VotingPowerTrackingEnabled`, `VotingPowerDisableAtBlock`, `VotingPowerEmaAlpha` | `VotingPowerTrackingEnabled`, `VotingPowerTrackingDisableScheduled`, `VotingPowerTrackingDisabled`, `VotingPowerEmaAlphaSet` |
| `senate` | `senate`, `triumvirate` | No dedicated custom pallet under `subtensor/pallets/*`; runtime comments mark older triumvirate/senate pallets as migrated/deprecated (`runtime/src/lib.rs`) | None in current custom pallet set |
| `mev-shield` | `mevshield`, `mev`, `mevprotection` | `shield::PendingExtrinsics`, `shield::NextPendingExtrinsicIndex`, `shield::MaxExtrinsicWeight` (`pallets/shield/src/lib.rs`) | `EncryptedSubmitted`, `ExtrinsicStored`, `ExtrinsicDispatched`, `ExtrinsicDispatchFailed` |
| `limits` | `limits`, `networklimits`, `chainlimits` | `subtensor::MaxAllowedUids`, `MinAllowedUids`, `MaxAllowedValidators`, `WeightsSetRateLimit`, `ServingRateLimit`, `MaxRegistrationsPerBlock` | `MaxAllowedUidsSet`, `MinAllowedUidsSet`, `MaxAllowedValidatorsSet`, `WeightsSetRateLimitSet`, `ServingRateLimitSet`, `MaxRegistrationsPerBlockSet` |
| `hyperparams` | `hyperparams`, `hyperparameters`, `params` | `subtensor::Tempo`, `Rho`, `Kappa`, `WeightsVersionKey`, `AdjustmentInterval`, `CommitRevealWeightsEnabled` | `TempoSet`, `RhoSet`, `KappaSet`, `WeightsVersionKeySet`, `AdjustmentIntervalSet`, `CommitRevealEnabled` |
| `axon` | `axon`, `axoninfo`, `serving` | `subtensor::Axons`, `Prometheus`, `ServingRateLimit` | `AxonServed`, `PrometheusServed`, `ServingRateLimitSet` |
| `take` | `take`, `delegatetake`, `validatortake` | `subtensor::Delegates`, `MaxDelegateTake`, `MinDelegateTake` | `TakeIncreased`, `TakeDecreased`, `MaxDelegateTakeSet`, `MinDelegateTakeSet` |
| `recycle` | `recycle`, `recyclealpha`, `burn`, `burnalpha` | `subtensor::RecycleOrBurn`, `SubnetAlphaOut` | `AlphaRecycled`, `AlphaBurned`, `AddStakeBurn` |
| `pow` | `pow`, `powregistration`, `proofofwork` | `subtensor::NetworkPowRegistrationAllowed`, `Difficulty`, `MinDifficulty`, `MaxDifficulty`, `UsedWork` | `PowRegistrationAllowed`, `DifficultySet`, `NeuronRegistered` |
| `archive` | `archive`, `archivenode`, `historical`, `wayback` | No dedicated runtime storage. This is a node data-retention mode used to read historical values of normal storage keys. | None. Read-only behavior. |
| `diff` | `diff`, `compare`, `historicaldiff` | No dedicated runtime storage. Compares snapshots from existing storage at two block hashes. | None. Read-only behavior. |
| `owner-workflow` | `ownerworkflow`, `ow`, `subnetowner`, `ownerguide` | Composite guide that spans `subtensor::SubnetOwner`, `SubnetIdentitiesV3`, `Tempo`, `CommitRevealWeightsEnabled`, `Weights` | Composite workflow, not one event. Common events include `NetworkAdded`, `SubnetIdentitySet`, `TempoSet`, `CommitRevealEnabled`, `WeightsSet` |

## Notes for audit consumers

- `explain` topic matching normalizes case and strips `-` and `_`.
- Unknown topic behavior is explicit and machine-readable in JSON mode.
- `--full` file loading includes path traversal checks and alias redirection for select topics (`cr`, `amm`, `nominate`, and weight-setting aliases).
- Fuzzy fallback exists for partial topic substrings in canonical keys.
