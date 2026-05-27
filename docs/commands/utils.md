# utils — Utility Commands

Miscellaneous tools: unit conversion and latency benchmarking.

> **Note:** `agcli doctor`, `agcli completions`, and `agcli update` are **not** subcommands
> of `utils`.  See [doctor.md](doctor.md) and the top-level help for those.

---

## Subcommands

### utils convert

Convert between TAO and RAO, or between TAO and subnet Alpha tokens.

**Flags**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--amount` | `f64` | No | Amount to convert (RAO → TAO when `--to-rao` is absent) |
| `--to-rao` | bool flag | No | Convert `--amount` as TAO → RAO (default direction is RAO → TAO) |
| `--tao` | `f64` | No | TAO amount to simulate swap to Alpha (requires `--netuid`) |
| `--alpha` | `f64` | No | Alpha amount to simulate swap to TAO (requires `--netuid`) |
| `--netuid` | `u16` | Conditional | Required when using `--tao` or `--alpha` for Alpha↔TAO conversion |

**Examples**

```bash
# RAO → TAO (default)
agcli utils convert --amount 1000000000
# TAO → RAO
agcli utils convert --amount 1.5 --to-rao
# TAO → Alpha (requires network connection)
agcli utils convert --tao 1.0 --netuid 1
# Alpha → TAO (requires network connection)
agcli utils convert --alpha 50.0 --netuid 1
```

**Output (table/default)**

```
1000000000 RAO = 1.000000000 TAO
```

**Output (JSON, `--output json`)**

RAO→TAO:
```json
{"rao": 1000000000, "tao": 1.0}
```

TAO→RAO:
```json
{"tao": 1.5, "rao": 1500000000}
```

TAO→Alpha:
```json
{"netuid": 1, "tao_in": 1.0, "alpha_out": 49.1234}
```

Alpha→TAO:
```json
{"netuid": 1, "alpha_in": 50.0, "tao_out": 1.0183}
```

**Exit codes**

| Code | Condition |
|------|-----------|
| 0 | Conversion successful |
| 1 (GENERIC) | Invalid RAO amount (not finite, negative, or out of u64 range) |
| 10 (NETWORK) | Chain connection failed (TAO↔Alpha only) |
| 12 (VALIDATION) | `--netuid` missing when `--tao` or `--alpha` specified |

**Pallet reference**: No on-chain write. TAO↔Alpha path calls `SubtensorModule.simulateSwapTaoForAlpha` / `simulateSwapAlphaForTao` (read-only chain queries via `src/chain/queries.rs`). No extrinsic is submitted. No events emitted.

---

### utils latency

Benchmark RPC endpoint latency.

**Flags**

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--extra` | `String` | No | Comma-separated list of additional WebSocket URLs to test |
| `--pings` | `usize` | No | Number of pings per endpoint (default: `5`) |

**Examples**

```bash
agcli utils latency
agcli utils latency --pings 10
agcli utils latency --extra "ws://localhost:9944,wss://custom.endpoint:443"
```

**Output (table/default)**

```
finney    wss://entrypoint-finney.opentensor.ai:443
  Connect: 84ms | avg: 102ms | min: 98ms | max: 115ms
```

**Output (JSON, `--output json`)**

```json
{
  "latency": [
    {
      "label": "finney",
      "url": "wss://entrypoint-finney.opentensor.ai:443",
      "connected": true,
      "avg_ms": 102,
      "min_ms": 98,
      "max_ms": 115,
      "failures": 0
    }
  ]
}
```

**Exit codes**

| Code | Condition |
|------|-----------|
| 0 | All endpoints tested (some may have failed to connect — check `connected` field) |
| 1 (GENERIC) | No endpoints configured (would only happen if network resolves to zero URLs) |

**Pallet reference**: Read-only RPC calls only (`get_block_number`). No extrinsic submitted. No events emitted.

---

### completions

Generate shell tab-completion scripts. This is a **top-level** command (not a `utils` subcommand).

**Flags**

| Flag | Type | Required | Allowed values |
|------|------|----------|----------------|
| `--shell` | `String` | Yes | `bash`, `zsh`, `fish`, `powershell` |

The `--shell` flag is validated at parse time by clap; any value outside the allowed set causes a parse error (exit code `2`, clap usage error) before `generate_completions` is called.

**Examples**

```bash
agcli completions --shell bash    >> ~/.bash_completion.d/agcli
agcli completions --shell zsh     > ~/.zfunc/_agcli
agcli completions --shell fish    > ~/.config/fish/completions/agcli.fish
agcli completions --shell powershell > _agcli.ps1
```

**Output**: Writes the completion script to **stdout**. No JSON variant — this command always writes raw shell script.

**Exit codes**

| Code | Condition |
|------|-----------|
| 0 | Completion script written to stdout |
| 2 | Invalid or missing `--shell` value (clap parse error, before handler runs) |

Note: An unsupported shell value that somehow bypasses clap validation would hit the `_ => eprintln!(...)` arm in `generate_completions` and return exit code `0` (the handler returns `()`, not `Result`). This is a latent bug — see Findings.

**Pallet reference**: No on-chain interaction. No events emitted.

---

### update

Self-update `agcli` to the latest version from GitHub via `cargo install --git`. This is a **top-level** command.

**Flags**: None.

**Example**

```bash
agcli utils convert --alpha 100.0 --netuid 18
# 100.0000 Alpha (SN18) → 98.7432 TAO
```

**Behavior**

Shells out to `cargo install --git https://github.com/unarbos/agcli --force`. Requires:
- `cargo` on `$PATH`
- Network access to `github.com`

Stdout output:
```
Updating agcli from GitHub...
agcli updated successfully!
```

**Exit codes**

| Code | Condition |
|------|-----------|
| 0 | `cargo install` succeeded |
| 1 (GENERIC) | `cargo install` exited non-zero (printed exit code in error message) |
| 1 (GENERIC) | `cargo` binary not found on `$PATH` (message: "Failed to run cargo install: …") |

Note: The exit code is always `1` (GENERIC) on failure — the error message is propagated through `anyhow::bail!` and classified by `src/error.rs::classify` as GENERIC because it contains no recognized keywords (`network`, `timeout`, `auth`, etc.).

**Pallet reference**: No on-chain interaction. No events emitted.

---

## Diagnostics (top-level)

```json
{ "netuid": 18, "alpha_in": 100.0, "tao_out": 98.7432 }
```

## Source Code

**agcli handler**: [`src/cli/system_cmds.rs`](https://github.com/unarbos/agcli/blob/main/src/cli/system_cmds.rs)

- `handle_utils()` — `utils convert`, `utils latency`
- `generate_completions()` — `completions`
- `handle_update()` — `update`

**CLI surface**: [`src/cli/mod.rs`](https://github.com/unarbos/agcli/blob/main/src/cli/mod.rs)

- `Commands::Completions { shell: String }` (line ~308)
- `Commands::Update` (line ~316)
- `Commands::Utils(UtilsCommands)` → `UtilsCommands::Convert { … }`, `UtilsCommands::Latency { … }` (line ~2297)

**Dispatch**: [`src/cli/commands.rs`](https://github.com/unarbos/agcli/blob/main/src/cli/commands.rs) lines ~651–655.

**No on-chain interaction** for `convert` (TAO/RAO path), `completions`, or `update`. The `utils latency` and `utils convert --tao/--alpha` paths make read-only RPC calls.

## Related Commands

- [`agcli doctor`](doctor.md) — Full connectivity / wallet smoke panel
- `agcli explain` — Built-in concept reference
- `agcli config show` — Current configuration
