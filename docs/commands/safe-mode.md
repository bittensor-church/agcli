# safe-mode — Safe Mode Operations

Safe mode restricts the chain to a configured whitelist of extrinsics.  It is
an emergency tool: entering it prevents most transactions from executing.  Any
account can enter or extend safe mode by placing a deposit; privileged accounts
can force-enter, force-exit, or manage deposits via sudo.

Pallet: **SafeMode** (FRAME `pallet-safe-mode`, opentensor/polkadot-sdk fork
rev `7cc54bf2d50ae3921d718736dfeb0de9468539c7`, call-index range 0–7).

---

## Subcommands

### `safe-mode enter`

Permissionlessly enter safe mode for the configured `EnterDuration` blocks.
Reserves `EnterDepositAmount` from the caller's balance.

```
agcli [GLOBAL OPTIONS] safe-mode enter
```

**Flags:** none (inherits global `--wallet`, `--hotkey`, `--password`,
`--network`, `--yes`, `--output`).

**Pallet dispatchable:** `SafeMode::enter` (call index 0).
No explicit call arguments. Deposit amount and duration are pallet constants.

**SCALE encoding:** `(origin)` — signed extrinsic, no extra fields.

**On-chain events emitted:**
- `SafeMode::Entered { until: BlockNumber }` — block until which safe mode is
  active.
- `SafeMode::DepositPlaced { account: AccountId, amount: Balance }` — if the
  pallet is configured with a non-zero `EnterDepositAmount`.

**Storage keys written:** `SafeMode::EnteredUntil` (type `Option<BlockNumber>`).

**Output (current):** plain text — `Safe mode entered. Tx: 0x…`

**Output (missing):** the `Entered { until }` block number and
`DepositPlaced { amount }` are not printed. Agents need them to know when safe
mode expires and how much TAO was locked.

**Exit codes:**

| Code | Meaning |
|------|---------|
| 0    | Transaction included; safe mode entered. |
| 11   | Wallet unlock failed (bad password, missing key). |
| 13   | Chain rejected the call (e.g. `SafeMode::Entered` — already in safe mode; `SafeMode::NotConfigured` — enter disabled by config). |
| 10   | Network / RPC error. |
| 1    | Unexpected error. |

---

### `safe-mode extend`

Permissionlessly extend the current safe mode duration by `ExtendDuration`
blocks.  Reserves `ExtendDepositAmount` from the caller's balance.  Can only
be called while safe mode is active (`EnteredUntil` is `Some`).

```
agcli [GLOBAL OPTIONS] safe-mode extend
```

**Flags:** none.

**Pallet dispatchable:** `SafeMode::extend` (call index 2).
No explicit call arguments.

**SCALE encoding:** `(origin)` — signed extrinsic, no extra fields.

**On-chain events emitted:**
- `SafeMode::Extended { until: BlockNumber }`.
- `SafeMode::DepositPlaced { account, amount }`.

**Storage keys written:** `SafeMode::EnteredUntil`.

**Output (current):** plain text — `Safe mode extended. Tx: 0x…`

**Output (missing):** `Extended { until }` block and `DepositPlaced { amount }`
are not surfaced.

**Exit codes:** same as `enter` above; chain error code `SafeMode::Exited`
(safe mode not active) maps to exit code 13.

---

### `safe-mode force-enter`

Force-enter safe mode via `Sudo::sudo`.  Requires the caller to be the sudo key
(or a configured `ForceEnterOrigin`).  No deposit is reserved.  Duration is set
by chain config (`ForceEnterOrigin`), not by CLI flags.

```
agcli [GLOBAL OPTIONS] safe-mode force-enter
```

**Flags:** none.

**Pallet dispatchable:** `SafeMode::force_enter` (call index 1).
No explicit call arguments.

**SCALE encoding:** `Sudo::sudo(call: SafeMode::force_enter())` — no fields.

**On-chain events emitted:**
- `SafeMode::Entered { until: BlockNumber }`.
- `Sudo::Sudid { sudo_result: Ok(()) }`.

**Storage keys written:** `SafeMode::EnteredUntil`.

**Output (current):** plain text — `Safe mode force-entered. Tx: 0x…`

**Exit codes:**

| Code | Meaning |
|------|---------|
| 0    | Transaction included; safe mode force-entered. |
| 11   | Wallet unlock / sudo-key mismatch. |
| 13   | Chain error (e.g. `SafeMode::Entered`). |
| 10   | Network error. |
| 1    | Unexpected error. |

