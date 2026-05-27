# crowdloan — Crowdloan Commands

Crowdloan commands map to the `Crowdloan` pallet in `subtensor/pallets/crowdloan/src/lib.rs`.

## Subcommands covered by `CrowdloanCommands`

- `create`
- `contribute`
- `withdraw`
- `finalize`
- `refund`
- `dissolve`
- `update-cap`
- `update-end`
- `update-min-contribution`
- `list`
- `info`
- `contributors`

## Global flags commonly used with this group

All subcommands also accept global flags from `agcli` (for example: `--network`, `--endpoint`, `--wallet`, `--wallet-dir`, `--output`, `--password`, `--yes`, `--timeout`, `--finalization-timeout`).

## Exit codes (from `src/error.rs`)

- `0`: success
- `1`: generic/uncategorized error
- `10`: network / RPC connectivity failure
- `11`: wallet/auth failure (unlock, key loading)
- `12`: validation failure (bad flags, invalid SS58, invalid amount relationships)
- `13`: chain/runtime dispatch failure (pallet error)
- `14`: file I/O failure
- `15`: timeout

For crowdloan write extrinsics, runtime rejections are classified as `13` (examples: `InvalidCrowdloanId`, `AlreadyFinalized`, `CapNotRaised`, `ContributionTooLow`, `InvalidOrigin`, `NotReadyToDissolve`).

---

## `agcli crowdloan create`

Create a campaign and lock creator deposit.

```bash
agcli crowdloan create \
  --deposit 10.0 \
  --min-contribution 1.0 \
  --cap 100.0 \
  --end-block 500000 \
  [--target 5F...]
```

**Clap flags**
- `--deposit <f64>` (TAO)
- `--min-contribution <f64>` (TAO)
- `--cap <f64>` (TAO)
- `--end-block <u32>`
- `--target <String>` (optional SS58)

**Pallet dispatchable**
- `Crowdloan::create(origin, deposit, min_contribution, cap, end, call, target_address)`
- agcli sets `call = None` and only exposes `target_address`.

**Storage keys touched**
- `Crowdloan::NextCrowdloanId`
- `Crowdloan::Crowdloans`
- `Crowdloan::Contributions`

**On-chain events**
- `Crowdloan::Created { crowdloan_id, creator, end, cap }`

**Output JSON schema**
- Current implementation prints plain text only (`"Crowdloan created. Tx: <hash>"`), even if `--output json` is requested.

---

## `agcli crowdloan contribute`

Contribute TAO to an active crowdloan.

```bash
agcli crowdloan contribute --crowdloan-id 3 --amount 5.5
```

**Clap flags**
- `--crowdloan-id <u32>`
- `--amount <f64>` (TAO)

**Pallet dispatchable**
- `Crowdloan::contribute(origin, crowdloan_id, amount)`

**Storage keys touched**
- `Crowdloan::Crowdloans`
- `Crowdloan::Contributions`

**On-chain events**
- `Crowdloan::Contributed { crowdloan_id, contributor, amount }`

**Output JSON schema**
- Current implementation prints plain text only (`"Contribution submitted. Tx: <hash>"`).

---

## `agcli crowdloan withdraw`

Withdraw caller contribution (subject to pallet rules).

```bash
agcli crowdloan withdraw --crowdloan-id 3
```

**Clap flags**
- `--crowdloan-id <u32>`

**Pallet dispatchable**
- `Crowdloan::withdraw(origin, crowdloan_id)`

**Storage keys touched**
- `Crowdloan::Crowdloans`
- `Crowdloan::Contributions`

**On-chain events**
- `Crowdloan::Withdrew { crowdloan_id, contributor, amount }`

**Output JSON schema**
- Current implementation prints plain text only (`"Withdrawal submitted. Tx: <hash>"`).

---

## `agcli crowdloan finalize`

Finalize a fully raised crowdloan (creator origin required by pallet).

```bash
agcli crowdloan finalize --crowdloan-id 3
```

**Clap flags**
- `--crowdloan-id <u32>`

**Pallet dispatchable**
- `Crowdloan::finalize(origin, crowdloan_id)`

**Storage keys touched**
- `Crowdloan::Crowdloans`
- `Crowdloan::CurrentCrowdloanId` (temporary, only if inner call exists)

**On-chain events**
- `Crowdloan::Finalized { crowdloan_id }`

**Output JSON schema**
- Current implementation prints plain text only (`"Crowdloan finalized. Tx: <hash>"`).

---

## `agcli crowdloan refund`

Refund contributors in batches (creator origin required by pallet).

```bash
agcli crowdloan refund --crowdloan-id 3
```

**Clap flags**
- `--crowdloan-id <u32>`

**Pallet dispatchable**
- `Crowdloan::refund(origin, crowdloan_id)`

