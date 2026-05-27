//! Audit green-path tests for the `contracts` command group.
//!
//! Covers: upload, instantiate, call, remove-code.
//! Run with: cargo test --test audit_contracts

use agcli::cli::Cli;
use clap::Parser;

// ── upload ──────────────────────────────────────────────────────────────────

#[test]
fn parse_upload_minimal() {
    let cli = Cli::try_parse_from(["agcli", "contracts", "upload", "--code", "contract.wasm"]);
    assert!(cli.is_ok(), "upload minimal: {:?}", cli.err());
}

#[test]
fn parse_upload_with_storage_deposit_limit() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "upload",
        "--code",
        "/tmp/contract.wasm",
        "--storage-deposit-limit",
        "5000000000",
    ]);
    assert!(
        cli.is_ok(),
        "upload with storage-deposit-limit: {:?}",
        cli.err()
    );
}

#[test]
fn parse_upload_missing_code_flag_rejected() {
    let cli = Cli::try_parse_from(["agcli", "contracts", "upload"]);
    assert!(cli.is_err(), "upload without --code must be rejected");
}

// ── instantiate ──────────────────────────────────────────────────────────────

#[test]
fn parse_instantiate_minimal() {
    // Only --code-hash is required; all others have defaults.
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "instantiate",
        "--code-hash",
        "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab",
    ]);
    assert!(cli.is_ok(), "instantiate minimal: {:?}", cli.err());
}

#[test]
fn parse_instantiate_full() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "instantiate",
        "--code-hash",
        "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "--value",
        "1000000",
        "--data",
        "0xcafebabe",
        "--salt",
        "0x01",
        "--gas-ref-time",
        "20000000000",
        "--gas-proof-size",
        "2097152",
        "--storage-deposit-limit",
        "10000000000",
    ]);
    assert!(cli.is_ok(), "instantiate full: {:?}", cli.err());
}

#[test]
fn parse_instantiate_default_gas_values_accepted() {
    // Verifies the clap defaults (10_000_000_000 / 1_048_576) parse cleanly.
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "instantiate",
        "--code-hash",
        "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "--gas-ref-time",
        "10000000000",
        "--gas-proof-size",
        "1048576",
    ]);
    assert!(cli.is_ok(), "instantiate default gas: {:?}", cli.err());
}

#[test]
fn parse_instantiate_missing_code_hash_rejected() {
    let cli = Cli::try_parse_from(["agcli", "contracts", "instantiate"]);
    assert!(
        cli.is_err(),
        "instantiate without --code-hash must be rejected"
    );
}

// ── call ──────────────────────────────────────────────────────────────────────

#[test]
fn parse_call_minimal() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "call",
        "--contract",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--data",
        "0x",
    ]);
    assert!(cli.is_ok(), "call minimal: {:?}", cli.err());
}

#[test]
fn parse_call_full() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "call",
        "--contract",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--data",
        "0xdeadbeef01020304",
        "--value",
        "500000",
        "--gas-ref-time",
        "5000000000",
        "--gas-proof-size",
        "524288",
        "--storage-deposit-limit",
        "1000000",
    ]);
    assert!(cli.is_ok(), "call full: {:?}", cli.err());
}

#[test]
fn parse_call_missing_contract_rejected() {
    let cli = Cli::try_parse_from(["agcli", "contracts", "call", "--data", "0xdeadbeef"]);
    assert!(cli.is_err(), "call without --contract must be rejected");
}

#[test]
fn parse_call_missing_data_rejected() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "call",
        "--contract",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
    assert!(cli.is_err(), "call without --data must be rejected");
}

// ── remove-code ───────────────────────────────────────────────────────────────

#[test]
fn parse_remove_code_minimal() {
    let cli = Cli::try_parse_from([
        "agcli",
        "contracts",
        "remove-code",
        "--code-hash",
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    ]);
    assert!(cli.is_ok(), "remove-code minimal: {:?}", cli.err());
}

#[test]
fn parse_remove_code_missing_hash_rejected() {
    let cli = Cli::try_parse_from(["agcli", "contracts", "remove-code"]);
    assert!(
        cli.is_err(),
        "remove-code without --code-hash must be rejected"
    );
}

// ── structural / routing ──────────────────────────────────────────────────────

#[test]
fn parse_contracts_help_does_not_panic() {
    // --help triggers clap's early-exit (Err with DisplayHelp kind), not a panic.
    let result = Cli::try_parse_from(["agcli", "contracts", "--help"]);
    match result {
        Err(e) => assert_eq!(e.kind(), clap::error::ErrorKind::DisplayHelp),
        Ok(_) => panic!("expected DisplayHelp error from --help"),
    }
}

#[test]
fn parse_contracts_no_subcommand_rejected() {
    // The contracts group requires a subcommand.
    let cli = Cli::try_parse_from(["agcli", "contracts"]);
    assert!(
        cli.is_err(),
        "contracts without subcommand must be rejected"
    );
}

// ── ignore-gated localnet integration test ────────────────────────────────────

/// Attempts to connect to a local subtensor node and exercise the contracts
/// pallet.  This test is `#[ignore]`-gated because it requires a running
/// localnet instance (Docker / `agcli localnet start`), which is unavailable
/// in the CI cloud-agent VM.
///
/// To run manually:
///   agcli localnet start          # starts ghcr.io/opentensor/subtensor-localnet
///   cargo test --test audit_contracts green_path_contracts -- --ignored --nocapture
#[tokio::test]
#[ignore = "requires running subtensor localnet (Docker); run with --ignored after `agcli localnet start`"]
async fn green_path_contracts() {
    use agcli::chain::Client;
    use std::fs;
    use tempfile::TempDir;

    const LOCALNET: &str = "ws://127.0.0.1:9944";

    let client = Client::connect(LOCALNET)
        .await
        .expect("connect to localnet");

    // Verify the Contracts pallet is reachable by querying chain metadata.
    // A real green-path would:
    //   1. Build a minimal WASM ink! contract (or use a pre-built fixture).
    //   2. Call contracts_upload_code and capture the CodeStored event's code_hash.
    //   3. Call contracts_instantiate with that code_hash.
    //   4. Call contracts_call against the instantiated address.
    //   5. Call contracts_remove_code to clean up.
    //
    // Because this environment does not have a pre-built WASM binary or ink!
    // tooling, we only assert that the node is reachable and the runtime
    // reports pallet index 29 for Contracts (as declared in subtensor runtime).
    let meta = client.metadata();
    let contracts_pallet = meta.pallet_by_name("Contracts");
    assert!(
        contracts_pallet.is_some(),
        "Contracts pallet (index 29) must be present in chain metadata"
    );
    let pallet = contracts_pallet.unwrap();
    // Verify the four dispatchables we rely on are present.
    for call_name in ["upload_code", "instantiate", "call", "remove_code"] {
        assert!(
            pallet.call_variant_by_name(call_name).is_some(),
            "Contracts::{call_name} must exist in chain metadata"
        );
    }
    println!("[ok] Contracts pallet and all 4 dispatchables verified in localnet metadata");

    // Minimal WASM sanity: confirm validate_wasm_file rejects non-WASM bytes.
    let tmp = TempDir::new().unwrap();
    let bad_wasm = tmp.path().join("bad.wasm");
    fs::write(&bad_wasm, b"not wasm bytes").unwrap();
    let data = fs::read(&bad_wasm).unwrap();
    let result = agcli::cli::helpers::validate_wasm_file(&data, bad_wasm.to_str().unwrap());
    assert!(
        result.is_err(),
        "validate_wasm_file should reject non-WASM bytes"
    );
}
