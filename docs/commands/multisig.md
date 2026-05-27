# multisig — Multisig Operations

Create and manage multi-signature (M-of-N) transactions on Substrate's `pallet-multisig` (pallet index 13 in the subtensor runtime).

## Subcommands

| Subcommand | Pallet dispatchable | Summary |
|---|---|---|
| `address` | _(local computation)_ | Derive deterministic multisig SS58 address |
| `submit` | `Multisig::approve_as_multi` | First signatory proposes + registers first approval |
| `approve` | `Multisig::approve_as_multi` | Subsequent signatory registers approval by call hash |
| `execute` | `Multisig::as_multi` | Final signatory executes the underlying call |
| `cancel` | `Multisig::cancel_as_multi` | Originating signatory cancels a pending operation |
| `list` | `Multisig::Multisigs` (storage query) | List pending operations for a multisig account |

---

### `multisig address`

Derives a deterministic multisig account address from a set of signatories and a threshold, using Substrate's `multi_account_id` algorithm:
`blake2_256( SCALE_encode( b"modlpy/utilisuba" ++ compact(n) ++ sorted_account_ids ++ threshold_le16 ) )`

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--signatories` | `String` | yes | Comma-separated SS58 addresses of all signatories (≥ 2) |
| `--threshold` | `u16` | yes | Minimum number of approvals required to execute |

**Output** (stdout, plain text)

```
Multisig address: 5XYZ...
  Threshold: 2/3
  Signatories:
    5Alice...
    5Bob...
    5Charlie...
```

No JSON output mode for this subcommand.

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success |
| 12 (VALIDATION) | Fewer than 2 signatories, or an address fails SS58 decode |

**Example**

```bash
agcli multisig address \
  --signatories "5GrwvaEF...,5FHneW46..." \
  --threshold 2
```

---

### `multisig submit`

Proposes a new multisig call and registers the first approval. Internally calls `Multisig::approve_as_multi` with `maybe_timepoint = None` and `max_weight = {ref_time: 0, proof_size: 0}`.

> **Note (Audit):** `submit` maps to `approve_as_multi`, not `as_multi`, despite the variant docstring saying "Submit a multisig call (as_multi)". If `threshold == 1`, `submit` will NOT execute the call — use `execute` instead.

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--others` | `String` | yes | Comma-separated SS58 addresses of the other signatories (excluding yourself) |
| `--threshold` | `u16` | yes | Approval threshold |
| `--pallet` | `String` | yes | Pallet name for the inner call (e.g. `SubtensorModule`, `Balances`) |
| `--call` | `String` | yes | Dispatchable name within the pallet |
| `--args` | `String` | no | Call arguments as a JSON array (e.g. `'[arg1, arg2]'`) |
| `--dry-run` | flag | no | Print the transaction without submitting |

**SCALE encoding of inner call**

`submit` encodes the inner call via `subxt::dynamic::tx(pallet, call, fields)` and passes its `call_data()` through `blake2_256` to obtain the 32-byte call hash used in `approve_as_multi`.

**Output** (stdout, plain text)

```
Submitting multisig call: Balances.transfer_keep_alive (threshold 2/3)
Multisig call submitted: Balances.transfer_keep_alive (threshold 2/3).
  Tx: 0xabc123...
```

No JSON output mode for this subcommand.

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success |
| 11 (AUTH) | Wallet not found, wrong password |
| 12 (VALIDATION) | Invalid JSON args, SS58 decode failure, or spending-limit exceeded |
| 13 (CHAIN) | Extrinsic rejected (e.g. `AlreadyApproved`, `TooFewSignatories`) |
| 10 (NETWORK) | Could not connect to node |

**Pallet ref**: `Multisig::approve_as_multi(threshold, other_signatories, maybe_timepoint, call_hash, max_weight)`

**Events emitted**: `MultisigApproval { approving, timepoint, multisig, call_hash }`

**Example**

```bash
agcli --wallet alice multisig submit \
  --others "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" \
  --threshold 2 \
  --pallet SubtensorModule \
  --call add_stake \
  --args '[5, "5HotKey...", 1000000000]'
```

---

### `multisig approve`

Registers an approval for an existing multisig call identified by its 32-byte call hash. Uses `Multisig::approve_as_multi` with `maybe_timepoint = None` and `max_weight = {ref_time: 0, proof_size: 0}`.

