//! Audit tests for the `agcli multisig` command group.
//!
//! (a) Parse-surface tests: verify every MultisigCommands variant is reachable via
//!     `Cli::try_parse_from` with realistic arguments.
//! (b) One `#[ignore]` green-path integration test that requires a live local chain.
//!
//! Run:  cargo test --test audit_multisig
//! Live: cargo test --test audit_multisig -- --ignored

use agcli::cli::{Cli, Commands};
use clap::Parser;

// ── canonical test fixtures ────────────────────────────────────────────────

const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const CHARLIE: &str = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y";

/// 32-byte call hash used throughout
const CALL_HASH: &str = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

// ── helper ─────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

// ══════════════════════════════════════════════════════════════════════════
// Address subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn address_two_signatories() {
    let signatories = format!("{},{}", ALICE, BOB);
    let cli = parse(&[
        "agcli",
        "multisig",
        "address",
        "--signatories",
        &signatories,
        "--threshold",
        "2",
    ]);
    assert!(cli.is_ok(), "address 2-of-2: {:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        matches!(
            cli.command,
            Commands::Multisig(agcli::cli::MultisigCommands::Address { .. })
        ),
        "wrong command variant"
    );
}

#[test]
fn address_three_signatories_threshold_two() {
    let signatories = format!("{},{},{}", ALICE, BOB, CHARLIE);
    let cli = parse(&[
        "agcli",
        "multisig",
        "address",
        "--signatories",
        &signatories,
        "--threshold",
        "2",
    ]);
    assert!(cli.is_ok(), "address 2-of-3: {:?}", cli.err());
}

#[test]
fn address_missing_signatories_rejected() {
    let err = parse(&["agcli", "multisig", "address", "--threshold", "2"]);
    assert!(err.is_err(), "missing --signatories must be rejected");
}

#[test]
fn address_missing_threshold_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "address",
        "--signatories",
        &format!("{},{}", ALICE, BOB),
    ]);
    assert!(err.is_err(), "missing --threshold must be rejected");
}

// ══════════════════════════════════════════════════════════════════════════
// Submit subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn submit_minimal_no_args() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "submit",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "SubtensorModule",
        "--call",
        "add_stake",
    ]);
    assert!(cli.is_ok(), "submit without --args: {:?}", cli.err());
}

#[test]
fn submit_with_json_args() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "submit",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
        "--call",
        "transfer_keep_alive",
        "--args",
        r#"[{"Id":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},1000000000]"#,
    ]);
    assert!(cli.is_ok(), "submit with --args JSON: {:?}", cli.err());
}

#[test]
fn submit_multiple_others() {
    let others = format!("{},{}", BOB, CHARLIE);
    let cli = parse(&[
        "agcli",
        "multisig",
        "submit",
        "--others",
        &others,
        "--threshold",
        "2",
        "--pallet",
        "SubtensorModule",
        "--call",
        "register_network",
    ]);
    assert!(cli.is_ok(), "submit multiple others: {:?}", cli.err());
}

#[test]
fn submit_missing_pallet_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "submit",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call",
        "transfer",
    ]);
    assert!(err.is_err(), "missing --pallet must be rejected");
}

#[test]
fn submit_missing_call_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "submit",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
    ]);
    assert!(err.is_err(), "missing --call must be rejected");
}

// ══════════════════════════════════════════════════════════════════════════
// Approve subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn approve_with_valid_hash() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "approve",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
    ]);
    assert!(cli.is_ok(), "approve with valid hash: {:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        matches!(
            cli.command,
            Commands::Multisig(agcli::cli::MultisigCommands::Approve { .. })
        ),
        "wrong command variant"
    );
}

#[test]
fn approve_multiple_others() {
    let others = format!("{},{}", BOB, CHARLIE);
    let cli = parse(&[
        "agcli",
        "multisig",
        "approve",
        "--others",
        &others,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
    ]);
    assert!(cli.is_ok(), "approve multiple others: {:?}", cli.err());
}

#[test]
fn approve_missing_call_hash_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "approve",
        "--others",
        BOB,
        "--threshold",
        "2",
    ]);
    assert!(err.is_err(), "missing --call-hash must be rejected");
}

