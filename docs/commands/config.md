# config — Persistent Configuration

Manage agcli configuration stored in `~/.agcli/config.toml`. Settings persist across invocations. Priority: CLI flags > env vars > config > defaults.

**No on-chain interaction.** All subcommands are purely local; no chain connection is required or attempted.

## Subcommands

### config show

Show current configuration as TOML.

```bash
agcli config show
```

**Flags:** none (inherits global `--output` but output is always TOML text regardless of `--output json`; see audit finding below).

**Output (table/TOML):**
```toml
network = "finney"
wallet = "default"
hotkey = "default"
output = "json"
```

If no keys are set the handler prints a plain-text hint:
```
No configuration set. Use 'agcli config set <key> <value>' to configure.
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Success |
| 14   | IO — config file unreadable (permissions, corrupt TOML) |

---

### config set

Set a single configuration key.

```bash
agcli config set --key <KEY> --value <VALUE>
```

**Flags:**
| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--key` | `String` | yes | Config key to set |
| `--value` | `String` | yes | Value to persist |

**Valid keys and their accepted values:**
| Key | Accepted values | Validation |
|-----|----------------|------------|
| `network` | `finney`, `test`, `local`, `archive` | `validate_config_network` — rejects unknown names |
| `endpoint` | WebSocket URL (`wss://…` or `ws://…`) | `validate_url` — must start with `ws://` or `wss://` |
| `wallet_dir` | Filesystem path | none (any string accepted) |
| `wallet` | Wallet name | none (any string accepted) |
| `hotkey` | Hotkey name | none (any string accepted) |
| `output` | `table`, `json`, `csv` | exact match; other values rejected |
| `proxy` | SS58 address | `validate_ss58` — must be valid SS58 |
| `live_interval` | Unsigned integer (seconds) | parsed as `u64`; non-numeric rejected |
| `batch` | `true` or `false` | parsed as `bool`; other values rejected |
| `spending_limit.<netuid>` | Non-negative float (TAO) | netuid must be `0–65535` or `*`; value must be finite and ≥ 0 |

> **Note:** The config struct also has `finalization_timeout` (u64, seconds) and `mortality_blocks` (u64) fields that survive TOML round-trips, but there is **no `config set` key** for either. They can only be written by directly editing `~/.agcli/config.toml`. See audit findings.

**Examples:**
```bash
agcli config set --key network --value finney
agcli config set --key endpoint --value wss://my-node:443
agcli config set --key output --value json
agcli config set --key batch --value true
agcli config set --key live_interval --value 30
agcli config set --key spending_limit.97 --value 100.0
agcli config set --key spending_limit.* --value 500.0
```

**Output (success):**
```
Set network = finney
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Key written to `~/.agcli/config.toml` |
| 12   | Validation — unknown key, bad network name, bad URL, bad SS58, bad bool/u64, invalid netuid |
| 14   | IO — config file not writable |

---

### config unset

Remove a configuration key (sets it to absent/`None`).

```bash
agcli config unset --key <KEY>
```

**Flags:**
| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--key` | `String` | yes | Config key to remove |

**Valid keys:** same set as `config set` (`network`, `endpoint`, `wallet_dir`, `wallet`, `hotkey`, `output`, `proxy`, `live_interval`, `batch`, `spending_limit.<netuid>`).

**Examples:**
```bash
agcli config unset --key network
agcli config unset --key spending_limit.97
```

**Output (success):**
```
Unset network
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Key removed |
| 12   | Validation — unknown key |
| 14   | IO — config file not writable |

---

### config path

Print the absolute path to the config file (does not check whether it exists).

```bash
agcli config path
```

**Flags:** none.

**Output:**
```
/root/.agcli/config.toml
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Always succeeds (path is computed from `$HOME`) |

---

### config cache-clear

Delete all disk-cached entries (subnet info, dynamic info, etc.).

```bash
agcli config cache-clear
```

**Flags:** none.

**Output (entries present):**
```
Cleared 3 cached entries.
```