**Storage keys touched**
- `Crowdloan::Crowdloans`
- `Crowdloan::Contributions`

**On-chain events**
- `Crowdloan::PartiallyRefunded { crowdloan_id }` or
- `Crowdloan::AllRefunded { crowdloan_id }`

**Output JSON schema**
- Current implementation prints plain text only (`"Refund submitted. Tx: <hash>"`).

---

## `agcli crowdloan dissolve`

Remove a refunded, non-finalized crowdloan and return creator deposit.

```bash
agcli crowdloan dissolve --crowdloan-id 3
```

**Clap flags**
- `--crowdloan-id <u32>`

**Pallet dispatchable**
- `Crowdloan::dissolve(origin, crowdloan_id)`

**Storage keys touched**
- `Crowdloan::Crowdloans` (removed)
- `Crowdloan::Contributions` (creator row removed)

**On-chain events**
- `Crowdloan::Dissolved { crowdloan_id }`

**Output JSON schema**
- Current implementation prints plain text only (`"Crowdloan dissolved. Tx: <hash>"`).

---

## `agcli crowdloan update-cap`

Update funding cap.

```bash
agcli crowdloan update-cap --crowdloan-id 3 --cap 250.0
```

**Clap flags**
- `--crowdloan-id <u32>`
- `--cap <f64>` (TAO)

**Pallet dispatchable**
- `Crowdloan::update_cap(origin, crowdloan_id, new_cap)`

**Storage keys touched**
- `Crowdloan::Crowdloans`

**On-chain events**
- `Crowdloan::CapUpdated { crowdloan_id, new_cap }`

**Output JSON schema**
- Current implementation prints plain text only (`"Cap updated. Tx: <hash>"`).

---

## `agcli crowdloan update-end`

Update campaign end block.

```bash
agcli crowdloan update-end --crowdloan-id 3 --end-block 600000
```

**Clap flags**
- `--crowdloan-id <u32>`
- `--end-block <u32>`

**Pallet dispatchable**
- `Crowdloan::update_end(origin, crowdloan_id, new_end)`

**Storage keys touched**
- `Crowdloan::Crowdloans`

**On-chain events**
- `Crowdloan::EndUpdated { crowdloan_id, new_end }`

**Output JSON schema**
- Current implementation prints plain text only (`"End block updated. Tx: <hash>"`).

---

## `agcli crowdloan update-min-contribution`

Update minimum contribution requirement.

```bash
agcli crowdloan update-min-contribution --crowdloan-id 3 --min-contribution 2.0
```

**Clap flags**
- `--crowdloan-id <u32>`
- `--min-contribution <f64>` (TAO)

**Pallet dispatchable**
- `Crowdloan::update_min_contribution(origin, crowdloan_id, new_min_contribution)`

**Storage keys touched**
- `Crowdloan::Crowdloans`

**On-chain events**
- `Crowdloan::MinContributionUpdated { crowdloan_id, new_min_contribution }`

**Output JSON schema**
- Current implementation prints plain text only (`"Min contribution updated. Tx: <hash>"`).

---

## `agcli crowdloan list`

List campaigns from chain state.

```bash
agcli crowdloan list [--output table|json|csv]
```

**Clap flags**
- none (besides global)

**Read path**
- storage: `Crowdloan::Crowdloans` (iterated)

**On-chain events**
- none (read-only command)

**Output JSON schema**
- `Array<[id: u32, creator_ss58: String, deposit_rao: u64, raised_rao: u64, cap_rao: u64, end_block: u32, finalized: bool]>`

---

## `agcli crowdloan info`

Show one campaign.

```bash
agcli crowdloan info --crowdloan-id 3
```

**Clap flags**
- `--crowdloan-id <u32>`

**Read path**
- storage: `Crowdloan::Crowdloans(crowdloan_id)`

**On-chain events**
- none (read-only command)

**Output JSON schema**
- Current implementation prints plain text only; no JSON object is emitted for this command today.

---

## `agcli crowdloan contributors`

List contributors for one campaign.

```bash
agcli crowdloan contributors --crowdloan-id 3 [--output table|json|csv]
```

**Clap flags**
- `--crowdloan-id <u32>`

**Read path**
- intended storage key from pallet: `Crowdloan::Contributions(crowdloan_id, account_id)`

**On-chain events**
- none (read-only command)

**Output JSON schema**
- `Array<[contributor_ss58: String, amount_rao: u64]>`

---

## Source references

- CLI command surface: `src/cli/mod.rs` (`CrowdloanCommands`)
- CLI handler: `src/cli/network_cmds.rs` (`handle_crowdloan`)
- Client extrinsics and reads: `src/chain/extrinsics.rs`, `src/chain/queries.rs`
- Pallet: `subtensor/pallets/crowdloan/src/lib.rs`