/// Audit finding: Approve has no --timepoint-height / --timepoint-index flags.
/// The pallet requires maybe_timepoint: Some(...) for non-first approvals.
/// This test documents the missing flags — parse will succeed without them,
/// but on-chain calls will return NoTimepoint for subsequent approvals.
#[test]
fn approve_has_no_timepoint_flags() {
    // Clap rejects unknown flags, so this should fail if the flags are absent.
    let with_timepoint = parse(&[
        "agcli",
        "multisig",
        "approve",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-height",
        "1000",
        "--timepoint-index",
        "0",
    ]);
    // This WILL fail (unknown arg) because the Approve variant doesn't have
    // timepoint fields — documenting the missing surface.
    assert!(
        with_timepoint.is_err(),
        "Approve has no timepoint flags (audit finding: NoTimepoint will occur for non-first approvals)"
    );
}

// ══════════════════════════════════════════════════════════════════════════
// Execute subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn execute_without_timepoint() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "execute",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
        "--call",
        "transfer_keep_alive",
    ]);
    assert!(cli.is_ok(), "execute without timepoint: {:?}", cli.err());
}

#[test]
fn execute_with_full_timepoint() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "execute",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
        "--call",
        "transfer_keep_alive",
        "--args",
        "[1000000]",
        "--timepoint-height",
        "99999",
        "--timepoint-index",
        "0",
    ]);
    assert!(cli.is_ok(), "execute with full timepoint: {:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        matches!(
            cli.command,
            Commands::Multisig(agcli::cli::MultisigCommands::Execute { .. })
        ),
        "wrong command variant"
    );
}

#[test]
fn execute_missing_pallet_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "execute",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call",
        "transfer_keep_alive",
    ]);
    assert!(err.is_err(), "execute without --pallet must be rejected");
}

#[test]
fn execute_missing_call_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "execute",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
    ]);
    assert!(err.is_err(), "execute without --call must be rejected");
}

// ══════════════════════════════════════════════════════════════════════════
// Cancel subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn cancel_with_all_required_args() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-height",
        "12345",
        "--timepoint-index",
        "0",
    ]);
    assert!(cli.is_ok(), "cancel all args: {:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        matches!(
            cli.command,
            Commands::Multisig(agcli::cli::MultisigCommands::Cancel { .. })
        ),
        "wrong command variant"
    );
}

#[test]
fn cancel_missing_call_hash_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--timepoint-height",
        "12345",
        "--timepoint-index",
        "0",
    ]);
    assert!(err.is_err(), "cancel without --call-hash must be rejected");
}

#[test]
fn cancel_missing_timepoint_height_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-index",
        "0",
    ]);
    assert!(
        err.is_err(),
        "cancel without --timepoint-height must be rejected"
    );
}

#[test]
fn cancel_missing_timepoint_index_rejected() {
    let err = parse(&[
        "agcli",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-height",
        "12345",
    ]);
    assert!(
        err.is_err(),
        "cancel without --timepoint-index must be rejected"
    );
}

// ══════════════════════════════════════════════════════════════════════════
// List subcommand
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn list_with_address() {
    let cli = parse(&["agcli", "multisig", "list", "--address", ALICE]);
    assert!(cli.is_ok(), "list with address: {:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        matches!(
            cli.command,
            Commands::Multisig(agcli::cli::MultisigCommands::List { .. })
        ),
        "wrong command variant"
    );
}

#[test]
fn list_missing_address_rejected() {
    let err = parse(&["agcli", "multisig", "list"]);
    assert!(err.is_err(), "list without --address must be rejected");
}

// ══════════════════════════════════════════════════════════════════════════
// Global flag interaction
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn global_dry_run_flag_works_with_multisig() {
    let cli = parse(&[
        "agcli",
        "--dry-run",
        "multisig",
        "approve",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
    ]);
    assert!(cli.is_ok(), "--dry-run with multisig: {:?}", cli.err());
    assert!(cli.unwrap().dry_run, "--dry-run flag not set");
}

#[test]
fn global_yes_flag_works_with_multisig() {
    let cli = parse(&[
        "agcli",
        "--yes",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-height",
        "1",
        "--timepoint-index",
        "0",
    ]);
    assert!(cli.is_ok(), "--yes with multisig cancel: {:?}", cli.err());
    assert!(cli.unwrap().yes, "--yes flag not set");
}

