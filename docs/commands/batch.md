# batch — Batch Extrinsic Submission

Submit multiple extrinsics from a JSON file via the Substrate `Utility` pallet.
The default mode is atomic (`batch_all`): all calls succeed or all revert.

## Usage

```bash
agcli [global-flags] batch --file <PATH> [--no-atomic] [--force]
```

## Flags

| Flag | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `--file <PATH>` | `String` | **yes** | — | Path to JSON file containing the array of calls |
| `--no-atomic` | `bool` (flag) | no | `false` | Use `Utility::batch` (non-atomic) instead of `batch_all` |
| `--force` | `bool` (flag) | no | `false` | Use `Utility::force_batch` (never reverts) instead of `batch_all` |

Global flags that affect this command:

| Flag | Description |
|------|-------------|
| `--yes` / `-y` | Skip wallet-unlock confirmation prompts |
| `--output json` | Emit `{"tx_hash": "<hash>"}` to stdout instead of the default table line |
| `--batch` | Treat all missing args as hard errors (no interactive prompts) |
| `--mev` | Encrypt the extrinsic through the MEV shield before submission |
| `--network <URL>` | Override the RPC endpoint (default: Finney) |

## JSON File Format

```json
[
  {
    "pallet": "SubtensorModule",
    "call": "add_stake",
    "args": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1, 1000000000]
  },
  {
    "pallet": "Balances",
    "call": "transfer_allow_death",
    "args": ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 5000000000]
  },
  {
    "pallet": "SubtensorModule",
    "call": "set_weights",
    "args": [1, [0, 1], [100, 200], 0]
  }
]
```

**Required fields per call object:**

| Field | Type | Description |
|-------|------|-------------|
| `"pallet"` | string | Pallet name as registered in the runtime (e.g. `"SubtensorModule"`, `"Balances"`, `"System"`) |
| `"call"` | string | Dispatchable name in snake_case (e.g. `"add_stake"`, `"transfer_allow_death"`) |
| `"args"` | array | Positional arguments; use `[]` for no arguments |

**Argument type mapping:**

| JSON type | SCALE encoding |
|-----------|----------------|
| `number` (integer) | `u128` or `i128` depending on sign |
| `number` (float) | stringified (use integers for on-chain amounts) |
| `string` | `str` (SS58 addresses are accepted) |
| `"0x..."` string | raw bytes (hex-decoded) |
| `bool` | `bool` |
| `array` | unnamed composite sequence |

**Limits:**

- Maximum 1000 calls per batch file (enforced client-side by `validate_batch_file`).
- No minimum beyond 1 call (empty array is rejected).
- Spending limits configured in `~/.config/agcli/config.toml` are enforced per-call
  for `SubtensorModule.add_stake`, `remove_stake`, `move_stake`, `swap_stake`,
  `transfer_stake`, `add_stake_limit`, `remove_stake_limit`.

## Batch Modes

| Flag | Utility dispatchable | Behavior |
|------|---------------------|----------|
| *(none — default)* | `Utility::batch_all` | **Atomic**: all calls succeed or all revert |
| `--no-atomic` | `Utility::batch` | **Non-atomic**: stops on first failure; prior calls are not reverted |
| `--force` | `Utility::force_batch` | **Force**: continues past individual call failures; never reverts successful calls |

When both `--no-atomic` and `--force` are supplied clap accepts them without error.
The handler resolves the conflict with `force > no_atomic > default`; `--no-atomic` is
silently ignored when `--force` is also set.

## On-chain Pallet Reference

Pallet: **`Utility`** (`subtensor/pallets/utility`)

| Dispatchable | agcli surface | Notes |
|-------------|---------------|-------|
| `batch(calls)` | `--no-atomic` flag | Non-atomic, halts on first error |
| `batch_all(calls)` | default | Atomic, reverts on error |
| `force_batch(calls)` | `--force` flag | Non-atomic, continues on error |
| `as_derivative(index, call)` | **none** | No agcli surface |
| `dispatch_as(as_origin, call)` | **none** | No agcli surface |
| `with_weight(call, weight)` | **none** | No agcli surface |
| `if_else(main, fallback)` | **none** | No agcli surface |

Inner calls are encoded via `subxt::dynamic::tx(pallet, call_name, args)` →
`client.tx().call_data(…)` (SCALE-encoded `RuntimeCall` bytes), then wrapped as
`Value::from_bytes(…)` entries in the outer `Utility::{batch,batch_all,force_batch}`
payload.

