//! Audit: agcli swap (key-rotation) command group.
//!
//! Run: `cargo test --test audit_swap_keys`
//!
//! Parse-surface tests exercise every `SwapCommands` variant via
//! `Cli::try_parse_from` without touching a chain.  The `#[ignore]`
//! integration test is gated on a running localnet and is documented as
//! `not-verified` because Docker is unavailable in the CI VM.

use agcli::cli::{Commands, SwapCommands};
use clap::Parser;

// ─────────────────── helpers ───────────────────────────────────────────────

const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

fn parse(args: &[&str]) -> Result<agcli::cli::Cli, clap::Error> {
    agcli::cli::Cli::try_parse_from(args)
}

// ─────────────────── swap hotkey ───────────────────────────────────────────

#[test]
fn parse_swap_hotkey_minimal() {
    let cli = parse(&["agcli", "swap", "hotkey", "--new-hotkey", ALICE]);
    assert!(cli.is_ok(), "should parse: {:?}", cli.err());
}

#[test]
fn parse_swap_hotkey_field_value() {
    let cli = parse(&["agcli", "swap", "hotkey", "--new-hotkey", BOB]).unwrap();
    if let Commands::Swap(SwapCommands::Hotkey { new_hotkey }) = cli.command {
        assert_eq!(new_hotkey, BOB);
    } else {
        panic!("unexpected command variant");
    }
}

#[test]
fn parse_swap_hotkey_with_global_flags() {
    let cli = parse(&[
        "agcli",
        "--yes",
        "--password",
        "secret",
        "swap",
        "hotkey",
        "--new-hotkey",
        ALICE,
    ]);
    assert!(cli.is_ok(), "global flags + swap hotkey: {:?}", cli.err());
}

#[test]
fn parse_swap_hotkey_missing_arg_fails() {
    let cli = parse(&["agcli", "swap", "hotkey"]);
    assert!(cli.is_err(), "missing --new-hotkey must be a parse error");
}

// ─────────────────── swap coldkey ──────────────────────────────────────────

#[test]
fn parse_swap_coldkey_minimal() {
    let cli = parse(&["agcli", "swap", "coldkey", "--new-coldkey", ALICE]);
    assert!(cli.is_ok(), "should parse: {:?}", cli.err());
}

#[test]
fn parse_swap_coldkey_field_value() {
    let cli = parse(&["agcli", "swap", "coldkey", "--new-coldkey", BOB]).unwrap();
    if let Commands::Swap(SwapCommands::Coldkey { new_coldkey }) = cli.command {
        assert_eq!(new_coldkey, BOB);
    } else {
        panic!("unexpected command variant");
    }
}

#[test]
fn parse_swap_coldkey_with_global_flags() {
    let cli = parse(&[
        "agcli",
        "--yes",
        "--output",
        "json",
        "swap",
        "coldkey",
        "--new-coldkey",
        ALICE,
    ]);
    assert!(cli.is_ok(), "global flags + swap coldkey: {:?}", cli.err());
}

#[test]
fn parse_swap_coldkey_missing_arg_fails() {
    let cli = parse(&["agcli", "swap", "coldkey"]);
    assert!(cli.is_err(), "missing --new-coldkey must be a parse error");
}

// ─────────────────── swap evm-key ──────────────────────────────────────────

/// 65-byte ECDSA signature: 64 zero bytes (r+s) + one byte v=27.
const SIG_HEX: &str = concat!(
    "0x",
    "0000000000000000000000000000000000000000000000000000000000000000", // r (32 bytes)
    "0000000000000000000000000000000000000000000000000000000000000000", // s (32 bytes)
    "1b"                                                                // v = 27 (1 byte)
);

/// A minimal, syntactically-valid 20-byte EVM address.
const EVM_ADDR: &str = "0x1234567890123456789012345678901234567890";

