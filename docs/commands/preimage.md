# preimage — call-preimage management

`agcli preimage` exposes chain writes for storing and removing call preimages used by scheduler/governance workflows.

`PreimageCommands` surface in CLI:

- `note`
- `unnote`

Handler: `src/cli/network_cmds.rs::handle_preimage`.

## Command surface and chain mapping

| CLI command | Clap flags (type) | Handler call path | Subxt dynamic dispatch | Runtime pallet call |
|---|---|---|---|---|
| `agcli preimage note` | `--pallet <String>` (required), `--call <String>` (required), `--args <String>` (optional JSON array) | `handle_preimage -> Client::note_preimage` | `tx("Preimage", "note_preimage", [Value::from_bytes(encoded_inner_call)])` | `Preimage::note_preimage(bytes: Vec<u8>)` |
| `agcli preimage unnote` | `--hash <String>` (required 32-byte hex, with/without `0x`) | `handle_preimage -> Client::unnote_preimage` | `tx("Preimage", "unnote_preimage", [Value::from_bytes(hash32)])` | `Preimage::unnote_preimage(hash: T::Hash)` |

Runtime registration: `subtensor/runtime/src/lib.rs` (`Preimage: pallet_preimage = 14`).

## Subcommands

### `agcli preimage note`

Store an encoded runtime call preimage.

```bash
agcli preimage note \
  --pallet System \
  --call remark \
  --args '["0x68656c6c6f"]'
```

#### Flag schema

| Flag | Type | Required | Notes |
|---|---|---|---|
| `--pallet` | `String` | yes | Runtime pallet name of the inner call. |
| `--call` | `String` | yes | Dispatchable name in that pallet. |
| `--args` | `String` | no | Must parse as a JSON array; converted element-by-element into `subxt::dynamic::Value`. |

#### SCALE encoding path

1. Build inner dynamic call from `--pallet`, `--call`, parsed args.
2. Encode inner call bytes via `self.inner.tx().call_data(&inner_call)`.
3. Submit `Preimage::note_preimage(bytes)` with those encoded bytes.
4. CLI computes and prints `blake2_256(inner_call_bytes)` as the preimage hash.

#### Storage keys and events

- Storage keys:
  - `Preimage.RequestStatusFor` (insert/update)
  - `Preimage.PreimageFor` (insert)
- Emitted event:
  - `Preimage::Noted { hash }`

### `agcli preimage unnote`

Remove a previously noted preimage by hash.

```bash
agcli preimage unnote \
  --hash 0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

#### Flag schema

| Flag | Type | Required | Notes |
|---|---|---|---|
| `--hash` | `String` | yes | Must decode to exactly 32 bytes (64 hex chars), `0x` optional. |

#### SCALE encoding path

1. Validate hex string and decode into `[u8; 32]`.
2. Submit `Preimage::unnote_preimage(hash)`.

#### Storage keys and events

- Storage keys:
  - `Preimage.RequestStatusFor` (update/remove)
  - `Preimage.PreimageFor` (remove when bytes are cleared)
- Emitted event:
  - `Preimage::Cleared { hash }` when preimage bytes are actually removed.

## In-scope pallet dispatchables not currently exposed by `agcli preimage`

The runtime pallet also includes:

- `Preimage::request_preimage(hash)`
- `Preimage::unrequest_preimage(hash)`

No `PreimageCommands` variant currently maps to these dispatchables.

Related pallet behavior for those calls:

- Storage key: `Preimage.RequestStatusFor` (and sometimes `Preimage.PreimageFor` for final clear).
- Events:
  - `request_preimage` can emit `Preimage::Requested { hash }` on first request.
  - `unrequest_preimage` can emit `Preimage::Cleared { hash }` when the final request is removed and owned bytes are cleared.

## Success output and JSON schema

`handle_preimage` prints plain text success messages for both subcommands, even when `--output json` is set.

Typical success text:

```text
Storing preimage for <pallet>.<call>
Preimage stored for <pallet>.<call>.
  Hash: 0x<64-hex>
  Tx: <tx_hash | dry-run>
```

```text
Removing preimage 0x<64-hex>
Preimage 0x<64-hex> removed.
  Tx: <tx_hash | dry-run>
```

In `--dry-run`, the shared submit path also prints a JSON preview object:

```json
{
  "dry_run": true,
  "signer": "<ss58>",
  "call_data_hex": "0x...",
  "call_data_len": 123
}
```

On error (including `--output json` / `--batch`), top-level CLI emits:

```json
{
  "error": true,
  "code": 12,
  "message": "human-readable error chain",
  "hint": "optional remediation hint"
}
```

## Exit codes

Exit code classification is centralized in `src/error.rs`.

| Code | Meaning |
|---|---|
| `0` | Success |
| `2` | Clap argument/usage error |
| `10` | Network / RPC connectivity failure |
| `11` | Auth / wallet unlock failure |
| `12` | Validation failure (`--hash`, JSON args, pallet/call names, parse errors) |
| `13` | Chain dispatch/extrinsic failure |
| `15` | Timeout |
| `1` | Uncategorized fallback |

## Related commands

- `agcli scheduler schedule` / `schedule-named` (submits calls that commonly depend on preimages)
- `agcli batch` (multi-call submission alternative when delayed dispatch is not required)
