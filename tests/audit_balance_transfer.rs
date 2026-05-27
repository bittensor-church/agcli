//! Audit: balance / transfer / transfer-all / transfer-keep-alive command group.
//!
//! Run with: cargo test --test audit_balance_transfer
//!
//! Covers:
//!   - Parse-surface tests for every subcommand flag combination.
//!   - Validation-layer (helpers) error-path tests (no chain required).
//!   - One `#[ignore]` localnet integration test (requires Docker + localnet at ws://127.0.0.1:9944).

use clap::Parser as _;

// ──── helpers ────────────────────────────────────────────────────────────────

/// Alice's dev SS58 address (standard Substrate dev key).
const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB";
/// Bob's dev SS58 address.
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

// ──── balance ────────────────────────────────────────────────────────────────

#[test]
fn parse_balance_no_args() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "balance"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance {
            address,
            watch,
            threshold,
            at_block,
        } => {
            assert!(address.is_none());
            assert!(watch.is_none());
            assert!(threshold.is_none());
            assert!(at_block.is_none());
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_with_address() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "balance", "--address", ALICE]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance { address, .. } => {
            assert_eq!(address.as_deref(), Some(ALICE));
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_with_at_block() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "balance", "--at-block", "1000"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance { at_block, .. } => {
            assert_eq!(at_block, Some(1000u32));
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_watch_no_interval() {
    // --watch without a value means "use default interval"
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "balance", "--watch"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance { watch, .. } => {
            // watch == Some(None): flag present, no numeric arg
            assert!(watch.is_some(), "watch should be Some");
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_watch_with_interval() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "balance", "--watch", "30"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance { watch, .. } => {
            assert_eq!(watch, Some(Some(30u64)));
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_with_threshold() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "balance", "--threshold", "5.0"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Balance { threshold, .. } => {
            assert!((threshold.unwrap() - 5.0f64).abs() < 1e-9);
        }
        _ => panic!("expected Commands::Balance"),
    }
}

#[test]
fn parse_balance_all_flags() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "balance",
        "--address",
        ALICE,
        "--threshold",
        "1.0",
        "--at-block",
        "999",
    ])
    .unwrap();
    assert!(cli.output.is_json());
    match cli.command {
        agcli::cli::Commands::Balance {
            address,
            threshold,
            at_block,
            ..
        } => {
            assert_eq!(address.as_deref(), Some(ALICE));
            assert!((threshold.unwrap() - 1.0f64).abs() < 1e-9);
            assert_eq!(at_block, Some(999u32));
        }
        _ => panic!("expected Commands::Balance"),
    }
}

// ──── transfer ───────────────────────────────────────────────────────────────

#[test]
fn parse_transfer_minimal() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "transfer", "--dest", BOB, "--amount", "1.0",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Transfer { dest, amount } => {
            assert_eq!(dest, BOB);
            assert!((amount - 1.0f64).abs() < 1e-9);
        }
        _ => panic!("expected Commands::Transfer"),
    }
}

#[test]
fn parse_transfer_with_global_flags() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "--yes",
        "--dry-run",
        "transfer",
        "--dest",
        BOB,
        "--amount",
        "0.001",
    ])
    .unwrap();
    assert!(cli.output.is_json());
    assert!(cli.yes);
    assert!(cli.dry_run);
    match cli.command {
        agcli::cli::Commands::Transfer { dest, amount } => {
            assert_eq!(dest, BOB);
            assert!((amount - 0.001f64).abs() < 1e-12);
        }
        _ => panic!("expected Commands::Transfer"),
    }
}

#[test]
fn parse_transfer_missing_dest_fails() {
    let result = agcli::cli::Cli::try_parse_from(["agcli", "transfer", "--amount", "1.0"]);
    assert!(result.is_err(), "should fail: --dest is required");
}

#[test]
fn parse_transfer_missing_amount_fails() {
    let result = agcli::cli::Cli::try_parse_from(["agcli", "transfer", "--dest", BOB]);
    assert!(result.is_err(), "should fail: --amount is required");
}

// ──── transfer-all ───────────────────────────────────────────────────────────

#[test]
fn parse_transfer_all_minimal() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "transfer-all", "--dest", BOB])
        .unwrap();
    match cli.command {
        agcli::cli::Commands::TransferAll { dest, keep_alive } => {
            assert_eq!(dest, BOB);
            assert!(!keep_alive, "keep_alive should default to false");
        }
        _ => panic!("expected Commands::TransferAll"),
    }
}

#[test]
fn parse_transfer_all_keep_alive() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "transfer-all", "--dest", BOB, "--keep-alive",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::TransferAll { dest, keep_alive } => {
            assert_eq!(dest, BOB);
            assert!(keep_alive);
        }
        _ => panic!("expected Commands::TransferAll"),
    }
}

