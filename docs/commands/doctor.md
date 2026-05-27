# doctor — diagnostics command

`agcli doctor` is a single-command diagnostic surface (no nested subcommands) that runs local + on-chain health checks and prints a report.

## Command surface

- Top-level dispatch: `Commands::Doctor` in `src/cli/commands.rs`
- Handler: `handle_doctor(...)` in `src/cli/system_cmds.rs`
- Subcommands under `doctor`: **none**

## Usage

```bash
agcli doctor
agcli --network test doctor
agcli --endpoint ws://127.0.0.1:9944 --output json doctor
agcli --wallet-dir ~/.bittensor/wallets --wallet default doctor
```

## Clap flags and types

`doctor` has no local flags; it consumes global `Cli` flags.

### Flags used directly by `handle_doctor`

| Flag | Type | Default | Meaning in doctor |
|---|---|---|---|
| `--network` (`-n`) | `String` | `"finney"` | Chooses network preset, converted to RPC URL list via `resolve_network()` / `Network::ws_urls()`. |
| `--endpoint` | `Option<String>` | `None` | Overrides `--network` and forces a custom RPC endpoint. |
| `--wallet-dir` | `String` | `"~/.bittensor/wallets"` | Base path used for wallet status check. |
| `--wallet` (`-w`) | `String` | `"default"` | Wallet name used for wallet status check. |
| `--output` | `OutputFormat` (`table|json|csv`) | `table` | `json` emits structured payload; all non-JSON formats currently use table-style text. |

### Global flags that can still affect execution

| Flag | Type | Effect |
|---|---|---|
| `--timeout` | `Option<u64>` | Wraps whole command in process-level timeout from `src/main.rs`; timeout errors classify to exit code `15`. |
| `--time` | `bool` | Prints elapsed wall-clock time to stderr in `main`. |
| `--verbose` / `--debug` / `--log-file` | `bool` / `bool` / `Option<String>` | Logging behavior only. |

## Check list and chain mapping (execution order)

| Check row | Chain call path | Pallet / storage / RPC reference | SCALE args | On-chain events emitted by this check |
|---|---|---|---|---|
| `Version` | local constant `env!("CARGO_PKG_VERSION")` | none (local build metadata) | n/a | none |
| `Network` | `network.ws_urls()` | none (local config mapping) | n/a | none |
| `Connection` | `Client::connect_network -> connect_with_retry -> connect_once` | subxt RPC transport connect only; no explicit `system_chain` / `system_version` probe is performed | n/a | none |
| `Block height` | `Client::get_block_number` | best-head query via `inner.blocks().at_latest()` (RPC-backed head lookup) | n/a | none |
| `Subnets` | `Client::get_total_networks` | `api::storage().subtensor_module().total_networks()` → `SubtensorModule::TotalNetworks` (`StorageValue<u16>`) in `subtensor/pallets/subtensor/src/lib.rs` | none (`StorageValue`, keyless fetch) | none (read-only); writes to this storage emit events such as `NetworkAdded` / `NetworkRemoved` in `subtensor/pallets/subtensor/src/macros/events.rs` |
| `Latency (3 pings)` | three `Client::get_block_number` calls | same as `Block height` | n/a | none |
| `Disk cache` | `queries::disk_cache::{list_keys,path}` + local `std::fs::metadata` | none (local filesystem) | n/a | none |
| `Wallet` | `wallet::Wallet::open("{wallet_dir}/{wallet}")`, `coldkey_ss58`, `list_hotkeys` | none (wallet files on disk) | n/a | none |

## Dispatchables in scope

`doctor` submits **no extrinsics**. There is no pallet dispatchable name or call-argument SCALE encoding path in this command.

## Output contract

### Human/table mode (`--output table` or `--output csv`)

Rows are printed as:

```
[STATUS] CHECK_NAME DETAIL
```

where `STATUS` is `OK` or `FAIL`.

### JSON mode (`--output json`)

Schema:

```json
{
  "type": "object",
  "required": ["doctor"],
  "properties": {
    "doctor": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["check", "detail", "ok"],
        "properties": {
          "check": { "type": "string" },
          "detail": { "type": "string" },
          "ok": { "type": "boolean" }
        }
      }
    }
  }
}
```

Example payload:

```json
{
  "doctor": [
    { "check": "Version", "detail": "agcli v0.0.0", "ok": true },
    { "check": "Connection", "detail": "OK (78ms)", "ok": true }
  ]
}
```

## Exit codes

Compared to `src/error.rs`:

| Condition | Exit code |
|---|---|
| Doctor finishes and prints report (even with failed rows) | `0` |
| Clap parse/usage failure (invalid arg shape) | `2` (clap standard) |
| Global timeout reached (`--timeout`) | `15` (`error::exit_code::TIMEOUT`) |
| Internal unexpected error outside normal doctor flow | classified by `src/error.rs` (`1/10/11/12/13/14/15`) |

`handle_doctor` intentionally accumulates failures into report rows instead of bubbling `Err`, so RPC/wallet check failures usually remain in-band (`ok: false`) with process exit `0`.

## Related commands

- `agcli utils latency`
- `agcli balance`
- `agcli config show`