## Storage Keys

The `Utility` pallet does not write any storage keys itself; all state changes are
produced by the inner calls.

## On-chain Events Emitted

After a successful submission the `Utility` pallet emits:

| Event | When |
|-------|------|
| `Utility::BatchCompleted` | All inner calls succeeded (`batch_all` / `force_batch` with no failures) |
| `Utility::BatchCompletedWithErrors` | Some calls failed in `force_batch` |
| `Utility::BatchInterrupted { index, error }` | First failing call in `batch` (non-atomic) |
| `Utility::ItemCompleted` | Emitted once per successful inner call |
| `Utility::ItemFailed { error }` | Emitted once per failing call in `force_batch` |

Inner calls emit their own pallet events normally.

## Output

**Text mode (default):**

```
Batch (3 calls) submitted. Tx: 0xabcd...
```

(Diagnostic `#N: Pallet.call (N bytes)` lines are emitted to stderr before submission.)

**JSON mode (`--output json`):**

```json
{"tx_hash": "0xabcd..."}
```

Note: the JSON output does not include `calls_count`, `mode`, or `mev_shielded`.
These are written to stderr only.

## Exit Codes

| Code | Value | Trigger condition |
|------|-------|------------------|
| `IO` | 14 | Batch file not found or unreadable (`std::io::ErrorKind::NotFound`) |
| `VALIDATION` | 12 | Malformed JSON in the batch file (`serde_json::Error` in the error chain) |
| `VALIDATION` | 12 | Invalid call argument (bad SS58 address, missing field, wrong type) |
| `CHAIN` | 13 | Extrinsic rejected by the runtime (e.g. `Utility::TooManyCalls`, insufficient balance) |
| `AUTH` | 11 | Wallet locked, wrong password, or missing hotkey |
| `NETWORK` | 10 | RPC endpoint unreachable / WebSocket error |
| `TIMEOUT` | 15 | Block finalization exceeded the deadline |
| `GENERIC` | 1 | Validation bail! messages from `validate_batch_file` that do not match classify() patterns (e.g. "too many calls", "empty", "missing field" text messages) — see Findings |

## Examples

Submit two operations atomically:

```bash
agcli --yes batch --file ops.json
```

Non-atomic — continue past failures:

```bash
agcli --yes batch --file ops.json --no-atomic
```

Force batch — never reverts:

```bash
agcli --yes batch --file ops.json --force
```

Emit JSON for programmatic use:

```bash
agcli --output json batch --file ops.json | jq .tx_hash
```

## Related Commands

- `agcli scheduler schedule` — Schedule calls for future blocks
- `agcli proxy` — Proxy calls through a proxy account
- `agcli preimage` — Pre-image a large call for scheduling

## Audit Findings (batch group)

1. **`as_derivative`, `dispatch_as`, `with_weight`, `if_else` have no agcli surface.**
   Four of the seven `Utility` pallet dispatchables are not exposed. Agents that
   need derivative-account dispatch or weight-capped calls must resort to raw JSON
   workarounds.

2. **Both `--no-atomic` and `--force` are accepted simultaneously; `--no-atomic` is silently discarded.**
   Clap declares no `conflicts_with` between the two flags. When both are set, `force`
   wins silently. This is undocumented and may confuse callers.

3. **`validate_batch_file` error messages classify as GENERIC (exit 1), not VALIDATION (12).**
   Client-side validation failures ("is empty", "has too many calls", "missing pallet
   field", etc.) are plain `anyhow::bail!` strings. `error::classify()` does not
   recognise them; they all exit 1 instead of 12, making it impossible for an agent to
   distinguish input-format errors from unexpected failures.

4. **JSON output is too sparse: only `tx_hash` is emitted.**
   Agents cannot determine which mode was used, how many calls were submitted, or
   whether MEV shielding was applied from the JSON output alone.

5. **Diagnostic `eprintln!` calls always go to stderr regardless of `--output json`.**
   Lines like `"Batch: 3 calls, mode=batch_all (atomic)"` and per-call encoding reports
   are emitted to stderr unconditionally. This is correct behaviour (diagnostics on
   stderr) but agents capturing combined output may be surprised.

6. **No stdin (`-`) support for the batch file.**
   `--file` requires a filesystem path. There is no way to pipe a batch JSON via stdin,
   unlike many UNIX tools.