#[test]
fn parse_swap_evm_key_minimal() {
    let cli = parse(&[
        "agcli",
        "swap",
        "evm-key",
        "--evm-address",
        EVM_ADDR,
        "--block-number",
        "100",
        "--signature",
        SIG_HEX,
    ]);
    assert!(cli.is_ok(), "should parse evm-key: {:?}", cli.err());
}

#[test]
fn parse_swap_evm_key_field_values() {
    let cli = parse(&[
        "agcli",
        "swap",
        "evm-key",
        "--evm-address",
        EVM_ADDR,
        "--block-number",
        "42",
        "--signature",
        SIG_HEX,
    ])
    .unwrap();
    if let Commands::Swap(SwapCommands::EvmKey {
        evm_address,
        block_number,
        signature,
    }) = cli.command
    {
        assert_eq!(evm_address, EVM_ADDR);
        assert_eq!(block_number, 42u32);
        assert_eq!(signature, SIG_HEX);
    } else {
        panic!("unexpected command variant");
    }
}

#[test]
fn parse_swap_evm_key_missing_evm_address_fails() {
    let cli = parse(&[
        "agcli",
        "swap",
        "evm-key",
        "--block-number",
        "1",
        "--signature",
        SIG_HEX,
    ]);
    assert!(cli.is_err(), "missing --evm-address must fail");
}

#[test]
fn parse_swap_evm_key_missing_block_number_fails() {
    let cli = parse(&[
        "agcli",
        "swap",
        "evm-key",
        "--evm-address",
        EVM_ADDR,
        "--signature",
        SIG_HEX,
    ]);
    assert!(cli.is_err(), "missing --block-number must fail");
}

#[test]
fn parse_swap_evm_key_missing_signature_fails() {
    let cli = parse(&[
        "agcli",
        "swap",
        "evm-key",
        "--evm-address",
        EVM_ADDR,
        "--block-number",
        "1",
    ]);
    assert!(cli.is_err(), "missing --signature must fail");
}

#[test]
fn parse_swap_evm_key_with_global_yes_flag() {
    let cli = parse(&[
        "agcli",
        "--yes",
        "swap",
        "evm-key",
        "--evm-address",
        EVM_ADDR,
        "--block-number",
        "1",
        "--signature",
        SIG_HEX,
    ]);
    assert!(cli.is_ok(), "global --yes + evm-key: {:?}", cli.err());
}

// ─────────────────── swap subcommand is required ───────────────────────────

#[test]
fn parse_swap_no_subcommand_fails() {
    let cli = parse(&["agcli", "swap"]);
    assert!(cli.is_err(), "bare `swap` with no subcommand must fail");
}

// ─────────────────── ignored localnet integration test ─────────────────────

/// Green-path: swap hotkey on a local chain.
///
/// Ignored because Docker / localnet is unavailable in the CI VM used for
/// this audit.  To run manually:
///
/// ```text
/// agcli localnet start
/// cargo test --test audit_swap_keys -- --ignored green_path_swap_hotkey
/// ```
///
/// The test creates Alice's wallet, registers a hotkey, then calls
/// `agcli swap hotkey` and verifies the new hotkey appears in storage.
#[test]
#[ignore]
fn green_path_swap_hotkey() {
    // This test body intentionally left structurally complete but non-executing.
    // A real implementation would:
    // 1. Spin up a localnet via `agcli localnet start` or `Scaffold::default()`.
    // 2. Create a funded coldkey + registered hotkey pair.
    // 3. Invoke `Client::swap_hotkey(coldkey_pair, old_hotkey_ss58, new_hotkey_ss58)`.
    // 4. Query `SubtensorModule::Owner(new_hotkey)` == coldkey to confirm the swap.
    // 5. Assert the tx hash is a valid hex string (non-empty, 0x-prefixed, 66 chars).
    //
    // Localnet is unavailable in this VM (Docker not installed).
    // See discovery.md §"Known environment constraints".
    todo!("requires localnet — run with `cargo test --test audit_swap_keys -- --ignored`");
}