**Output (already empty):**
```
Disk cache is already empty.
```

**Cache location:** `~/.agcli/cache/` (from `disk_cache::path()`).

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Always succeeds (missing entries are silently skipped) |

---

### config cache-info

Show disk cache statistics (entry count, per-entry size, total size, directory path).

```bash
agcli config cache-info
```

**Flags:** none.

**Output (entries present):**
```
Cache directory: /root/.agcli/cache
  subnet_metagraph_1 (12.4KB)
  dynamic_info_18 (3.1KB)
Total: 2 entries, 15.5KB
```

**Output (empty):**
```
Cache directory: /root/.agcli/cache
No cached entries.
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0    | Always succeeds |

---

## Configurable Keys Reference

| Key | Type | Description | Default |
|-----|------|-------------|---------|
| `network` | string | Default network | `finney` |
| `endpoint` | string | Custom RPC WebSocket URL (overrides `network`) | — |
| `wallet_dir` | string | Wallet directory | `~/.bittensor/wallets` |
| `wallet` | string | Default wallet name | `default` |
| `hotkey` | string | Default hotkey name | `default` |
| `output` | string | Default output format (`table`, `json`, `csv`) | `table` |
| `proxy` | string | Default proxy account (SS58) | — |
| `live_interval` | u64 | Default `--live` poll interval in seconds | — |
| `batch` | bool | Enable batch (non-interactive) mode globally | `false` |
| `spending_limit.<netuid>` | f64 | Max TAO per stake operation on subnet N | — |
| `spending_limit.*` | f64 | Global max TAO per stake operation | — |
| `finalization_timeout` | u64 | Extrinsic finalization timeout in seconds (TOML only — no `config set` key) | `30` |
| `mortality_blocks` | u64 | Extrinsic mortality in blocks (TOML only — no `config set` key) | ~64 |

## Spending Limits (Agent Safety)

```bash
agcli config set --key spending_limit.97 --value 100.0   # Max 100 TAO on SN97
agcli config set --key spending_limit.* --value 500.0     # Global max
```

Pre-flight check runs before every `stake add`. Prevents accidental large stakes.

## Config writes are atomic

Uses temp-file + rename to prevent corruption on crash. File permissions are set to `0600` (owner-only) before rename on Unix.

## Pallet Reference

`config` is **entirely local**. It does not call any pallet or dispatchable. There are no on-chain events, no SCALE encoding, and no storage keys involved.

## Source Code

**Handler**: [`src/cli/system_cmds.rs`](https://github.com/unarbos/agcli/blob/main/src/cli/system_cmds.rs) — `handle_config()` at L9.

**Subcommand dispatch:**
- `Show` → L11: `Config::load()` + `toml::to_string_pretty`
- `Set` → L23: key dispatch + validation + `Config::save()`
- `Unset` → L68: key dispatch + `Config::save()`
- `Path` → L92: `Config::default_path().display()`
- `CacheClear` → L96: `disk_cache::list_keys()` + `disk_cache::remove()`
- `CacheInfo` → L108: `disk_cache::list_keys()` + per-entry `fs::metadata`

**Config struct**: [`src/config.rs`](https://github.com/unarbos/agcli/blob/main/src/config.rs) — `Config` struct with atomic `save_to`.

**Disk cache**: [`src/queries/disk_cache.rs`](https://github.com/unarbos/agcli/blob/main/src/queries/disk_cache.rs) — `list_keys`, `remove`, `path`.

## Audit Findings

See `tests/audit_config.rs` and the worker handoff for the full findings list.

1. **`cache-clear` and `cache-info` missing from original docs** — both subcommands existed in `ConfigCommands` but were not documented.
2. **`finalization_timeout` and `mortality_blocks` have no `config set` surface** — the two fields exist in `Config` struct and survive TOML round-trips, but no `config set --key finalization_timeout --value N` arm exists in the handler.
3. **`config show` ignores `--output json`** — output is always TOML text; agents expecting a JSON envelope when `--output json` is set will not get one from this subcommand.
