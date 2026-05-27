# utils — Utility Commands

Miscellaneous tools: unit conversion and latency benchmarking.

> **Note:** `agcli doctor`, `agcli completions`, and `agcli update` are **not** subcommands
> of `utils`.  See [doctor.md](doctor.md) and the top-level help for those.

---

## Subcommands

| Subcommand | Purpose | Chain required |
|---|---|---|
| `utils convert` | Convert between RAO↔TAO or TAO↔Alpha | Only for `--tao` / `--alpha` |
| `utils latency` | Benchmark RPC endpoint latency | Yes (WebSocket connect + ping) |

---

## utils convert

Convert between denomination pairs.  Three modes, resolved in priority order:

1. **`--tao <F>`** — simulate swapping TAO into Alpha on subnet `--netuid` (reads chain).
2. **`--alpha <F>`** — simulate swapping Alpha into TAO on subnet `--netuid` (reads chain).
3. **`--amount <F>` / `--to-rao`** — pure arithmetic RAO↔TAO; no chain connection.

### Flags

| Flag | Type | Default | Description |
|---|---|---|---|
| `--amount <F>` | `f64` (optional) | `None` (treated as `0.0`) | Amount to convert. In default mode (RAO→TAO) this is a RAO value; with `--to-rao` it is a TAO value. |
| `--to-rao` | `bool` (flag) | `false` | Convert TAO→RAO instead of the default RAO→TAO. |
| `--tao <F>` | `f64` (optional) | `None` | TAO amount to simulate swapping to Alpha. Requires `--netuid`. |
| `--alpha <F>` | `f64` (optional) | `None` | Alpha amount to simulate swapping to TAO. Requires `--netuid`. |
| `--netuid <U16>` | `u16` (optional) | `None` | Subnet UID required for TAO↔Alpha simulation. |

### Modes and output

#### RAO → TAO (default)

```bash
agcli utils convert --amount 1000000000
# 1000000000 RAO = 1.000000000 TAO
```

JSON (`--output json`):

```json
{ "rao": 1000000000, "tao": 1.0 }
```