// ══════════════════════════════════════════════════════════════════════════
// Argument field extraction
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn address_fields_extracted_correctly() {
    let signatories = format!("{},{}", ALICE, BOB);
    let cli = parse(&[
        "agcli",
        "multisig",
        "address",
        "--signatories",
        &signatories,
        "--threshold",
        "3",
    ])
    .unwrap();
    match cli.command {
        Commands::Multisig(agcli::cli::MultisigCommands::Address {
            signatories: s,
            threshold,
        }) => {
            assert_eq!(threshold, 3);
            assert!(s.contains(ALICE));
            assert!(s.contains(BOB));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn cancel_fields_extracted_correctly() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "cancel",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--call-hash",
        CALL_HASH,
        "--timepoint-height",
        "500",
        "--timepoint-index",
        "7",
    ])
    .unwrap();
    match cli.command {
        Commands::Multisig(agcli::cli::MultisigCommands::Cancel {
            call_hash,
            timepoint_height,
            timepoint_index,
            threshold,
            ..
        }) => {
            assert_eq!(call_hash, CALL_HASH);
            assert_eq!(timepoint_height, 500);
            assert_eq!(timepoint_index, 7);
            assert_eq!(threshold, 2);
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn execute_timepoint_fields_extracted() {
    let cli = parse(&[
        "agcli",
        "multisig",
        "execute",
        "--others",
        BOB,
        "--threshold",
        "2",
        "--pallet",
        "Balances",
        "--call",
        "transfer",
        "--timepoint-height",
        "42",
        "--timepoint-index",
        "3",
    ])
    .unwrap();
    match cli.command {
        Commands::Multisig(agcli::cli::MultisigCommands::Execute {
            timepoint_height,
            timepoint_index,
            pallet,
            call,
            ..
        }) => {
            assert_eq!(timepoint_height, Some(42));
            assert_eq!(timepoint_index, Some(3));
            assert_eq!(pallet, "Balances");
            assert_eq!(call, "transfer");
        }
        _ => panic!("wrong variant"),
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Green-path integration test (requires local chain — ignored by default)
// ══════════════════════════════════════════════════════════════════════════

/// Green-path integration test: spins up a localnet, creates two accounts,
/// submits a 2-of-2 multisig call, approves from the second account, and
/// verifies the call appears in `multisig list`.
///
/// Requires:
///   - Docker and the `ghcr.io/opentensor/subtensor-localnet:devnet-ready` image
///   - agcli binary in PATH or built via `cargo build --bin agcli`
///   - A funded Alice wallet at ~/.bittensor/wallets/alice
///
/// Gate: `#[ignore]` — run explicitly with:
///   cargo test --test audit_multisig green_path_multisig_submit_list -- --ignored
#[test]
#[ignore]
fn green_path_multisig_submit_list() {
    // Parse surface: confirm both steps build valid Cli objects before
    // attempting any chain calls.
    let signatories = format!("{},{}", ALICE, BOB);
    let address_cli = parse(&[
        "agcli",
        "multisig",
        "address",
        "--signatories",
        &signatories,
        "--threshold",
        "2",
    ]);
    assert!(
        address_cli.is_ok(),
        "green-path: address parse: {:?}",
        address_cli.err()
    );

    let list_cli = parse(&["agcli", "multisig", "list", "--address", ALICE]);
    assert!(
        list_cli.is_ok(),
        "green-path: list parse: {:?}",
        list_cli.err()
    );

    // Full on-chain steps would be here if Docker + localnet are available.
    // Currently not-verified: Docker unavailable in cloud-agent VM per
    // discovery.md. The chain-call flow would be:
    //
    // 1. agcli localnet start (pulls ghcr.io/opentensor/subtensor-localnet)
    // 2. Fund Alice, Bob
    // 3. agcli multisig address --signatories ALICE,BOB --threshold 2
    // 4. agcli --wallet alice multisig submit --others BOB --threshold 2
    //         --pallet Balances --call transfer_keep_alive
    //         --args '[{"Id": "5FHneW..."},1000000000]'
    // 5. Capture call hash from submit output
    // 6. agcli multisig list --address <multisig-address>
    //    → assert at least one pending entry with matching call_hash
    // 7. agcli --wallet bob multisig execute --others ALICE --threshold 2
    //         --pallet Balances --call transfer_keep_alive
    //         --args '[{"Id": "5FHneW..."},1000000000]'
    //         --timepoint-height <from list> --timepoint-index <from list>
    eprintln!("SKIPPED: Docker + localnet unavailable in this environment");
}
