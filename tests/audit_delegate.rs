//! Audit: agcli delegate subcommand group — parse-surface and integration tests.
//!
//! Run with: cargo test --test audit_delegate
//!
//! Covers: list, show, decrease-take, increase-take.
//! The `#[ignore]` test requires a running localnet on ws://127.0.0.1:9944.

use clap::Parser;

// ──────── helpers ────────

fn parse(args: &[&str]) -> Result<agcli::cli::Cli, clap::Error> {
    agcli::cli::Cli::try_parse_from(args)
}

fn assert_parses(args: &[&str]) {
    let result = parse(args);
    assert!(
        result.is_ok(),
        "expected parse to succeed for {:?}, got: {:?}",
        args,
        result.err()
    );
}

fn assert_parse_fails(args: &[&str]) {
    let result = parse(args);
    assert!(
        result.is_err(),
        "expected parse to fail for {:?} but it succeeded",
        args,
    );
}

// ──────── parse-surface tests ────────

#[test]
fn parse_delegate_list() {
    assert_parses(&["agcli", "delegate", "list"]);
}

#[test]
fn parse_delegate_show_no_hotkey() {
    // hotkey is optional — defaults to wallet hotkey
    assert_parses(&["agcli", "delegate", "show"]);
}

#[test]
fn parse_delegate_show_with_hotkey() {
    assert_parses(&[
        "agcli",
        "delegate",
        "show",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
}

#[test]
fn parse_delegate_decrease_take_minimal() {
    // --take is required; hotkey is optional
    assert_parses(&["agcli", "delegate", "decrease-take", "--take", "10.0"]);
}

#[test]
fn parse_delegate_decrease_take_with_hotkey() {
    assert_parses(&[
        "agcli",
        "delegate",
        "decrease-take",
        "--take",
        "5.0",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
}

#[test]
fn parse_delegate_decrease_take_zero() {
    // 0% is within the allowed range (min take is queried from chain at runtime)
    assert_parses(&["agcli", "delegate", "decrease-take", "--take", "0"]);
}

#[test]
fn parse_delegate_decrease_take_missing_take_flag_fails() {
    assert_parse_fails(&["agcli", "delegate", "decrease-take"]);
}

#[test]
fn parse_delegate_increase_take_minimal() {
    assert_parses(&["agcli", "delegate", "increase-take", "--take", "15.0"]);
}

#[test]
fn parse_delegate_increase_take_with_hotkey() {
    assert_parses(&[
        "agcli",
        "delegate",
        "increase-take",
        "--take",
        "18.0",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
}

#[test]
fn parse_delegate_increase_take_missing_take_flag_fails() {
    assert_parse_fails(&["agcli", "delegate", "increase-take"]);
}

// ──────── argument-encoding unit tests ────────

/// take percentage → u16 SCALE encoding used by both extrinsics.
/// Formula: (pct / 100.0 * 65535.0).round() as u16
/// Pallet expects this exact encoding (full-range linear, not per-mill).
#[test]
fn take_encoding_boundary_values() {
    let encode = |pct: f64| -> u16 { (pct / 100.0 * 65535.0).round().min(65535.0) as u16 };

    // 0% → 0
    assert_eq!(encode(0.0), 0);
    // 18% → 11796 (InitialDefaultDelegateTake)
    assert_eq!(encode(18.0), 11796);
    // 100% → 65535
    assert_eq!(encode(100.0), 65535);
    // 1% → 655
    assert_eq!(encode(1.0), 655);
    // 10% → 6553 (rounded)
    assert_eq!(encode(10.0), 6554);
}

/// validate_delegate_take rejects values outside [0, 18]%.
/// This is a CLI-side gate that runs before the chain extrinsic.
/// Note: the chain's MaxDelegateTake is a runtime storage value that can
/// be changed by admin-utils sudo_set_min_delegate_take; the CLI hardcodes
/// 18.0 as upper bound, which may diverge if the chain admin lowers the cap.
#[test]
fn validate_take_range() {
    // in-range values
    for pct in [0.0_f64, 0.01, 1.0, 9.0, 18.0] {
        assert!(
            agcli::cli::helpers::validate_delegate_take(pct).is_ok(),
            "expected {pct}% to be valid"
        );
    }
    // out-of-range values
    for pct in [-0.01_f64, 18.01, 19.0, 100.0, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            agcli::cli::helpers::validate_delegate_take(pct).is_err(),
            "expected {pct}% to be rejected"
        );
    }
    // NaN is non-finite — must be rejected
    assert!(agcli::cli::helpers::validate_delegate_take(f64::NAN).is_err());
}

// ──────── error-classification tests ────────

/// DelegateTakeTooLow and DelegateTakeTooHigh must map to exit_code::CHAIN (13).
#[test]
fn error_classify_delegate_take_errors() {
    use agcli::error::{classify, exit_code};

    for msg in &[
        "Dispatch error: DelegateTakeTooLow",
        "Dispatch error: DelegateTakeTooHigh",
    ] {
        let err = anyhow::anyhow!("{}", msg);
        assert_eq!(
            classify(&err),
            exit_code::CHAIN,
            "expected CHAIN(13) for '{msg}'"
        );
    }
}

/// DelegateTxRateLimitExceeded — pallet error for increase_take rate limit.
/// This error IS produced by the chain but is NOT explicitly covered by a
/// classify test or hint in src/error.rs; it falls through to CHAIN via the
/// generic "Dispatch error:" branch. Test documents current behaviour.
#[test]
fn error_classify_delegate_tx_rate_limit() {
    use agcli::error::{classify, exit_code};

    let err = anyhow::anyhow!("Dispatch error: DelegateTxRateLimitExceeded");
    assert_eq!(
        classify(&err),
        exit_code::CHAIN,
        "DelegateTxRateLimitExceeded should classify as CHAIN(13)"
    );
}

/// NonAssociatedColdKey — pallet error when coldkey doesn't own the hotkey.
#[test]
fn error_classify_non_associated_coldkey() {
    use agcli::error::{classify, exit_code};

    let err = anyhow::anyhow!("Dispatch error: NonAssociatedColdKey");
    assert_eq!(classify(&err), exit_code::CHAIN);
}

// ──────── green-path integration test (requires localnet) ────────

/// End-to-end green path: list delegates and show delegate info against a
/// running localnet at ws://127.0.0.1:9944.
///
/// Prerequisites:
///   docker run -d --network=host ghcr.io/opentensor/subtensor-localnet:devnet-ready
///
/// Run with: cargo test --test audit_delegate green_path_delegate -- --ignored
#[tokio::test]
#[ignore = "requires localnet on ws://127.0.0.1:9944"]
async fn green_path_delegate() {
    use agcli::chain::Client;

    let endpoint = std::env::var("AGCLI_ENDPOINT")
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());

    let client = Client::connect(&endpoint)
        .await
        .expect("should connect to localnet");

    // list must return without error; it may be empty on a fresh chain
    let delegates = client.get_delegates().await.expect("get_delegates failed");
    println!("delegate count: {}", delegates.len());

    // get_nominator_min_stake should return a u128 (zero on fresh chain is fine)
    let min_stake = client
        .get_nominator_min_stake()
        .await
        .expect("get_nominator_min_stake failed");
    println!("nominator min stake (rao): {min_stake}");
}