> **Audit finding — missing timepoint:** The pallet requires `maybe_timepoint: Some(Timepoint)` for any approval after the first. Passing `None` causes the runtime to return `NoTimepoint` for non-first approvals. `approve` does not accept `--timepoint-height` / `--timepoint-index` flags, so there is currently no way to perform a second+ approval without using `execute`.

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--others` | `String` | yes | Comma-separated SS58 of the other signatories (excluding yourself) |
| `--threshold` | `u16` | yes | Approval threshold |
| `--call-hash` | `String` | yes | 0x-prefixed hex string of the 32-byte call hash |
| `--dry-run` | flag | no | Print without submitting |

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success |
| 11 (AUTH) | Wallet error |
| 12 (VALIDATION) | Bad call hash (not 32 bytes or non-hex) |
| 13 (CHAIN) | `AlreadyApproved`, `NoTimepoint` (see audit note above), `NotFound` |
| 10 (NETWORK) | Connection failure |

**Pallet ref**: `Multisig::approve_as_multi(threshold, other_signatories, maybe_timepoint, call_hash, max_weight)`

**Storage key**: `Multisig::Multisigs[(account_id, call_hash)]`

**Events emitted**: `MultisigApproval { approving, timepoint, multisig, call_hash }`

**Example**

```bash
agcli --wallet bob multisig approve \
  --others "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
  --threshold 2 \
  --call-hash 0xdeadbeef...
```

---

### `multisig execute`

Executes the underlying call as the final signatory. Calls `Multisig::as_multi` with `maybe_timepoint` (required if a prior approval exists) and a generous `max_weight` default (`ref_time: 10_000_000_000`, `proof_size: 1_048_576`).

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--others` | `String` | yes | Comma-separated SS58 of the other signatories |
| `--threshold` | `u16` | yes | Approval threshold |
| `--pallet` | `String` | yes | Pallet name for the inner call |
| `--call` | `String` | yes | Dispatchable name |
| `--args` | `String` | no | Call arguments as JSON array |
| `--timepoint-height` | `u32` | no† | Block height of the initial approval (from `multisig list`) |
| `--timepoint-index` | `u32` | no† | Extrinsic index of the initial approval |
| `--dry-run` | flag | no | Print without submitting |

† Both timepoint flags must be given together, or both omitted. If prior approvals exist on-chain the pallet requires a valid timepoint — omitting it returns `UnexpectedTimepoint`.

**SCALE encoding**: the inner call is re-encoded from `--pallet`/`--call`/`--args` and passed as raw bytes to `Multisig::as_multi`. The call hash must match what was registered during `submit`.

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success |
| 11 (AUTH) | Wallet error |
| 12 (VALIDATION) | Invalid JSON, bad addresses, mismatched timepoint flags |
| 13 (CHAIN) | `WrongTimepoint`, `UnexpectedTimepoint`, `MaxWeightTooLow`, `MinimumThreshold` |
| 10 (NETWORK) | Connection failure |

**Pallet ref**: `Multisig::as_multi(threshold, other_signatories, maybe_timepoint, call, max_weight)`

**Storage key**: `Multisig::Multisigs[(account_id, call_hash)]` — entry is removed on successful execution.

**Events emitted**:
- `MultisigApproval { approving, timepoint, multisig, call_hash }` — if threshold not yet reached
- `MultisigExecuted { approving, timepoint, multisig, call_hash, result }` — if threshold is reached and call executes

**Example**

```bash
agcli --wallet charlie multisig execute \
  --others "5GrwvaEF...,5FHneW46..." \
  --threshold 2 \
  --pallet SubtensorModule \
  --call add_stake \
  --args '[5, "5HotKey...", 1000000000]' \
  --timepoint-height 123456 \
  --timepoint-index 0
```

---

### `multisig cancel`

Cancels a pending multisig operation. Only the originating signatory (who called `submit`) can cancel. Calls `Multisig::cancel_as_multi`. Timepoint is **required**.

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--others` | `String` | yes | Comma-separated SS58 of the other signatories |
| `--threshold` | `u16` | yes | Approval threshold |
| `--call-hash` | `String` | yes | 0x-prefixed hex of the 32-byte call hash |
| `--timepoint-height` | `u32` | yes | Block height of the original submission |
| `--timepoint-index` | `u32` | yes | Extrinsic index of the original submission |
| `--dry-run` | flag | no | Print without submitting |

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success |
| 11 (AUTH) | Wallet error |
| 12 (VALIDATION) | Invalid call hash format |
| 13 (CHAIN) | `NotOwner` (only proposer can cancel), `WrongTimepoint`, `NotFound` |
| 10 (NETWORK) | Connection failure |

**Pallet ref**: `Multisig::cancel_as_multi(threshold, other_signatories, timepoint, call_hash)`

**Storage key**: `Multisig::Multisigs[(account_id, call_hash)]` — entry is removed and deposit is returned.

**Events emitted**: `MultisigCancelled { cancelling, timepoint, multisig, call_hash }`

**Example**

```bash
agcli --wallet alice multisig cancel \
  --others "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" \
  --threshold 2 \
  --call-hash 0xdeadbeef... \
  --timepoint-height 123456 \
  --timepoint-index 0
