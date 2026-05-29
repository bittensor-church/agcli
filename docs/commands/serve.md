# serve — Axon / Prometheus Endpoint Registration

Announce serving endpoints on-chain so other neurons can reach your miner or
validator. All five subcommands under `serve` sign and submit extrinsics as the
configured hotkey.

```
agcli serve <SUBCOMMAND>
```

Global flags relevant to `serve`: `--wallet`, `--hotkey-name`, `--password`,
`--yes`, `--network`/`--endpoint`, `--dry-run`.

---

## Subcommands

### serve axon

Register (or update) the axon TCP endpoint for your hotkey on a specific subnet.

```bash
agcli serve axon --netuid <U16> --ip <IPV4> --port <U16> \
  [--protocol <U8>] [--version <U32>]
```

| Flag | Type | Default | Required | Description |
|------|------|---------|----------|-------------|
| `--netuid` | u16 | — | yes | Target subnet UID |
| `--ip` | String (IPv4) | — | yes | IPv4 address (dotted-quad) |
| `--port` | u16 | — | yes | Listening port (1–65535) |
| `--protocol` | u8 | `0` | no | Transport: **0 = TCP**, **1 = UDP** (not IPv4 — `ip_type` is always 4) |
| `--version` | u32 | `0` | no | Axon software version tag |

**Pallet**: `SubtensorModule` · call index **4** · `serve_axon`  
**Dispatch signature**: `(origin, netuid: NetUid, version: u32, ip: u128, port: u16, ip_type: u8, protocol: u8, placeholder1: u8, placeholder2: u8)`  
**Implementation**: `src/cli/network_cmds.rs` → `handle_serve` → `ServeCommands::Axon`; extrinsic at `src/chain/extrinsics.rs::serve_axon` (static generated API).

**Storage written**: `SubtensorModule.Axons[(netuid, hotkey)]` → `AxonInfo { block, version, ip, port, ip_type, protocol, placeholder1, placeholder2 }`

**Event emitted**: `SubtensorModule::AxonServed(netuid, hotkey)`

**Errors** (map to exit code 13 `CHAIN`):

| Pallet error | Trigger |
|---|---|
| `HotKeyNotRegisteredInNetwork` | Hotkey not registered on any subnet |
| `InvalidIpType` | ip_type ≠ 4 or 6 (currently hardcoded to 4) |
| `InvalidIpAddress` | IP numerically invalid for the given ip_type |
| `InvalidPort` | port == 0 |
| `ServingRateLimitExceeded` | Called too soon after the previous serve (see `serving_rate_limit` hyperparameter) |

**Input validation** (before chain submission, exit code 12 `VALIDATION`):
- `validate_ipv4`: rejects `0.0.0.0`, `255.255.255.255`, and non-IPv4 strings.
- `validate_port`: rejects port 0.

