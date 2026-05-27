# scheduler — Schedule Runtime Calls

Schedule and cancel future runtime calls through the `Scheduler` pallet.

Audited against:

- `src/cli/mod.rs` (`SchedulerCommands`)
- `src/cli/network_cmds.rs` (`handle_scheduler`)
- `src/chain/extrinsics.rs` (`schedule_call`, `schedule_named_call`, `cancel_scheduled`, `cancel_named_scheduled`)
- `subtensor/runtime/src/lib.rs` (`impl pallet_scheduler::Config for Runtime`)
- Upstream scheduler pallet source used by subtensor runtime:
  - `pallet_scheduler::Call::{schedule,cancel,schedule_named,cancel_named,schedule_after,schedule_named_after}`
  - Storage: `Agenda`, `Lookup`, `Retries`, `IncompleteSince`
  - Events: `Scheduled`, `Canceled`, `Dispatched`, `CallUnavailable`, `PeriodicFailed`, `RetryFailed`, `PermanentlyOverweight`, `AgendaIncomplete`

## Subcommands (agcli surface)

`agcli scheduler` exposes 4 subcommands:

1. `schedule`
2. `schedule-named`
3. `cancel`
4. `cancel-named`

## Common scheduler exit codes

`agcli` error classification comes from `src/error.rs`:

- `0`: success
- `10` (`NETWORK`): websocket/connectivity failures
- `11` (`AUTH`): wallet unlock/decryption/key access failures
- `12` (`VALIDATION`): invalid args, malformed JSON args, invalid scheduler id/repeat params
- `13` (`CHAIN`): dispatch/runtime errors (bad origin, not found, pallet error variants)
- `14` (`IO`): wallet/key file I/O failures
- `15` (`TIMEOUT`): finalization timeout

Clap parse/usage failures happen before `src/error.rs` classification.

## `scheduler schedule`

Schedule an anonymous call at an absolute block height.

### Clap flags

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--when` | `u32` | yes | target block |
| `--pallet` | `String` | yes | pallet for inner call |
| `--call` | `String` | yes | call in the inner pallet |
| `--args` | `Option<String>` | no | JSON array for inner call args |
| `--priority` | `u8` | no | default `128`, `0` highest / `255` lowest |
| `--repeat-every` | `Option<u32>` | no | must be paired with `--repeat-count` |
| `--repeat-count` | `Option<u32>` | no | must be paired with `--repeat-every` |

### Chain mapping and SCALE shape

- agcli handler: `network_cmds::handle_scheduler` → `Client::schedule_call`
- subxt dynamic tx: `Scheduler.schedule`
- SCALE argument order:
  1. `when: BlockNumberFor<T>` (agcli parses `u32`, encoded via `Value::u128`)
  2. `maybe_periodic: Option<(BlockNumberFor<T>, u32)>`
  3. `priority: schedule::Priority` (`u8`, encoded via `Value::u128`)
  4. `call: Box<RuntimeCall>` (agcli encodes inner call bytes with `call_data`)

### Pallet reference, storage, events

- Dispatchable: `Scheduler::schedule`
- Main storage touched:
  - writes `Scheduler::Agenda`
  - writes `Scheduler::Lookup` only when named (`schedule_named`), not this call
- Event emitted on successful scheduling:
  - `Scheduler::Scheduled { when, index }`
- Later execution (at target block) can emit:
  - `Dispatched`, `CallUnavailable`, `PeriodicFailed`, `RetryFailed`, `PermanentlyOverweight`, `AgendaIncomplete`

### Output JSON schema

Current implementation prints plain text and does not emit structured JSON from this handler, even under JSON output mode.

```json
{
  "type": "null",
  "description": "No structured scheduler JSON payload is currently emitted by agcli scheduler schedule."
}
```

## `scheduler schedule-named`

Schedule a named call so it can be cancelled by id.

### Clap flags

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--id` | `String` | yes | validated as non-empty and `<= 32` bytes |
| `--when` | `u32` | yes | target block |
| `--pallet` | `String` | yes | pallet for inner call |
| `--call` | `String` | yes | call in the inner pallet |
| `--args` | `Option<String>` | no | JSON array for inner call args |
| `--priority` | `u8` | no | default `128` |
| `--repeat-every` | `Option<u32>` | no | must be paired with `--repeat-count` |
| `--repeat-count` | `Option<u32>` | no | must be paired with `--repeat-every` |

