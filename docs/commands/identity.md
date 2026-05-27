# identity — On-Chain Identity

Manage on-chain identity for hotkeys (Registry pallet) and subnets (SubtensorModule).

## Subcommands

| Subcommand | Writes chain? | Pallet | Dispatchable |
|---|---|---|---|
| `show` | No | Registry | storage read |
| `set` | Yes | Registry | `set_identity` |
| `clear` | Yes | Registry | `clear_identity` |
| `set-subnet` | Yes | SubtensorModule | `set_identity` (call index 68) |

---

### `identity show`

Query the on-chain identity for any SS58 address.

```
agcli identity show --address <SS58>
```

**Flags:**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--address` | `String` | Yes | SS58 address to look up |

**Output:** Human-readable text (always; `--output json/csv` is ignored — see Findings §1).

```
Identity for 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Name:        Alice
  URL:         https://alice.io
  GitHub:      (always empty — see Findings §3)
  Discord:     alice#1234
  Description: (always empty — see Findings §3)
  Image:       https://alice.io/logo.png
```

**Fields returned** (`ChainIdentity` struct → `src/types/chain_data.rs`):

| Displayed field | Source in `IdentityInfo` | Storage key |
|---|---|---|
| Name | `display` | `Registry.IdentityOf[account_id].info.display` |
| URL | `web` | `Registry.IdentityOf[account_id].info.web` |
| GitHub | hardcoded `""` | *(not stored in Registry pallet)* |
| Discord | `riot` | `Registry.IdentityOf[account_id].info.riot` |
| Description | hardcoded `""` | *(not in IdentityInfo)* |
| Image | `image` | `Registry.IdentityOf[account_id].info.image` |

**Exit codes:**

| Code | Meaning |
|---|---|
| 0 | Success (including "No identity found") |
| 12 | VALIDATION — invalid SS58 address |
| 10 | NETWORK — RPC connection failure |

---

### `identity set`

Set the on-chain identity for the signing coldkey (Registry pallet).

```
agcli identity set --name <NAME> [--url <URL>] [--github <REPO>] \
  [--description <TEXT>] [--image <URL>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--name` | `String` | Yes | — | Display name (maps to `IdentityInfo.display`) |
| `--url` | `String` | No | `""` | Website URL (maps to `IdentityInfo.web`) |
| `--github` | `String` | No | `""` | GitHub handle or repo (stored as additional field) |
| `--description` | `String` | No | `""` | Description text (stored as additional field) |
| `--image` | `String` | No | `""` | Logo/avatar URL (maps to `IdentityInfo.image`) |

**Requires wallet:** coldkey unlocked (global `--wallet`, `--password`).

**Pallet ref:** `Registry::set_identity` (call index 0), source: `subtensor/pallets/registry/src/lib.rs`.

**Pallet signature:**
```rust
pub fn set_identity(
    origin: OriginFor<T>,
    identified: T::AccountId,
    info: Box<IdentityInfo<T::MaxAdditionalFields>>,
) -> DispatchResult
```

`IdentityInfo` fields: `additional`, `display`, `legal`, `web`, `riot`, `email`, `pgp_fingerprint`, `image`, `twitter`.

**Events emitted on success:** `Registry::IdentitySet { who: AccountId }`.

**Exit codes:**

| Code | Meaning |
|---|---|
| 0 | Identity set, tx hash printed |
| 11 | AUTH — wallet locked / wrong password |
| 13 | CHAIN — `CannotRegister`, `TooManyFieldsInIdentityInfo` |
| 10 | NETWORK |

**Known issues (see Findings §2, §4):**
- The `identified` AccountId argument is missing from the dynamic call; the extrinsic will be rejected at the chain level with a SCALE decode error.
- Field names in the dynamic composite (`name`, `url`, `description`, `github_repo`) do not match `IdentityInfo` struct fields (`display`, `web`). There is no `description` or `github_repo` field in `IdentityInfo`; these values are silently dropped.
- Missing flags: `--discord`, `--email`, `--twitter`, `--legal` (Registry pallet `IdentityInfo` supports all of these).

---

### `identity clear`

Remove the on-chain identity for the signing coldkey (Registry pallet). Releases the held deposit.

```
agcli identity clear
```

**Flags:** none (wallet flags are global).

**Requires wallet:** coldkey unlocked.

**Pallet ref:** `Registry::clear_identity` (call index 1), source: `subtensor/pallets/registry/src/lib.rs`.

**Pallet signature:**
```rust
pub fn clear_identity(
    origin: OriginFor<T>,
    identified: T::AccountId,
) -> DispatchResultWithPostInfo
```

**Events emitted on success:** `Registry::IdentityDissolved { who: AccountId }`.

**Exit codes:**

| Code | Meaning |
|---|---|
| 0 | Identity cleared, tx hash printed |
| 11 | AUTH — wallet locked / wrong password |
| 13 | CHAIN — `NotRegistered` (no identity to clear) |
| 10 | NETWORK |

**Known issue (see Findings §5):** The `identified` AccountId argument is missing from the dynamic call (`vec![]` instead of `vec![identified_account_id]`). The extrinsic will fail at the chain level.

---

### `identity set-subnet`

Set identity metadata for a subnet. Caller must be the subnet owner.

```
agcli identity set-subnet --netuid <NETUID> --name <NAME> \
  [--github <REPO>] [--url <URL>]
```

**Flags:**

| Flag | Type | Required | Default | Description |
|---|---|---|---|---|
| `--netuid` | `u16` | Yes | — | Subnet UID |
| `--name` | `String` | Yes | — | Subnet display name |
| `--github` | `String` | No | `""` | GitHub repository (`owner/repo`) |
| `--url` | `String` | No | `""` | Subnet website URL |

**Requires wallet:** coldkey unlocked (must be subnet owner).

**Pallet ref (intended):** `SubtensorModule::set_subnet_identity` (call index 78), source: `subtensor/pallets/subtensor/src/macros/dispatches.rs`.

**Intended pallet signature:**
```rust
pub fn set_subnet_identity(
    origin: OriginFor<T>,
    netuid: NetUid,
    subnet_name: Vec<u8>,
    github_repo: Vec<u8>,
    subnet_contact: Vec<u8>,
    subnet_url: Vec<u8>,
    discord: Vec<u8>,
    description: Vec<u8>,
    logo_url: Vec<u8>,
    additional: Vec<u8>,
) -> DispatchResult
```

**Storage key written:** `SubtensorModule.SubnetIdentitiesV3[netuid]` (`SubnetIdentityV3` struct).

**Events emitted on success:** `SubtensorModule::SubnetIdentitySet` (see `subtensor/pallets/subtensor/src/macros/events.rs`).

**Exit codes:**

| Code | Meaning |
|---|---|
| 0 | Subnet identity set, tx hash printed |
| 11 | AUTH — wallet locked / wrong password |
| 12 | VALIDATION — invalid subnet name / URL / GitHub repo format |
| 13 | CHAIN — `NotSubnetOwner`, insufficient balance |
| 10 | NETWORK |

**Known issues (see Findings §6, §7, §8):**
- `set_subnet_identity` calls `SubtensorModule::set_identity` (hotkey identity, call index 68) **not** `SubtensorModule::set_subnet_identity` (subnet identity, call index 78). The `--netuid` value is silently ignored (parameter named `_netuid`).
- `SubnetIdentity` struct in `src/types/chain_data.rs` is missing `logo_url` (present in the chain's `SubnetIdentityV3`).
- Missing flags: `--discord`, `--description`, `--logo-url`, `--subnet-contact`.

---

## Storage Keys

| Query | Storage map | Key type | Value type |
|---|---|---|---|
| `identity show` | `Registry.IdentityOf` | `Twox64Concat(AccountId)` | `Registration<Balance, MaxAdditionalFields>` |
| `identity set-subnet` (read) | `SubtensorModule.SubnetIdentitiesV3` | `Blake2_128Concat(NetUid)` | `SubnetIdentityV3` |

---

## Related Commands

- `agcli subnet register-with-identity` — register a subnet and set its identity atomically
- `agcli view account` — account overview (includes identity summary)
- `agcli delegate show` — validator identity

---

## Audit Findings

1. **`identity show` ignores `--output json/csv`** — the handler always calls `println!` unconditionally; `ctx.output` is never checked. Agents expecting JSON output will receive unparseable human-readable text.

2. **`identity set` — wrong field names in SCALE composite** — `set_registry_identity` constructs a composite with keys `name`, `url`, `description`, `github_repo`, `image`. The Registry pallet's `IdentityInfo` struct uses `display`, `web`, `riot`, `email`, `pgp_fingerprint`, `image`, `twitter`. The keys `name`, `description`, and `github_repo` do not exist in `IdentityInfo`; they will either be ignored or cause a decode error depending on how subxt matches dynamic fields to the SCALE type.

3. **`identity show` fields `github` and `description` are always empty** — `chain_identity_from_registration` hardcodes `github: String::new()` and `description: String::new()`. The Registry pallet does not have a `github` field; it may be stored in `additional` key-value pairs, but the decoder does not extract it. Displayed output is misleading.

4. **`identity set` — missing `identified` argument** — The Registry pallet's `set_identity` takes `identified: T::AccountId` as its first explicit parameter (the target account to register, distinct from origin). `set_registry_identity` submits `vec![info]`, passing `info` at the position of `identified`. The chain will reject the extrinsic with a SCALE decode error (cannot decode a composite as an AccountId).

5. **`identity clear` — missing `identified` argument** — `clear_registry_identity` submits `Registry::clear_identity` with `vec![]` (empty argument list). The pallet requires `identified: T::AccountId`. The extrinsic will fail with a SCALE decode error on every call.

6. **`identity set-subnet` calls wrong dispatchable** — `Client::set_subnet_identity` calls `api::tx().subtensor_module().set_identity(...)` (hotkey identity, call index 68) instead of `SubtensorModule::set_subnet_identity` (call index 78). This means `agcli identity set-subnet` overwrites the **coldkey's hotkey identity**, not the subnet's identity. The `_netuid` parameter is entirely ignored (leading underscore in the Rust source).

7. **`identity set-subnet` — `subnet_contact` passed as `image`** — When calling the hotkey `set_identity` (wrong call, but mapping present): `identity.subnet_contact` is passed to parameter position 4 (`image`), not as `subnet_contact`. No `image` field is settable from the CLI for subnets.

8. **`SubnetIdentity` struct missing `logo_url`** — The chain's `SubnetIdentityV3` includes a `logo_url: Vec<u8>` field (added in a recent pallet upgrade). `src/types/chain_data.rs::SubnetIdentity` does not have this field. Both the CLI (`--logo-url` flag absent) and the internal storage type are out of date.

9. **`IdentityCommands::Set` missing flags** — The Registry pallet's `IdentityInfo` supports `legal`, `riot` (Discord/Matrix), `email`, `twitter`, `pgp_fingerprint`, and `additional` key-value pairs. Only `display` (`--name`), `web` (`--url`), `image` (`--image`) are partially surfaced. No `--discord`, `--email`, `--twitter`, `--legal` flags exist.

10. **`identity clear` not documented in previous docs** — The subcommand existed in code but was absent from `docs/commands/identity.md`.

---

## Suggested Follow-ups

- **Fix `identity set` / `identity clear` SCALE encoding** — add `identified` (coldkey's account ID) as the first arg in both `set_registry_identity` and `clear_registry_identity` dynamic calls, and align field names to `IdentityInfo` (`display`, `web`, `riot`, `email`, `image`, `twitter`).
- **Fix `identity set-subnet`** — change `Client::set_subnet_identity` to call `api::tx().subtensor_module().set_subnet_identity(netuid, ...)` and use the `netuid` parameter. Add `logo_url` field to `SubnetIdentity` and pass it through.
- **Add JSON output to `identity show`** — route through `ctx.output` and emit a `ChainIdentity` JSON blob.
- **Add missing flags** — `--discord` / `--email` / `--twitter` / `--legal` for `identity set`; `--discord` / `--description` / `--logo-url` / `--subnet-contact` for `identity set-subnet`.
- **Fix `github` and `description` in `chain_identity_from_registration`** — extract from `additional` key-value pairs rather than hardcoding empty strings.