**Known audit findings**: see [Findings](#findings) — IPv6 not supported, no JSON output, `placeholder1`/`placeholder2` unexposed.

---

### serve reset

**Not supported.** `serve reset` fails immediately (exit 12) with an explanation — on-chain
`serve_axon` rejects `port=0` (`InvalidPort`). There is no extrinsic to clear axon storage.

Workaround: serve a non-routable endpoint, or stop using the neuron without clearing on-chain metadata.

```bash
# This command intentionally errors — do not use for clearing axons
agcli serve reset --netuid <U16>
```

---

### serve batch-axon

Submit `serve_axon` for multiple subnets in one CLI invocation from a JSON file.
The entire JSON is validated before any wallet unlock or chain submission.

```bash
agcli serve batch-axon --file <PATH>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file` | String (path) | yes | Path to JSON array file |

**File format**:
```json
[
  {"netuid": 1, "ip": "1.2.3.4", "port": 8091},
  {"netuid": 2, "ip": "10.0.0.1", "port": 8092, "protocol": 0, "version": 720}
]
```

Required per entry: `netuid` (u16), `ip` (IPv4 string), `port` (u16 ≥ 1).  
Optional per entry: `protocol` (u8, default **0** = TCP, 1 = UDP), `version` (u32, default 0).  
An empty JSON array `[]` is rejected by the validator.

**Pallet**: `SubtensorModule` · call index **4** · `serve_axon` — one transaction per entry, signed with the same hotkey.  
**Implementation**: `src/cli/network_cmds.rs` → `handle_serve` → `ServeCommands::BatchAxon`; validation at `src/cli/helpers.rs::validate_batch_axon_json`.

**Output**: one line per entry: `[N] SN<netuid> <ip>:<port> — Tx: <hash>` followed by a completion summary. No JSON output available.

---

### serve prometheus

Register (or update) the Prometheus metrics endpoint for your hotkey on a subnet.

```bash
agcli serve prometheus --netuid <U16> --ip <IPV4> --port <U16> [--version <U32>]
```

| Flag | Type | Default | Required | Description |
|------|------|---------|----------|-------------|
| `--netuid` | u16 | — | **yes** | Target subnet UID |
| `--ip` | String (IPv4) | — | yes | IPv4 address |
| `--port` | u16 | — | yes | Listening port (1–65535) |
| `--version` | u32 | `0` | no | Prometheus version tag |

**Pallet**: `SubtensorModule` · call index **5** · `serve_prometheus`  
**Dispatch signature**: `(origin, netuid: NetUid, version: u32, ip: u128, port: u16, ip_type: u8)`  
**Implementation**: `src/cli/network_cmds.rs` → `handle_serve` → `ServeCommands::Prometheus`; extrinsic at `src/chain/extrinsics.rs::serve_prometheus` (dynamic raw call with `Value::u128` per argument).

**Storage written**: `SubtensorModule.Prometheus[(netuid, hotkey)]` → `PrometheusInfo { block, version, ip, port, ip_type }`

**Event emitted**: `SubtensorModule::PrometheusServed(netuid, hotkey)`

**Errors** (exit code 13 `CHAIN`):

| Pallet error | Trigger |
|---|---|
| `HotKeyNotRegisteredInNetwork` | Hotkey not registered on any network |
| `InvalidIpType` | ip_type ≠ 4 or 6 (currently hardcoded to 4) |
| `InvalidIpAddress` | IP numerically invalid |
| `InvalidPort` | port == 0 |
| `ServingRateLimitExceeded` | Rate limit not yet elapsed |

**Known audit finding**: The `docs/commands/serve.md` example (before this update)
omitted the required `--netuid` flag, e.g. `agcli serve prometheus --ip 1.2.3.4
--port 9090`. The CLI _requires_ `--netuid`; omitting it causes a parse error.

---

### serve axon-tls

Register an axon endpoint with an associated TLS certificate (stored as a
`NeuronCertificate` on-chain). The certificate enables mutual TLS between neurons.

```bash
agcli serve axon-tls --netuid <U16> --ip <IPV4> --port <U16> --cert <PATH> \
  [--protocol <U8>] [--version <U32>]
```

| Flag | Type | Default | Required | Description |
|------|------|---------|----------|-------------|
| `--netuid` | u16 | — | yes | Target subnet UID |
| `--ip` | String (IPv4) | — | yes | IPv4 address |
| `--port` | u16 | — | yes | Listening port (1–65535) |
| `--cert` | String (path) | — | yes | Path to certificate file |
| `--protocol` | u8 | `4` | no | Protocol identifier |
| `--version` | u32 | `0` | no | Axon version tag |

**Pallet**: `SubtensorModule` · call index **40** · `serve_axon_tls`  
**Dispatch signature**: `(origin, netuid: NetUid, version: u32, ip: u128, port: u16, ip_type: u8, protocol: u8, placeholder1: u8, placeholder2: u8, certificate: Vec<u8>)`  
**Implementation**: `src/cli/network_cmds.rs` → `handle_serve` → `ServeCommands::AxonTls`; extrinsic at `src/chain/extrinsics.rs::serve_axon_tls` (dynamic raw call).

**Storage written**: Same `Axons` entry as `serve_axon`. Additionally, if the
certificate parses as a valid `NeuronCertificate`, it is written to
`SubtensorModule.NeuronCertificates[(netuid, hotkey)]`.

**Event emitted**: `SubtensorModule::AxonServed(netuid, hotkey)` (same event as plain axon).

**Known audit finding (critical)**: The `--cert` flag reads any file as raw bytes
and passes them verbatim to the chain as `certificate: Vec<u8>`. The pallet's
`NeuronCertificate::try_from(Vec<u8>)` expects **at most 65 bytes**: byte 0 is
the algorithm identifier, bytes 1–64 are the raw public key. A standard PEM or
DER TLS certificate file is hundreds to thousands of bytes and will be silently
dropped (the pallet skips insertion on decode failure). The `--cert` documentation
says "DER or PEM" which is misleading. Users must supply a 65-byte blob in the
format `[algorithm_byte][public_key_bytes...]`.

---

## Exit codes

| Code | Constant | Trigger |
|------|----------|---------|
| 0 | — | Success |
| 12 | `VALIDATION` | Invalid IP, port 0, invalid file path, batch JSON parse error |
| 13 | `CHAIN` | Pallet dispatch error (rate-limited, not registered, bad port, etc.) |
| 11 | `AUTH` | Wallet locked, wrong password, missing hotkey file |
| 14 | `IO` | Cannot read `--cert` or `--file` |
| 10 | `NETWORK` | WebSocket connection failure |

Source: `src/error.rs::classify`.

---

## Output format

All `serve` subcommands print human-readable text to stdout only. **No JSON or
CSV output is available**; the `--output json/csv` global flag is silently ignored
for all serve subcommands. The final line before exit always contains the transaction
hash, e.g.:

```
Axon served on SN1: 1.2.3.4:8091 (proto=4, ver=0).
  Tx: 0x3a4b…
```

---

## Pallet references

- **Dispatch implementations**: `subtensor/pallets/subtensor/src/subnets/serving.rs`
  — `do_serve_axon`, `do_serve_prometheus`
- **Dispatch entry points**: `subtensor/pallets/subtensor/src/macros/dispatches.rs`
  — `serve_axon` (index 4), `serve_axon_tls` (index 40), `serve_prometheus` (index 5)
- **Storage types**: `subtensor/pallets/subtensor/src/lib.rs` — `AxonInfo`,
  `PrometheusInfo`, `NeuronCertificate`
- **Rate-limit helper**: `serving_rate_limit` hyperparameter; query with
  `agcli subnet hyperparams --netuid <N>`

---

## Related commands

- `agcli subnet metagraph --netuid N --full` — show axon endpoints for all neurons
- `agcli subnet probe --netuid N` — test axon connectivity
- `agcli explain --topic axon` — what axons are

---

## Findings

1. **`serve reset` always fails at chain level** — `ServeCommands::Reset` submits
   `serve_axon` with `port: 0`. The pallet's `validate_axon_data` returns
   `InvalidPort` for any zero port. The command cannot succeed on any live subnet.

2. **`serve prometheus` docs missing `--netuid`** — The previous doc example
   `agcli serve prometheus --ip 1.2.3.4 --port 9090` omitted the required
   `--netuid` flag. The CLI rejects this with a parse error.

3. **`serve axon-tls` cert format undocumented** — `--cert` label says "DER or
   PEM" but the pallet silently discards any cert > 65 bytes. The actual accepted
   format is a 65-byte raw blob: 1 byte algorithm + up to 64 bytes public key.
   Standard TLS certs will be silently dropped with no error returned to the user.

4. **IPv6 not surfaced** — All serve subcommands hardcode `ip_type: 4` in
   `AxonInfo` and pass `ip_type: 4` (or `4`) to dynamic calls. There is no
   `--ip-type` flag. IPv6 axon/prometheus endpoints cannot be registered via agcli.

5. **`--output json` silently ignored** — `handle_serve` prints via `println!`
   without consulting `ctx.output`. Agent consumers relying on `--output json`
   receive unparseable human-readable text.

6. **`serve_prometheus` and `serve_axon_tls` use dynamic raw calls** — `serve_axon`
   uses the generated static API (`api::tx().subtensor_module().serve_axon(...)`),
   but `serve_prometheus` and `serve_axon_tls` use `submit_raw_call` with
   `Value::u128` for all integer arguments regardless of the actual SCALE type
   (`u8`, `u16`, `u32`, `NetUid`). Subxt's metadata-guided encoder handles the
   coercion correctly, but this is fragile and inconsistent with the `serve_axon`
   approach; a metadata mismatch would produce silent incorrect encoding.

7. **`serve batch-axon` rejects empty array** — `validate_batch_axon_json` bails
   on `[]`. The handler would print "Batch serving 0 axon updates" for zero
   entries, but a JSON file with an empty array fails before reaching the handler.
   This is arguably correct behavior but is undocumented.

8. **Handler line numbers in old docs were wrong** — The previous version cited
   `handle_serve()` at L430; the actual location is L1153 in `network_cmds.rs`.

9. **`placeholder1` / `placeholder2` not exposed** — The pallet accepts non-zero
   `u8` values for both placeholders (reserved for future use). The CLI hardcodes
   both to `0` with no flag to override.

10. **No `serve show` / `serve status` command** — There is no read-path subcommand
    to inspect your own currently-registered axon or Prometheus endpoint. Users must
    resort to `agcli subnet metagraph --netuid N --full` and visually find their
    entry.

---

## Suggested follow-ups

- **Fix `serve reset`** — implement a dedicated dispatchable or workaround (e.g.
  serve a loopback `127.0.0.1:1` placeholder) since the pallet has no `clear_axon`
  dispatchable. Alternatively document that "reset" is not possible and remove the
  subcommand.
- **Add `--ip-type` flag** — to support IPv6 addresses across `axon`, `batch-axon`,
  `prometheus`, and `axon-tls`.
- **Add JSON output** — route the final serve result (tx hash, confirmed endpoint)
  through `ctx.output` and serialize as JSON for agent consumers.
- **Fix `--cert` documentation** — document the 65-byte binary format requirement
  and add a validation step that warns or errors before submission if the cert file
  is too large.
- **Add `serve show` read subcommand** — query `SubtensorModule.Axons` and
  `SubtensorModule.Prometheus` for the current hotkey and print the registered
  endpoint.
- **Migrate `serve_prometheus` / `serve_axon_tls` to static API** — for consistency
  with `serve_axon` and to eliminate the `Value::u128` type coercion risk.