---

### `safe-mode force-exit`

Force-exit safe mode immediately via `Sudo::sudo`.  Requires the caller to be
the sudo key (or a configured `ForceExitOrigin`).

```
agcli [GLOBAL OPTIONS] safe-mode force-exit
```

**Flags:** none.

**Pallet dispatchable:** `SafeMode::force_exit` (call index 4).
No explicit call arguments.

**SCALE encoding:** `Sudo::sudo(call: SafeMode::force_exit())`.

**On-chain events emitted:**
- `SafeMode::Exited { reason: Force }`.
- `Sudo::Sudid { sudo_result: Ok(()) }`.

**Storage keys written:** `SafeMode::EnteredUntil` (set to `None`).

**Output (current):** plain text — `Safe mode force-exited. Tx: 0x…`

**Exit codes:**

| Code | Meaning |
|------|---------|
| 0    | Transaction included; safe mode exited. |
| 11   | Wallet / sudo error. |
| 13   | Chain error (`SafeMode::Exited` — not in safe mode). |
| 10   | Network error. |
| 1    | Unexpected error. |

---

## Pallet reference

### Storage

| Key | Type | Description |
|-----|------|-------------|
| `SafeMode::EnteredUntil` | `Option<BlockNumber>` | Block until which safe mode is active; `None` when inactive. |
| `SafeMode::Deposits` | `StorageDoubleMap<AccountId, BlockNumber, Balance>` | Reserved deposits by account and entry block. |

### All dispatchables (pallet surface vs. agcli surface)

| Call | Call index | agcli subcommand | Status |
|------|-----------|-----------------|--------|
| `enter()` | 0 | `safe-mode enter` | ✅ Surfaced |
| `force_enter()` | 1 | `safe-mode force-enter` | ✅ Surfaced |
| `extend()` | 2 | `safe-mode extend` | ✅ Surfaced |
| `force_extend()` | 3 | *(none)* | ❌ Missing |
| `force_exit()` | 4 | `safe-mode force-exit` | ✅ Surfaced |
| `force_slash_deposit(account, block)` | 5 | *(none)* | ❌ Missing |
| `release_deposit(account, block)` | 6 | *(none)* | ❌ Missing |
| `force_release_deposit(account, block)` | 7 | *(none)* | ❌ Missing |

### Events

| Event | Fields | Trigger |
|-------|--------|---------|
| `Entered` | `until: BlockNumber` | `enter` or `force_enter` succeeded |
| `Extended` | `until: BlockNumber` | `extend` or `force_extend` succeeded |
| `Exited` | `reason: ExitReason` (Force \| Timeout) | `force_exit` or timeout hook |
| `DepositPlaced` | `account: AccountId`, `amount: Balance` | `enter` or `extend` reserved a deposit |
| `DepositReleased` | `account: AccountId`, `amount: Balance` | `release_deposit` / `force_release_deposit` |
| `DepositSlashed` | `account: AccountId`, `amount: Balance` | `force_slash_deposit` |

---

## Missing subcommands (no agcli surface)

### `safe-mode force-extend` (pallet call index 3)

Privileged extension via sudo.  Equivalent to `extend` but without a deposit.
Needed for operators who want to extend safe mode without locking funds.

Proposed interface:
```
agcli safe-mode force-extend
```
No arguments; duration comes from `ForceExtendOrigin` config.

### `safe-mode release-deposit` (pallet call index 6)

Permissionlessly release a deposit placed by `enter` or `extend`, once safe
mode has ended and `ReleaseDelay` blocks have passed.

Without this subcommand, accounts that placed deposits to enter safe mode have
no way to recover their TAO through agcli.

Proposed interface:
```
agcli safe-mode release-deposit --account <SS58> --block <BLOCK_NUMBER>
```

### `safe-mode status` (read-only query)

Read `SafeMode::EnteredUntil` from chain state to report whether safe mode is
active and until which block.  The `Commands::SafeMode` doc comment already
lists "status" but the subcommand is not implemented.

Proposed interface:
```
agcli safe-mode status
```

Output:
```json
{ "active": true, "until": 1234567 }
```

---

## Related commands

- `agcli admin` — other sudo-level hyperparameter operations.
- `agcli batch` — submit multiple extrinsics atomically.
