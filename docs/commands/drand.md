# drand — Drand randomness beacon

This group targets the `Drand` pallet in `subtensor/pallets/drand/src/lib.rs`.

## CLI surface (current)

`DrandCommands` currently exposes one subcommand:

- `write-pulse`

No `agcli drand` subcommands currently exist for:

- `Drand::set_beacon_config(...)`
- `Drand::set_oldest_stored_round(...)`
- `Drand::random_at(round)` (runtime helper, not an extrinsic)

---

## `agcli drand write-pulse`

Write a Drand pulse payload/signature pair to chain through `handle_drand` in
`src/cli/network_cmds.rs`.

### Clap flags

| Flag | Type | Required | Notes |
|---|---|---|---|
| `--payload` | `String` | yes | Hex string; `0x` prefix optional. Parsed with `hex::decode`. |
| `--signature` | `String` | yes | Hex string; `0x` prefix optional. Parsed with `hex::decode`. |

### Handler and subxt call

- Handler: `src/cli/network_cmds.rs::handle_drand`
- Client call: `src/chain/extrinsics.rs::Client::drand_write_pulse`
- subxt dynamic target: pallet `Drand`, dispatchable `write_pulse`

Current dynamic argument construction in agcli:

1. `Value::from_bytes(pulses_payload: Vec<u8>)`
2. `Value::from_bytes(signature: Vec<u8>)`

### Runtime dispatchable reference

Pallet call definition (`subtensor/pallets/drand/src/lib.rs`):

- `Drand::write_pulse(origin, pulses_payload: PulsesPayload<Public, BlockNumber>, signature: Option<Signature>)`
- call index: `0`
- origin requirement: `ensure_none(origin)` (unsigned origin only)

### Storage keys touched by `write_pulse`

From `subtensor/pallets/drand/src/lib.rs`:

- `Drand::BeaconConfig` (read)
- `Drand::Pulses` (insert and prune/remove)
- `Drand::LastStoredRound` (read/write)
- `Drand::OldestStoredRound` (read/write)
- `Drand::NextUnsignedAt` (write)

### Events emitted

From `subtensor/pallets/drand/src/lib.rs::Event`:

- `Drand::NewPulse { rounds: Vec<u64> }` when at least one pulse is accepted

### Exit codes

`agcli` exit codes are defined in `src/error.rs`:

- `0`: success
- `1`: generic uncategorized error
- `10`: network error
- `11`: auth/wallet error
- `12`: validation error
- `13`: chain/runtime error
- `14`: I/O error
- `15`: timeout

`write-pulse` commonly maps to:

- `12` for bad `--payload` / `--signature` hex parse errors
- `11` for wallet unlock/load failures
- `13` for runtime dispatch failures (including drand pallet errors)

### Output

Current stdout is plaintext only, even with `--output json`.

Example:

```text
Writing Drand pulse (N bytes payload, M bytes sig)
Drand pulse written. Tx: 0x...
```

JSON schema for current output mode:

```json
{
  "type": "string",
  "description": "Plaintext line-oriented output; drand write-pulse does not emit structured JSON."
}
```

### Example

```bash
agcli drand write-pulse \
  --payload 0x0123abcd \
  --signature 0xdeadbeef
```