#[test]
fn parse_transfer_all_with_global_yes() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "--yes", "transfer-all", "--dest", BOB, "--keep-alive",
    ])
    .unwrap();
    assert!(cli.yes);
    match cli.command {
        agcli::cli::Commands::TransferAll { keep_alive, .. } => {
            assert!(keep_alive);
        }
        _ => panic!("expected Commands::TransferAll"),
    }
}

#[test]
fn parse_transfer_all_missing_dest_fails() {
    let result = agcli::cli::Cli::try_parse_from(["agcli", "transfer-all"]);
    assert!(result.is_err(), "should fail: --dest is required");
}

// ──── transfer-keep-alive ─────────────────────────────────────────────────────

#[test]
fn parse_transfer_keep_alive_minimal() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "transfer-keep-alive",
        "--dest",
        BOB,
        "--amount",
        "2.5",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::TransferKeepAlive { dest, amount } => {
            assert_eq!(dest, BOB);
            assert!((amount - 2.5f64).abs() < 1e-9);
        }
        _ => panic!("expected Commands::TransferKeepAlive"),
    }
}

#[test]
fn parse_transfer_keep_alive_with_global_flags() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "--yes",
        "transfer-keep-alive",
        "--dest",
        ALICE,
        "--amount",
        "0.5",
    ])
    .unwrap();
    assert!(cli.output.is_json());
    assert!(cli.yes);
    match cli.command {
        agcli::cli::Commands::TransferKeepAlive { dest, amount } => {
            assert_eq!(dest, ALICE);
            assert!((amount - 0.5f64).abs() < 1e-9);
        }
        _ => panic!("expected Commands::TransferKeepAlive"),
    }
}

#[test]
fn parse_transfer_keep_alive_missing_dest_fails() {
    let result =
        agcli::cli::Cli::try_parse_from(["agcli", "transfer-keep-alive", "--amount", "1.0"]);
    assert!(result.is_err(), "should fail: --dest is required");
}

#[test]
fn parse_transfer_keep_alive_missing_amount_fails() {
    let result =
        agcli::cli::Cli::try_parse_from(["agcli", "transfer-keep-alive", "--dest", BOB]);
    assert!(result.is_err(), "should fail: --amount is required");
}

// ──── validation-layer tests (no chain) ──────────────────────────────────────

