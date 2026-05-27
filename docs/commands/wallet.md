# wallet — Wallet Management

Create, import, and manage sr25519 keypairs. Wallets consist of a coldkey (encrypted with NaCl SecretBox / XSalsa20-Poly1305, compatible with Python `bittensor-wallet`) and one or more hotkeys (plaintext, for automated operations).

---

## Global flags (apply to every wallet subcommand)

| Flag | Env | Default | Description |
|------|-----|---------|-------------|
| `-w / --wallet` | `AGCLI_WALLET` | `default` | Wallet name to operate on |
| `--wallet-dir` | `AGCLI_WALLET_DIR` | `~/.bittensor/wallets` | Root directory for all wallets |
| `--hotkey` | `AGCLI_HOTKEY` | `default` | Hotkey name to load for on-chain ops |
| `--password` | `AGCLI_PASSWORD` | — | Coldkey decryption password |
| `--json` | — | — | Emit JSON output instead of human text |
| `--csv` | — | — | Emit CSV output |

---

## Subcommands

### wallet create

Create a new wallet (coldkey + hotkey).

```bash
agcli wallet create [--name <name>] [--hotkey-name <name>] [--password <pw>] [--no-mnemonic]
```

**Flags**

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--name` | `String` | `default` | Wallet directory name |
| `--hotkey-name` | `Option<String>` | `default` | Name of the initial hotkey |
| `--password` / `AGCLI_PASSWORD` | `Option<String>` | — | Coldkey encryption password; prompted interactively if omitted |
| `--no-mnemonic` | `bool` | `false` | Suppress mnemonic display; retrieve later with `wallet show-mnemonic` |

**Output (JSON)**
```json
{
  "name": "mywallet",
  "coldkey": "<SS58>",
  "hotkey": "<SS58>",
  "coldkey_mnemonic": "<12-word phrase>",
  "hotkey_mnemonic": "<12-word phrase>"
}
```
`coldkey_mnemonic` and `hotkey_mnemonic` are omitted when `--no-mnemonic` is set.

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Success |
| `12` | Validation error — invalid wallet/hotkey name or empty password |
| `14` | I/O error — wallet already exists or permission denied |

**On-chain**: No extrinsic; purely local key generation.

---

### wallet list

List all wallets in the wallet directory.

```bash
agcli wallet list
```

**Output (JSON)**
```json
[{"name": "default", "coldkey": "<SS58>"}, ...]
```

**Output (CSV)**
```
name,coldkey
default,<SS58>
```

**Exit codes**: `0` success, `14` I/O error.

---

### wallet show

Show wallet details. Without `--wallet`, shows all wallets. With `-w <name>`, shows only that wallet.

```bash
agcli wallet show [--all]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--all` | `bool` | `false` | Include all hotkeys per wallet in output |

**Output (JSON, with `--all`)**
```json
[
  {
    "name": "default",
    "coldkey": "<SS58>",
    "hotkeys": [{"name": "default", "address": "<SS58>"}]
  }
]
```

**Output (CSV, with `--all`)**
```
wallet,coldkey,hotkey_name,hotkey_address
default,<SS58>,default,<SS58>
```

**Exit codes**: `0` success, `12` if `--wallet` was set but not found, `14` I/O error.

---

### wallet import

Import a wallet from a BIP39 mnemonic phrase (creates coldkey only; no hotkey).

```bash
agcli wallet import [--name <name>] [--mnemonic "<phrase>"] [--password <pw>]
```

| Flag | Env | Type | Default | Description |
|------|-----|------|---------|-------------|
| `--name` | — | `String` | `default` | Wallet name |
| `--mnemonic` | `AGCLI_MNEMONIC` | `Option<String>` | — | BIP39 mnemonic; prompted interactively if omitted |
| `--password` | `AGCLI_PASSWORD` | `Option<String>` | — | Encryption password |

**Output (JSON)**
```json
{"name": "mywallet", "coldkey": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid mnemonic or empty password, `14` I/O error.

**On-chain**: No extrinsic.

---

### wallet regen-coldkey

Regenerate (overwrite) the coldkey for the active wallet from a mnemonic. Uses `--wallet` / `-w` to select the target wallet.

```bash
agcli wallet regen-coldkey [--mnemonic "<phrase>"] [--password <pw>]
```

| Flag | Env | Type | Default |
|------|-----|------|---------|
| `--mnemonic` | `AGCLI_MNEMONIC` | `Option<String>` | — |
| `--password` | `AGCLI_PASSWORD` | `Option<String>` | — |

**Output (JSON)**
```json
{"coldkey": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid mnemonic or empty password, `14` I/O error.

**On-chain**: No extrinsic.

---

### wallet regen-hotkey

Regenerate (overwrite) a named hotkey from a mnemonic.

```bash
agcli wallet regen-hotkey --name <hotkey-name> [--mnemonic "<phrase>"]
```

| Flag | Env | Type | Default |
|------|-----|------|---------|
| `--name` | — | `String` | `default` |
| `--mnemonic` | `AGCLI_MNEMONIC` | `Option<String>` | — |

**Output (JSON)**
```json
{"name": "default", "hotkey": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid name or mnemonic, `14` I/O error.

**On-chain**: No extrinsic.

---

### wallet new-hotkey

Generate a fresh hotkey for the active wallet.

```bash
agcli wallet new-hotkey --name <hotkey-name>
```

| Flag | Type | Description |
|------|------|-------------|
| `--name` | `String` | Required; must be a valid name (alphanumeric + `-_`) |

**Output (JSON)**
```json
{"name": "miner1", "hotkey": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid name, `14` hotkey already exists or I/O error.

**On-chain**: No extrinsic.

---

### wallet sign

Sign an arbitrary message with the coldkey. Output is always JSON (ignores `--csv`/`--json` flag; always emits JSON).

```bash
agcli wallet sign --message <msg> [--password <pw>]
```

| Flag | Type | Description |
|------|------|-------------|
| `--message` | `String` | UTF-8 string or `0x`-prefixed hex bytes |

**Output (always JSON)**
```json
{
  "signer": "<SS58>",
  "message": "<original message>",
  "signature": "0x<128 hex chars>"
}
```

**Exit codes**: `0` success, `11` auth/wallet error (wrong password, no coldkey), `12` invalid hex input.

**On-chain**: No extrinsic.

**Note (audit finding)**: `wallet sign` always emits JSON regardless of the `--json` flag; the other text-output subcommands respect the output format. This is intentional — signature output should be machine-parseable — but undocumented.

---

### wallet verify

Verify a sr25519 signature against a public key.

```bash
agcli wallet verify --message <msg> --signature <0xhex> [--signer <SS58>] [--password <pw>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--message` | `String` | — | Original message (UTF-8 or `0x` hex) |
| `--signature` | `String` | — | 64-byte signature as 0x-prefixed hex |
| `--signer` | `Option<String>` | wallet coldkey SS58 | SS58 address of the expected signer |

**Output (always JSON)**
```json
{"signer": "<SS58>", "valid": true}
```

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Signature valid |
| `1` | Signature invalid (via `anyhow::bail!`) |
| `11` | Wallet/auth error (no coldkey to default signer from) |
| `12` | Invalid hex in `--signature` or `--message`, wrong signature length |

---

### wallet derive

Derive an SS58 address from a public key hex or mnemonic. No private key is ever printed.

```bash
agcli wallet derive --input <0xhex-pubkey | mnemonic phrase>
```

| Flag | Type | Description |
|------|------|-------------|
| `--input` | `String` | Either `0x`-prefixed 32-byte hex public key, or a BIP39 mnemonic |

**Output (always JSON)**
```json
{"public_key": "0x<64 hex chars>", "ss58": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid hex or invalid mnemonic.

**On-chain**: No extrinsic.

---

### wallet dev-key (alias: dev)

Create a wallet from a Substrate dev-account URI (Alice, Bob, Charlie, Dave, Eve, Ferdie). Useful for localnet testing.

```bash
agcli wallet dev-key [--uri <URI>] [--password <pw>]
# Alias:
agcli wallet dev [--uri Alice] [--password <pw>]
```

| Flag | Env | Type | Default | Description |
|------|-----|------|---------|-------------|
| `--uri` | — | `String` | `Alice` | Dev account name (`Alice` / `alice` / `//Alice`) or full URI |
| `--password` | `AGCLI_PASSWORD` | `Option<String>` | — | Encryption password for coldkey |

**Behavior**: The CLI normalizes `Alice` → `//Alice`. Substrate URI derivation is **case-sensitive** (`//Alice` ≠ `//alice`).

**Output (JSON)**
```json
{"name": "alice", "uri": "//Alice", "coldkey": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "hotkey": "<SS58>"}
```

**Exit codes**: `0` success, `12` invalid URI, `14` I/O error.

**On-chain**: No extrinsic. Common use: create Alice wallet then run `agcli wallet associate-hotkey` on localnet.

---

### wallet associate-hotkey

Submit the `SubtensorModule::try_associate_hotkey` extrinsic, which records the hotkey→coldkey association on-chain.

```bash
agcli wallet associate-hotkey [--hotkey-address <SS58>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--hotkey-address` | `Option<String>` | wallet hotkey SS58 | Hotkey to associate with the signing coldkey |

**Behavior**

1. Connects to the chain specified by `--network`.
2. Unlocks the coldkey (requires `--password` / `AGCLI_PASSWORD`).
3. Resolves `--hotkey-address` or falls back to the wallet's default hotkey.
4. Submits `SubtensorModule::try_associate_hotkey(hotkey: AccountId)` signed by the coldkey.

**Output (JSON)**
```json
{"tx_hash": "0x<hash>", "message": "Hotkey associated."}
```

**Pallet reference**: `SubtensorModule` (`pallets/subtensor`)
**Dispatchable**: `try_associate_hotkey(origin, hotkey: T::AccountId)`
**SCALE encoding**: hotkey AccountId32 passed as `Value::from_bytes(account_id.0)` (32-byte array).
**Storage key written**: `SubtensorModule::Owner` — maps hotkey → coldkey.

**On-chain events emitted** (from pallet):
- `SubtensorModule::HotkeyAssociated { coldkey, hotkey }` (if implemented)

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Extrinsic included and finalized |
| `10` | Network error (connection failed) |
| `11` | Auth error (wrong password, no hotkey loaded) |
| `12` | Validation error (invalid SS58) |
| `13` | Chain error (extrinsic rejected — e.g. hotkey already associated with different coldkey) |
| `15` | Timeout |

---

### wallet check-swap

Query the `SubtensorModule::ColdkeySwapAnnouncements` storage map to determine whether a coldkey swap is scheduled.

```bash
agcli wallet check-swap [--address <SS58>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--address` | `Option<String>` | wallet coldkey SS58 | Address to check swap status for |

**Output (JSON, swap scheduled)**
```json
{
  "address": "<SS58>",
  "swap_scheduled": true,
  "execution_block": 12345,
  "new_coldkey_hash": "0x<64 hex chars>"
}
```

**Output (JSON, no swap)**
```json
{"address": "<SS58>", "swap_scheduled": false}
```

**Note (audit finding)**: The text output prints `new_coldkey_hash` but the docs previously showed `new_coldkey` as the JSON key name. The actual key is `new_coldkey_hash`. The JSON schema is now corrected here.

**Pallet reference**: `SubtensorModule` (`pallets/subtensor`)
**Storage map**: `SubtensorModule::ColdkeySwapAnnouncements` — key: coldkey AccountId, value: `(execution_block: u32, new_coldkey_hash: H256)`.
**Dispatchable (write path)**: Swap is initiated via `SubtensorModule::swap_coldkey` (separate `agcli swap coldkey` command, not under `wallet`).

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Query succeeded (may or may not have a scheduled swap) |
| `10` | Network error |
| `12` | Invalid SS58 address |

---

### wallet show-mnemonic

Decrypt and display the coldkey mnemonic. Requires the coldkey password.

```bash
agcli wallet show-mnemonic [--password <pw>]
```

| Flag | Env | Type | Description |
|------|-----|------|-------------|
| `--password` | `AGCLI_PASSWORD` | `Option<String>` | Coldkey decryption password |

**Output (JSON)**
```json
{"mnemonic": "<12 or 24-word BIP39 phrase>"}
```

**Output (text)**: prints the mnemonic directly to stdout.

**Exit codes**

| Code | Meaning |
|------|---------|
| `0` | Success |
| `11` | Auth error — wrong password or coldkey not found |
| `14` | I/O error — coldkey file missing |

**On-chain**: No extrinsic.

---

## Wallet Storage Layout

```
~/.bittensor/wallets/
└── <wallet-name>/
    ├── coldkey              # NaCl SecretBox encrypted sr25519 (JSON envelope)
    ├── coldkeypub.txt       # Raw hex-encoded 32-byte sr25519 public key (no 0x prefix, 0o644)
    └── hotkeys/
        ├── default          # Plaintext BIP39 mnemonic (0o600)
        └── <hotkey-name>    # Additional hotkeys
```

File permissions:
- `coldkey`: `0o600` (owner read/write only)
- `coldkeypub.txt`: `0o644` (world-readable)
- `hotkeys/*`: `0o600` (owner read/write only)

All file writes are **atomic** (write to `.tmp` then rename) to prevent partial writes on crash.

---

## Key Concepts

- **Coldkey**: Main signing key, always encrypted. Used for transfers, staking, governance, and on-chain signing.
- **Hotkey**: Automated key, stored plaintext. Used for weight setting, serving, registration.
- **SS58**: Base58 encoding with network prefix `42` (Bittensor / generic Substrate).
- **Mnemonic**: BIP39 phrase — 12 or 24 words. Both lengths accepted.
- **NaCl SecretBox**: XSalsa20-Poly1305 authenticated encryption compatible with Python `bittensor-wallet`.

---

## Security Notes

- Password can be supplied via `--password`, `AGCLI_PASSWORD` env var, or interactive prompt.
- Wallet creation is protected by a directory-level lock (prevents concurrent creation corruption).
- Mnemonics are **zeroized** (overwritten in memory) immediately after use.
- Never expose mnemonics or private keys in logs or output. The `--no-mnemonic` flag suppresses mnemonic display on creation.
- `wallet sign` and `wallet derive` never print private keys.

---

## Exit Code Reference

Exit codes are defined in `src/error.rs::exit_code`:

| Code | Constant | Meaning |
|------|----------|---------|
| `0` | — | Success |
| `1` | `GENERIC` | Uncategorized error |
| `10` | `NETWORK` | Connection/timeout failure |
| `11` | `AUTH` | Wrong password, missing key, locked wallet |
| `12` | `VALIDATION` | Invalid input (bad SS58, bad mnemonic, empty password) |
| `13` | `CHAIN` | Extrinsic rejected or chain runtime error |
| `14` | `IO` | File not found, permission denied |
| `15` | `TIMEOUT` | Operation exceeded deadline |

---

## Source Code References

- **agcli handler**: [`src/cli/wallet_cmds.rs`](../src/cli/wallet_cmds.rs) — `handle_wallet()` dispatches Create, List, Show, Import, RegenColdkey, RegenHotkey, NewHotkey, Sign, Verify, Derive, DevKey, ShowMnemonic.
- **On-chain handlers**: [`src/cli/commands.rs`](../src/cli/commands.rs) — AssociateHotkey (L123) and CheckSwap (L147) are dispatched before the general `Wallet(cmd)` match arm.
- **Extrinsic**: [`src/chain/extrinsics.rs`](../src/chain/extrinsics.rs) — `try_associate_hotkey()` at L908.
- **Query**: [`src/chain/queries.rs`](../src/chain/queries.rs) — `get_coldkey_swap_scheduled()` at L760.

---

## Related Commands

- `agcli balance` — Check wallet balance
- `agcli stake list` — View stakes for wallet
- `agcli swap coldkey` — Schedule coldkey swap (separate command group)
- `agcli proxy add` — Delegate signing to another key
