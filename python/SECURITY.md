# agcli Python SDK security notes

## Security model

- **Pickle disabled at the binding boundary.** All PyO3 classes exported from `agcli._agcli` implement `__reduce__` and `__getstate__` to raise `TypeError`, preventing accidental serialization of wallets, signers, clients, and runtime state.
- **Coldkey unlock is password gated.** Spending keys remain encrypted at rest and are only loaded into memory after explicit `unlock_coldkey(password)` / `unlock_coldkey_with_password(password)` calls.
- **Signing uses sr25519.** `Wallet.sign_message(...)` signs with the same sr25519 key material used by the Rust SDK wallet implementation.
- **No implicit key export over FFI.** Secret key material is not returned across the Python binding boundary; only explicit public getters (for example `coldkey_ss58`, `hotkey_ss58`) are exposed.
- **Mnemonic handling is explicit and one-time.** Mnemonics are only returned at wallet creation time (`Wallet.create(...) -> (wallet, coldkey_mnemonic, hotkey_mnemonic)`) so callers can persist backups intentionally.

## Dependency audit

`cargo audit -q` was run from the repository root on this branch.

- Result: `17 vulnerabilities found`, `12 allowed warnings found`
- Notable advisories affecting the current dependency graph:
  - `RUSTSEC-2025-0020` (`pyo3`): upgrade recommended to `>=0.24.1`
  - Multiple `wasmtime` advisories in transitive dependencies pulled via the substrate/polkadot stack
  - `RUSTSEC-2026-0002` (`lru`) transitive advisory

These findings are currently upstream/transitive to the SDK dependency tree and should be tracked for coordinated dependency upgrades.