#[test]
fn validate_amount_rejects_negative() {
    let err = agcli::cli::helpers::validate_amount(-1.0, "transfer amount").unwrap_err();
    let msg = format!("{:#}", err);
    assert!(msg.contains("transfer amount"), "label should appear in error: {}", msg);
    assert!(msg.contains("negative") || msg.contains("cannot be negative"), "{}", msg);
    // Classifies as VALIDATION exit code
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_amount_rejects_zero() {
    let err = agcli::cli::helpers::validate_amount(0.0, "transfer amount").unwrap_err();
    let msg = format!("{:#}", err);
    assert!(msg.contains("transfer amount"), "{}", msg);
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_amount_rejects_inf() {
    let err =
        agcli::cli::helpers::validate_amount(f64::INFINITY, "transfer amount").unwrap_err();
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_amount_accepts_positive() {
    agcli::cli::helpers::validate_amount(1.0, "transfer amount").unwrap();
    agcli::cli::helpers::validate_amount(0.000000001, "transfer amount").unwrap(); // 1 RAO
}

#[test]
fn validate_ss58_rejects_empty() {
    let err = agcli::cli::helpers::validate_ss58("", "destination").unwrap_err();
    let msg = format!("{:#}", err);
    assert!(msg.contains("destination") || msg.contains("Invalid"), "{}", msg);
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_ss58_rejects_garbage() {
    let err = agcli::cli::helpers::validate_ss58("not_an_ss58", "destination").unwrap_err();
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_ss58_accepts_valid_address() {
    // Alice's dev address must parse without error
    agcli::cli::helpers::validate_ss58(ALICE, "destination").unwrap();
}

#[test]
fn validate_threshold_rejects_negative() {
    let err =
        agcli::cli::helpers::validate_threshold(-0.1, "balance --threshold").unwrap_err();
    let msg = format!("{:#}", err);
    assert!(
        msg.contains("balance --threshold"),
        "label should appear: {}",
        msg
    );
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_threshold_rejects_nan() {
    let err =
        agcli::cli::helpers::validate_threshold(f64::NAN, "balance --threshold").unwrap_err();
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn validate_threshold_accepts_zero() {
    // Zero threshold means "always alert" — should be valid input
    agcli::cli::helpers::validate_threshold(0.0, "balance --threshold").unwrap();
}

// ──── error classification (transfer-specific) ───────────────────────────────

#[test]
fn classify_insufficient_balance_error_is_chain() {
    let err = anyhow::anyhow!(
        "Insufficient balance: you have 0.5 TAO but trying to transfer 1.0 TAO."
    );
    // Contains "insufficient" → CHAIN
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::CHAIN);
}

#[test]
fn classify_bad_ss58_dest_is_validation() {
    let err = anyhow::anyhow!("Invalid destination address 'garbage'.");
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn classify_bad_amount_is_validation() {
    let err = anyhow::anyhow!("Invalid transfer amount: -1. Amount cannot be negative.");
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

// ──── balance type round-trip ─────────────────────────────────────────────────

#[test]
fn balance_from_tao_round_trips() {
    // 1 TAO = 1_000_000_000 RAO; verify round-trip through the Balance newtype
    let b = agcli::types::Balance::from_tao(1.0);
    assert_eq!(b.rao(), 1_000_000_000u64);
    // tao() returns f64 — check 1 TAO round-trips within f64 precision
    assert!((b.tao() - 1.0f64).abs() < 1e-9, "tao() = {}", b.tao());
}

#[test]
fn balance_from_rao_one_rao() {
    let b = agcli::types::Balance::from_rao(1u64);
    assert_eq!(b.rao(), 1u64);
    // 1 RAO = 0.000000001 TAO
    assert!((b.tao() - 1e-9f64).abs() < 1e-18, "tao() = {}", b.tao());
}

#[test]
fn balance_zero_is_zero() {
    let b = agcli::types::Balance::ZERO;
    assert_eq!(b.rao(), 0u64);
    assert_eq!(b.tao(), 0.0f64);
}

// ──── global flag interaction ─────────────────────────────────────────────────

#[test]
fn dry_run_is_global_not_subcommand() {
    // --dry-run must be before the subcommand
    let ok = agcli::cli::Cli::try_parse_from([
        "agcli", "--dry-run", "transfer", "--dest", BOB, "--amount", "1.0",
    ]);
    assert!(ok.is_ok());
    // --dry-run after the subcommand is not recognized
    let bad = agcli::cli::Cli::try_parse_from([
        "agcli", "transfer", "--dest", BOB, "--amount", "1.0", "--dry-run",
    ]);
    assert!(bad.is_err(), "dry-run after subcommand should not parse");
}

#[test]
fn yes_flag_is_global() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "--yes", "transfer", "--dest", BOB, "--amount", "0.1",
    ])
    .unwrap();
    assert!(cli.yes);
}

// ──── ignore: localnet integration ───────────────────────────────────────────

/// Green-path: query Alice's balance, then transfer from Alice to Bob.
///
/// Prerequisites (must all be true for this test to run):
///   - Docker available
///   - `sudo docker run ... ghcr.io/opentensor/subtensor-localnet:devnet-ready` is running
///   - Exposed on ws://127.0.0.1:9944
///   - Alice account funded (standard dev account, always funded on devnet-ready)
///
/// Run with: cargo test --test audit_balance_transfer -- --ignored
#[tokio::test]
#[ignore]
async fn green_path_balance_transfer_localnet() {
    use agcli::chain::Client;

    let client = Client::connect_with_retry(&["ws://127.0.0.1:9944"])
        .await
        .expect("localnet must be running at ws://127.0.0.1:9944");

    // 1. Balance query — Alice
    let alice_balance = client
        .get_balance_ss58(ALICE)
        .await
        .expect("get_balance_ss58 must succeed");
    assert!(alice_balance.rao() > 0, "Alice should have funds on devnet-ready");

    // 2. Balance query — Bob (may be zero)
    let bob_before = client
        .get_balance_ss58(BOB)
        .await
        .expect("get_balance_ss58 Bob must succeed");

    // 3. Historical query — block 0 must resolve (devnet starts at 0)
    let block_hash = client
        .get_block_hash(0)
        .await
        .expect("block 0 must exist");
    let _genesis_balance = client
        .get_balance_at_block(ALICE, block_hash)
        .await
        .expect("at-block query for genesis must succeed");

    // 4. transfer_allow_death: Alice → Bob, 0.001 TAO
    use sp_core::Pair as _;
    let alice_pair = sp_core::sr25519::Pair::from_string("//Alice", None)
        .expect("//Alice is a valid dev key");
    let amount = agcli::types::Balance::from_tao(0.001);
    let hash = client
        .transfer(&alice_pair, BOB, amount)
        .await
        .expect("transfer must succeed on localnet");
    assert!(
        hash.starts_with("0x"),
        "tx hash should be a hex string: {}",
        hash
    );

    // 5. Verify Bob's balance increased
    let bob_after = client
        .get_balance_ss58(BOB)
        .await
        .expect("get_balance_ss58 Bob after transfer must succeed");
    assert!(
        bob_after.rao() > bob_before.rao(),
        "Bob's balance should have increased: before={} after={}",
        bob_before.rao(),
        bob_after.rao()
    );
}