Validation: `amount` must be a finite, non-negative number within `u64` range.  Values
outside this range return exit code **1 (GENERIC)** — see [Audit Finding #3](#finding-3).

#### TAO → RAO (`--to-rao`)

```bash
agcli utils convert --amount 1.5 --to-rao
# 1.5 TAO = 1500000000 RAO
```

JSON:

```json
{ "tao": 1.5, "rao": 1500000000 }
```

Internally uses `safe_rao(amount)` which multiplies by 1e9 and saturates to `u64::MAX`
on overflow.  No explicit error is returned for overflow; the result is silently clamped.

#### TAO → Alpha (simulation)

```bash
agcli utils convert --tao 2.5 --netuid 1
# 2.5000 TAO → 2.4812 Alpha (SN1)
```

JSON:

```json
{ "netuid": 1, "tao_in": 2.5, "alpha_out": 2.4812 }
```

Chain call: `SwapRuntimeApi::sim_swap_tao_for_alpha(netuid, tao_rao)` (runtime API,
**read-only**, no extrinsic, no fee).  `tao` is converted to RAO via `safe_rao` before
the call.  Returns the simulated `alpha_amount` from the response; `tao_fee` and
`alpha_fee` are discarded.

#### Alpha → TAO (simulation)

```bash
agcli utils convert --alpha 100.0 --netuid 18
# 100.0000 Alpha (SN18) → 98.7432 TAO
```

JSON:

```json
{ "netuid": 18, "alpha_in": 100.0, "tao_out": 98.7432 }
```

Chain call: `SwapRuntimeApi::sim_swap_alpha_for_tao(netuid, alpha_rao)`.  Same pattern as
above; `tao_amount` is returned, fees are discarded.

### Cross-field constraint

`--tao` and `--alpha` both require `--netuid`.  This is enforced at **runtime**, not by
clap.  Omitting `--netuid` results in an `anyhow` error classified as GENERIC (1), not
VALIDATION (12) — see Audit Finding #3.

### Pallet reference

`utils convert` does **not** submit any extrinsic.  The TAO↔Alpha modes call the
`SwapRuntimeApi` runtime API, which is part of the `swap` pallet runtime API surface
(not a dispatchable).  Storage keys read are internal to the runtime API.

### On-chain events emitted

None.  `utils convert` is purely a read path (simulation or arithmetic).

---

## utils latency

Benchmark WebSocket round-trip latency to one or more RPC endpoints by measuring
`getBlockNumber` call time.

### Flags

| Flag | Type | Default | Description |
|---|---|---|---|
| `--extra <URLS>` | `String` (optional) | `None` | Comma-separated list of additional `ws://` or `wss://` endpoints to test alongside the network default. |
| `--pings <N>` | `usize` | `5` | Number of `getBlockNumber` pings per endpoint. |

### Usage

```bash
# Benchmark the default finney endpoints
agcli utils latency

# Override ping count
agcli utils latency --pings 10

# Test additional custom endpoints
agcli utils latency --extra "ws://127.0.0.1:9944,wss://my-archive.example.com:443"

# Combine
agcli utils latency --pings 3 --extra "ws://127.0.0.1:9944"
```

### Output (human-readable, default)

```
Testing 2 endpoint(s) with 5 pings each...

finney       wss://entrypoint-finney.opentensor.ai:443
  Connect: 142ms | avg: 87ms | min: 82ms | max: 94ms

custom       ws://127.0.0.1:9944
  Connect: 3ms | avg: 2ms | min: 1ms | max: 3ms
```

If all pings for a connected endpoint fail:

```
finney       wss://...
  Connect: 142ms, pings: all 5 failed
```

If the endpoint cannot be connected:

```
custom       ws://unreachable:9944
  FAILED to connect: ...
```

### Output (JSON, `--output json`)

```json
{
  "latency": [
    {
      "label": "finney",
      "url": "wss://entrypoint-finney.opentensor.ai:443",
      "connected": true,
      "avg_ms": 87,
      "min_ms": 82,
      "max_ms": 94,
      "failures": 0
    },
    {
      "label": "custom",
      "url": "ws://127.0.0.1:9944",
      "connected": false,
      "avg_ms": null,
      "min_ms": null,
      "max_ms": null,
      "failures": 5
    }
  ]
}
```

`avg_ms`, `min_ms`, `max_ms` are `u128 | null`.  They are `null` when the endpoint failed
to connect (`connected: false`) or when all pings failed despite a successful connection.

### Human-readable output mixed with JSON

When `--output json`, the "Testing N endpoint(s)…" header line is still printed to
**stdout** before the JSON object.  An agent reading structured output must skip the first
line or use line-based JSON parsing (see Audit Finding #5).

### Pallet reference

`utils latency` does **not** submit any extrinsic.  It calls
`Client::get_block_number()` over the WebSocket RPC connection, which corresponds to the
`chain_getBlockNumber` RPC method — not a storage query and not a dispatchable.

### On-chain events emitted

None.

---

## Exit codes

| Code | Value | Condition |
|---|---|---|
| SUCCESS | 0 | All operations completed without error |
| GENERIC | 1 | `--netuid` missing for TAO↔Alpha, invalid RAO amount, "Chain connection required", no endpoints to test |
| NETWORK | 10 | WebSocket connection failure (endpoint unreachable, DNS error) |
| TIMEOUT | 15 | Connection or ping operation exceeded deadline |

All `utils convert` validation errors (missing `--netuid`, out-of-range amount) currently
produce exit code **1 (GENERIC)** because `error::classify()` does not match their message
strings against the VALIDATION (12) heuristics.

---

## Source

- Handler: `src/cli/system_cmds.rs` — `handle_utils()`
- Dispatch: `src/cli/commands.rs` — `Commands::Utils` match arm
- Chain query helpers: `src/chain/queries.rs` — `sim_swap_tao_for_alpha`, `sim_swap_alpha_for_tao`
- Arithmetic helper: `src/cli/helpers.rs` — `safe_rao()`

---

## Audit Findings

### Finding 1: Docs used wrong flag names

The pre-audit docs showed `agcli utils convert --tao 1.5` meaning "1.5 TAO → 1500000000 RAO"
and `agcli utils convert --rao 1000000000` with a `--rao` flag that does not exist.  The
actual flags are `--amount` (for the numeric value) and `--to-rao` (boolean) for the TAO→RAO
direction.  `--tao` is specifically the TAO→Alpha simulation flag, not a TAO denomination
selector.

### Finding 2: Docs used wrong flag name for latency

Pre-audit docs showed `--count 10`; the actual flag is `--pings <N>` with a default of 5.

### Finding 3: Some convert validation errors exit with GENERIC (1) not VALIDATION (12)

`"--netuid is required for TAO↔Alpha conversion"` and `"Chain connection required"` are
plain `anyhow::bail!` strings that do not match any heuristic in `error::classify()` →
exit code 1 (GENERIC).  By contrast, `"Invalid RAO amount: ... (must be a finite
non-negative number within u64 range)"` does contain `"must be "` and thus correctly
returns VALIDATION (12).  However, the netuid-missing error should also be VALIDATION —
agents cannot distinguish that bad-input case from unexpected runtime failures by exit
code alone.

### Finding 4: safe_rao silently saturates on TAO→RAO overflow

`utils convert --amount 9999999999999.0 --to-rao` feeds `safe_rao(9999999999999.0)` which
saturates to `u64::MAX` without any error or warning.  The output shows a clamped value with
no indication that overflow occurred.

### Finding 5: Latency "Testing N endpoint(s)…" header emitted to stdout before JSON

When `--output json` is set, the line `"Testing 2 endpoint(s) with 5 pings each...\n\n"`
is printed to stdout before the JSON object.  This breaks strict JSON parsers.  The header
should be suppressed (or redirected to stderr) when JSON output is selected.

### Finding 6: Latency connect_ms is printed but not included in JSON output

The connection time (`connect_ms`) is shown in human-readable output but is absent from
the JSON `EndpointResult` struct.  Agents that need to distinguish connection latency from
RPC round-trip latency cannot do so from the JSON output.

---

## Related Commands

- [`agcli doctor`](doctor.md) — Full connectivity / wallet smoke panel
- `agcli explain` — Built-in concept reference
- `agcli config show` — Current configuration
