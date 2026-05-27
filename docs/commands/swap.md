# swap — Key Rotation Operations

Rotate hotkeys or schedule coldkey swaps. These are critical security operations; hotkey swaps execute immediately, while the coldkey flow now uses a multi-step announcement protocol.

**Audit note (2026-05)**: The `swap coldkey` subcommand calls the deprecated `schedule_swap_coldkey` dispatchable, which always returns `Error::Deprecated` on-chain. Several on-chain swap dispatchables have no CLI surface at all — see §[Missing CLI Surface](#missing-cli-surface) below.

---

## Subcommands

### `swap hotkey`

Swap the current wallet hotkey to a new one. Transfers all registrations, ownership, stake metadata, and root membership to the new hotkey across all subnets (or a single subnet if the pallet `swap_hotkey_v2` variant is used; `agcli` hard-codes `None` for `netuid`, so all subnets are migrated).

```text
agcli swap hotkey --new-hotkey <SS58>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--new-hotkey` | SS58 address | yes | New hotkey to swap to |
| `--hotkey-name` | string | no (global) | Hotkey keypair name in the wallet (default: `default`) |
| `--wallet-name` | string | no (global) | Wallet name (default: `default`) |
| `--wallet-path` | path | no (global) | Wallet directory override |
| `--password` | string | no (global) | Coldkey decryption password |
| `--yes` / `-y` | flag | no (global) | Skip confirmation prompts |

**Exit codes**

| Code | Meaning |
|------|---------|
| 0 | Hotkey swapped successfully |
| 11 | Authentication failure — wrong password or missing coldkey |
| 12 | Validation failure — `new_hotkey` is not a valid SS58 address |
| 13 | Chain error — e.g. `NewHotKeyIsSameWithOld`, `HotKeySetTxRateLimitExceeded`, `NonAssociatedColdKey`, `HotKeyAlreadyRegisteredInSubNet` |
| 10 | Network error (connection refused, timeout) |

**Output** (plain text; `--output json` has no effect — all swap write ops use `println!` directly):

```text
Swapping hotkey 5Grwv... → 5FHne...
Hotkey swapped: 5Grwv... → 5FHne….
  Tx: 0xabc...
```

**Pallet ref**: `SubtensorModule::swap_hotkey` (call index 70), `pallets/subtensor/src/macros/dispatches.rs`

**Arguments on-chain**:

| Position | Name | SCALE type | agcli encoding |
|----------|------|-----------|----------------|
| 1 | `hotkey` | `AccountId32` | `ss58_to_account_id(old_hotkey_ss58)` |
| 2 | `new_hotkey` | `AccountId32` | `ss58_to_account_id(new_hotkey)` |
| 3 | `netuid` | `Option<NetUid>` | `None` (hard-coded — always swaps across all subnets) |

**Events emitted**:

- `HotkeySwapped { coldkey, old_hotkey, new_hotkey }` — when swap applies to all subnets
- `HotkeySwappedOnSubnet { coldkey, old_hotkey, new_hotkey, netuid }` — one event per subnet affected

**Storage keys written**: `Owner`, `OwnedHotkeys`, `Neurons`, `NeuronsToPruneAtNextEpoch`, `Stake`, `TotalHotkeyColdkeyStakesThisInterval`, `IsNetworkMember`, `Keys`, `Uids`, `LoadedEmission`, `HotkeyEmissionTempo`, `LastHotkeyEmissionDrain`, `TotalHotkeyAlpha`, `TotalColdkeyAlpha`, `ParentKeys`, `ChildKeys`, `StakingHotkeys`, `LastTxBlock`, `TransactionKeyBlock`.

---

### `swap coldkey`

**⚠ Deprecated — always fails on-chain.**

This subcommand calls `SubtensorModule::schedule_swap_coldkey` (call index 73), which was deprecated in favour of the `announce_coldkey_swap` / `swap_coldkey_announced` two-step flow. The on-chain implementation immediately returns `Error::Deprecated`.

```text
agcli swap coldkey --new-coldkey <SS58>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--new-coldkey` | SS58 address | yes | Intended new coldkey |
| `--wallet-name` | string | no (global) | Wallet name |
| `--password` | string | no (global) | Coldkey decryption password |
| `--yes` / `-y` | flag | no (global) | Skip confirmation prompts |

**Exit codes**

| Code | Meaning |
|------|---------|
| 12 | Validation — invalid SS58 |
| 13 | Chain error — **always** `Deprecated` at current chain version |
| 11 | Auth failure — wrong password |

**Output** (plain text; `--output json` has no effect):

```text
Scheduling coldkey swap to 5FHne...
```

Then the call fails with a chain error.

**Pallet ref**: `SubtensorModule::schedule_swap_coldkey` (call index 73, **deprecated**).

**New replacement flow** (not yet surfaced in `agcli`):

1. `SubtensorModule::announce_coldkey_swap(new_coldkey_hash: BlakeTwo256Hash)` — call index 125, pays swap fee on first announcement.
2. Wait `ColdkeySwapAnnouncementDelay` blocks.
3. `SubtensorModule::swap_coldkey_announced(new_coldkey: AccountId)` — call index 126, validates hash and executes swap.

---

### `swap evm-key`

Associate an Ethereum (EVM) address with your SS58 account by submitting a cryptographic proof. The proof is an EVM personal-sign over the SS58 public key + a recent block number.

```text
agcli swap evm-key --evm-address <0x…> --block-number <N> --signature <0x…>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--evm-address` | hex string (20 bytes, `0x`-prefixed) | yes | EVM address to associate |
| `--block-number` | u32 | yes | Block number used when producing the EVM signature |
| `--signature` | hex string (65 bytes, `0x`-prefixed; r‖s‖v) | yes | EVM personal-sign signature |
| `--wallet-name` | string | no (global) | Wallet name |
| `--password` | string | no (global) | Coldkey decryption password |
| `--yes` / `-y` | flag | no (global) | Skip confirmation prompts |

**⚠ Argument mismatch with pallet**: The pallet dispatchable `SubtensorModule::associate_evm_key` (call index 93) expects four arguments: `(netuid: NetUid, evm_key: H160, block_number: u64, signature: Signature)`. The agcli handler omits `netuid` entirely and passes `block_number` as `u32` (up-cast to `u128` for SCALE), whereas the pallet expects `u64`. Both mismatches will cause SCALE decode failures at the node. See §[Findings](#findings--audit-observations) in this file.

**Exit codes**

| Code | Meaning |
|------|---------|
| 0 | EVM key associated |
| 12 | Validation — invalid hex for address or signature, wrong length |
| 13 | Chain error — SCALE mismatch, hotkey not registered, rate limit, wrong signature |
| 11 | Auth failure |

**Output** (JSON-aware via `print_tx_result`):

```text
Associating EVM address 0x1234… with your account
EVM key 0x1234… associated
```

With `--output json`:

```json
{"status":"ok","message":"EVM key 0x1234… associated","tx":"0xabc…"}
```

**Pallet ref**: `SubtensorModule::associate_evm_key` (call index 93), `pallets/subtensor/src/macros/dispatches.rs`.

**Events emitted**: `EvmKeyAssociated { ... }` (emitted by `do_associate_evm_key`).

---

## Missing CLI Surface

The following on-chain swap dispatchables exist in `SubtensorModule` but have **no corresponding `agcli swap` subcommand**:

| Dispatchable | Call index | Purpose | Who can call |
|---|---|---|---|
| `announce_coldkey_swap(new_coldkey_hash)` | 125 | Phase 1 of new coldkey rotation: announce with BlakeTwo256 hash, pay swap fee | signed (coldkey) |
| `swap_coldkey_announced(new_coldkey)` | 126 | Phase 2: execute swap after announcement delay | signed (coldkey) |
| `dispute_coldkey_swap()` | 127 | Self-dispute a pending swap announcement (prevents execution until triumvirate resolves) | signed (same coldkey) |
| `reset_coldkey_swap(coldkey)` | 128 | Root: clear announcement + dispute for any coldkey | root/sudo |
| `clear_coldkey_swap_announcement()` | 133 | Clear your own expired announcement (after reannouncement delay, undisputed) | signed (coldkey) |
| `swap_coldkey(old, new, cost)` | 71 | Root: unconditional immediate coldkey swap without announcement | root/sudo |
| `swap_hotkey_v2(old, new, netuid, keep_stake)` | 72 | Hotkey swap with per-subnet and keep-stake options | signed (coldkey) |

---

## Source Code References

- **CLI enum**: `src/cli/mod.rs` — `pub enum SwapCommands` (L1728–L1754)
- **Handler**: `src/cli/network_cmds.rs` — `handle_swap()` (L278–L361)
- **Extrinsics**:
  - `src/chain/extrinsics.rs:719` — `Client::schedule_swap_coldkey`
  - `src/chain/extrinsics.rs:733` — `Client::swap_hotkey`
  - `src/chain/extrinsics.rs:2271` — `Client::associate_evm_key`
- **Pallet dispatches**: `subtensor/pallets/subtensor/src/macros/dispatches.rs`
- **Events**: `subtensor/pallets/subtensor/src/macros/events.rs`
- **Swap logic**: `subtensor/pallets/subtensor/src/swap/`

## Related Commands

- `agcli wallet check-swap` — Check coldkey swap status
- `agcli explain --topic coldkey-swap` — Coldkey swap mechanics conceptual overview
