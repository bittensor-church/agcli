<!-- orchestrate handoff
task: audit-serve
branch: orch/agcli-audit/audit-serve
agentId: bc-76a0b611-305d-468b-ac5a-67061a3557f2
runId: run-5b6d93b3-bbd7-4fc2-baa0-1c8a37e61d23
resultStatus: finished
finishedAt: 2026-05-27T13:26:42.248Z
-->

## Status
success

## Branch
`orch/agcli-audit/audit-serve`

## What I did

- **`tests/audit_serve.rs`** (new, ~380 LOC): 34 parse-surface tests covering all 5 `ServeCommands` subcommands (`Axon`, `Reset`, `BatchAxon`, `Prometheus`, `AxonTls`) plus global-flag interaction tests, field value assertions, and negative tests for missing required args. Includes helper function tests (`validate_ipv4`, `validate_port`, `validate_batch_axon_json`) exercising the pre-chain input validation layer. One `#[ignore]`-gated localnet green-path test that calls `serve_axon` and `serve_prometheus` via the SDK against Alice's key.
- **`docs/commands/serve.md`** (fully rewritten): All 5 subcommands documented with clap flags + types + defaults, exit codes mapped to `src/error.rs` constants, pallet refs + call indices (`serve_axon`=4, `serve_axon_tls`=40, `serve_prometheus`=5), storage keys (`Axons`, `Prometheus`, `NeuronCertificates`), on-chain events (`AxonServed`, `PrometheusServed`), output format notes, and 10 concrete audit findings.

## Measurements

- `cargo check --all-targets`: exit 0 → exit 0
- `cargo test --no-run --test audit_serve`: compiled
- `cargo test --test audit_serve`: 34 passing, 0 failed, 1 ignored

## Verification
unit-test-verified

## Notes, concerns, deviations, findings, thoughts, feedback

Docker is not installed in the cloud-agent VM; the `#[ignore]` localnet test is left ignored and marked `not-verified` for the live path. The `green_path_serve_axon_localnet` test uses Alice's well-known localnet mnemonic and `NetUid(0)` (root net); it would require `pair_from_mnemonic` (which is public in `agcli::wallet::keypair`) and a running subtensor at `ws://127.0.0.1:9944`.

## Findings

1. **`serve reset` always fails at chain level (critical)** — `ServeCommands::Reset` submits `serve_axon` with `port: 0`. The pallet's `validate_axon_data` check `if port == 0 { return Err(InvalidPort) }` fires unconditionally. The command cannot succeed on any live subnet.

2. **`serve prometheus` docs missing required `--netuid`** — The prior doc example `agcli serve prometheus --ip 1.2.3.4 --port 9090` omitted `--netuid`, which is a required arg. The CLI rejects such invocations with a parse error.

3. **`serve axon-tls` `--cert` format misleading (critical)** — Flag is documented as "DER or PEM". The pallet's `NeuronCertificate::try_from(Vec<u8>)` accepts at most 65 bytes (byte 0 = algorithm, bytes 1–64 = raw public key). Any standard PEM or DER file (typically hundreds to thousands of bytes) is silently discarded with no error returned. The axon endpoint is registered but no certificate is stored.

4. **IPv6 not exposed** — All serve subcommands hardcode `ip_type: 4` in `AxonInfo` construction. No `--ip-type` flag exists. IPv6 axon/prometheus endpoints cannot be registered via agcli.

5. **`--output json` silently ignored** — `handle_serve` calls `println!` unconditionally; `ctx.output` is never consulted. Agent consumers using `--output json` receive unstructured human-readable text.

6. **`serve_prometheus` and `serve_axon_tls` use dynamic raw calls with `Value::u128`** — `serve_axon` uses the generated static API; `serve_prometheus` and `serve_axon_tls` use `submit_raw_call` with `Value::u128` for every integer argument regardless of SCALE type. Subxt's metadata encoder handles the coercion, but the inconsistency is fragile.

7. **`serve batch-axon` rejects empty array** — `validate_batch_axon_json` bails on `[]`; an empty file fails before the handler which would print "0 entries". This is undocumented behavior.

8. **`placeholder1`/`placeholder2` not exposed** — Both are hardcoded to `0` with no CLI flag to override.

9. **No `serve show` / `serve status` command** — There is no read-path subcommand to inspect the currently registered axon or Prometheus endpoint for a hotkey.

10. **Handler line numbers in old docs were wrong** — Previous docs cited `handle_serve()` at L430; actual location is L1153 in `network_cmds.rs`.

## Suggested follow-ups

- **Fix `serve reset`** — implement a workaround (loopback placeholder serve, e.g. `127.0.0.1:1`) since `serve_axon` requires non-zero port; or remove the subcommand and document the absence of a "clear" dispatchable.
- **Fix `--cert` documentation and validation** — add a pre-submission check that the cert file is ≤ 65 bytes and document the binary format requirement. Rename flag to `--neuron-cert` to distinguish from standard TLS certs.
- **Add `--ip-type` flag** — to support IPv6 across all serve subcommands.
- **Add JSON output** — serialize the final result (tx hash, confirmed endpoint) via `ctx.output` for agent consumers.
- **Migrate `serve_prometheus`/`serve_axon_tls` to static API** — for consistency with `serve_axon` and to eliminate `Value::u128` coercion fragility.
- **Add `serve show` read subcommand** — query `SubtensorModule.Axons` and `SubtensorModule.Prometheus` for the current hotkey.