```

---

### `multisig list`

Queries `Multisig::Multisigs` storage to list all pending operations for a multisig account.

**Flags**

| Flag | Type | Required | Description |
|---|---|---|---|
| `--address` | `String` | yes | SS58 address of the multisig account |

**Output** (stdout, plain text)

```
Pending multisig operations for 5XYZ... (2 found):
  Call hash:  0xabcdef...
  Timepoint: height=123456, index=0
  Approvals: 1
  Deposit:   20080000000 RAO

  Call hash:  0x112233...
  ...
```

> **Audit finding — no JSON mode:** `list` always emits plain text. Unlike most other agcli commands, it has no `--output json` path, making programmatic consumption error-prone.

**Exit codes**

| Code | Meaning |
|---|---|
| 0 | Success (even if list is empty) |
| 12 (VALIDATION) | Invalid SS58 address |
| 13 (CHAIN) | Storage query failure |
| 10 (NETWORK) | Connection failure |

**Storage key**: `Multisig::Multisigs[account_id]` — double-map prefix scan by `account_id`.

**Call hash extraction**: extracted from the last 32 bytes of the raw storage key (the `Blake2_128Concat` double-map appends the raw call hash after its 16-byte hash prefix).

**Example**

```bash
agcli multisig list --address 5MultisigSS58...
```

---

## Full M-of-N Workflow

```
Signatory A                      Signatory B                  On-chain
─────────────────                ─────────────────            ──────────
multisig address                                              [local]
  → derive multisig address

Fund multisig account                                         transfer TAO

multisig submit                                               approve_as_multi(None)
  → call_hash returned                                        → MultisigApproval stored

                                 multisig execute             as_multi(Some(timepoint))
                                   --timepoint-height X       → MultisigExecuted
                                   --timepoint-index Y
```

If there are 3+ signatories, intermediate approvers should use `execute` with `--timepoint-height`/`--timepoint-index` (obtained from `multisig list`) because `approve` cannot pass a timepoint.

---

## Runtime Configuration (subtensor)

| Parameter | Value |
|---|---|
| Pallet index | 13 (`Multisig: pallet_multisig = 13`) |
| `MaxSignatories` | 100 |
| `DepositBase` | `deposit(1, 112)` |
| `DepositFactor` | `deposit(0, 32)` |
| Pallet source | `pallet-multisig` from `github.com/opentensor/polkadot-sdk` rev `7cc54bf2d50ae3921d718736dfeb0de9468539c7` |

---

## Audit Notes

1. **`approve` is broken for non-first approvals.** The handler always passes `maybe_timepoint = None`. The pallet returns `NoTimepoint` for any approval on an already-registered call. There is no `--timepoint-height`/`--timepoint-index` on `approve`. Workaround: use `execute` (which handles the timepoint). Suggested fix: add timepoint flags to `Approve` variant and pass them to `approve_multisig`.

2. **`submit` docstring says `as_multi` but calls `approve_as_multi`.** The clap variant doc reads "Submit a multisig call (as_multi)" but the implementation calls `approve_multisig` which dispatches `approve_as_multi`. For a 1-of-N multisig this means the call is never executed — only registered. Misleading for users building scripts.

3. **`submit` and `approve` pass `max_weight = {ref_time: 0, proof_size: 0}`.** This is safe for hash-only registrations (where execution happens later via `execute`), but would cause `MaxWeightTooLow` if either subcommand were somehow the final approval. The `execute` path uses a generous default (10B ref_time, 1MB proof_size), so this only matters in practice if the workflow is misused.

4. **`list` output is plain text only.** Unlike most agcli commands, there is no JSON output mode for `multisig list`. Agent consumers must screen-scrape the human-readable output. The returned tuple `(call_hash, height, index, approvals, deposit)` maps to a straightforward JSON schema but is never emitted as such.

5. **Call hash extraction in `list_multisig_pending` assumes the last 32 bytes are the call hash.** The storage key for `Multisig::Multisigs` is a `Blake2_128Concat` double map: `prefix(16) + storage_hash(16) + blake2_128(account_id)(16) + account_id(32) + blake2_128(call_hash)(16) + call_hash(32)`. Extracting the last 32 bytes is correct given this layout, but it silently returns `"unknown"` if the key is shorter than 32 bytes, which could happen with a different codec version.

6. **No `multisig as_multi_with_deposit` or deposit-query surface.** The pallet stores a deposit per pending call (`deposit = DepositBase + DepositFactor * (threshold - 1)`), returned on execution or cancellation. There is no agcli command to estimate the deposit before calling `submit`. The `list` output shows the deposit in RAO but does not convert it to TAO.

## Related Commands

- `agcli proxy add` — single-signer delegation (no M-of-N requirement)
- `agcli batch` — bundle multiple calls in one extrinsic (no multisig threshold)