### Chain mapping and SCALE shape

- agcli handler: `network_cmds::handle_scheduler` → `Client::schedule_named_call`
- subxt dynamic tx: `Scheduler.schedule_named`
- SCALE argument order:
  1. `id: TaskName` (`[u8; 32]` in scheduler v3 API)
  2. `when: BlockNumberFor<T>`
  3. `maybe_periodic: Option<(BlockNumberFor<T>, u32)>`
  4. `priority: schedule::Priority`
  5. `call: Box<RuntimeCall>`

agcli currently passes `id.as_bytes()` directly as dynamic bytes.

### Pallet reference, storage, events

- Dispatchable: `Scheduler::schedule_named`
- Main storage touched:
  - writes `Scheduler::Agenda`
  - writes `Scheduler::Lookup` (task name → `(when, index)`)
- Event emitted on successful scheduling:
  - `Scheduler::Scheduled { when, index }`
- Later execution can emit the same execution-path events as `schedule`.

### Output JSON schema

```json
{
  "type": "null",
  "description": "No structured scheduler JSON payload is currently emitted by agcli scheduler schedule-named."
}
```

## `scheduler cancel`

Cancel an anonymous scheduled task by block and index.

### Clap flags

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--when` | `u32` | yes | block where task is queued |
| `--index` | `u32` | yes | agenda index within that block |

### Chain mapping and SCALE shape

- agcli handler: `network_cmds::handle_scheduler` → `Client::cancel_scheduled`
- subxt dynamic tx: `Scheduler.cancel`
- SCALE argument order:
  1. `when: BlockNumberFor<T>`
  2. `index: u32`

### Pallet reference, storage, events

- Dispatchable: `Scheduler::cancel`
- Main storage touched:
  - mutates `Scheduler::Agenda`
  - removes `Scheduler::Retries` for the task if present
  - named path also mutates `Lookup`; anonymous path does not
- Event emitted on successful cancellation:
  - `Scheduler::Canceled { when, index }`

### Output JSON schema

```json
{
  "type": "null",
  "description": "No structured scheduler JSON payload is currently emitted by agcli scheduler cancel."
}
```

## `scheduler cancel-named`

Cancel a named scheduled task by id.

### Clap flags

| Flag | Type | Required | Notes |
|---|---|---:|---|
| `--id` | `String` | yes | validated as non-empty and `<= 32` bytes |

### Chain mapping and SCALE shape

- agcli handler: `network_cmds::handle_scheduler` → `Client::cancel_named_scheduled`
- subxt dynamic tx: `Scheduler.cancel_named`
- SCALE argument order:
  1. `id: TaskName` (`[u8; 32]`)

agcli currently passes `id.as_bytes()` directly as dynamic bytes.

### Pallet reference, storage, events

- Dispatchable: `Scheduler::cancel_named`
- Main storage touched:
  - mutates `Scheduler::Lookup` (remove id mapping)
  - mutates `Scheduler::Agenda`
  - removes `Scheduler::Retries` for the task if present
- Event emitted on successful cancellation:
  - `Scheduler::Canceled { when, index }`

### Output JSON schema

```json
{
  "type": "null",
  "description": "No structured scheduler JSON payload is currently emitted by agcli scheduler cancel-named."
}
```

## Pallet dispatchables in scope not surfaced by `agcli scheduler`

`Scheduler` pallet includes:

- `schedule_after`
- `schedule_named_after`

Neither has a CLI subcommand under `agcli scheduler` today.

`Client::schedule_after` exists in `src/chain/extrinsics.rs`, but no CLI path calls it.
There is no `Client::schedule_named_after` wrapper.

## Related commands

- `agcli preimage note` (store encoded calls for indirection/preimage workflows)
- `agcli batch` (execute immediately instead of scheduling)
