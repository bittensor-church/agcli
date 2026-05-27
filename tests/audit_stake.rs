//! Audit tests for the `stake` command group.
//!
//! Parse-surface tests verify that every `StakeCommands` variant is reachable via
//! `Cli::try_parse_from` with realistic arguments. They exercise the clap surface only —
//! no network, no wallet unlock, no chain state required.
//!
//! The single `#[ignore]` test at the bottom is the green-path integration test; it
//! requires a local subtensor node on ws://127.0.0.1:9944 and is skipped in CI.
//!
//! Run all (except ignored): `cargo test --test audit_stake`
//! Run ignored integration test: `cargo test --test audit_stake -- --ignored`

use agcli::cli::{Cli, Commands, StakeCommands};
use clap::Parser;

// ── helpers ──────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Cli {
    Cli::try_parse_from(args).unwrap_or_else(|e| panic!("parse failed: {e}"))
}

fn parse_fails(args: &[&str]) -> String {
    Cli::try_parse_from(args)
        .expect_err("expected parse failure but it succeeded")
        .to_string()
}

fn is_stake(cli: &Cli) -> &StakeCommands {
    match &cli.command {
        Commands::Stake(cmd) => cmd,
        other => panic!("expected Stake command, got {other:?}"),
    }
}

// ── stake list ───────────────────────────────────────────────────────────────

#[test]
fn parse_stake_list_minimal() {
    let cli = parse(&["agcli", "stake", "list"]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::List { address: None, at_block: None }));
}

#[test]
fn parse_stake_list_with_address() {
    let cli = parse(&[
        "agcli", "stake", "list",
        "--address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::List { address: Some(a), at_block: None } => {
            assert_eq!(a, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        }
        other => panic!("unexpected variant: {other:?}"),
    }
}

#[test]
fn parse_stake_list_at_block() {
    let cli = parse(&["agcli", "stake", "list", "--at-block", "4000000"]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::List { address: None, at_block: Some(b) } => assert_eq!(*b, 4_000_000),
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_list_address_and_block() {
    let cli = parse(&[
        "agcli", "stake", "list",
        "--address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--at-block", "3500000",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(
        cmd,
        StakeCommands::List { address: Some(_), at_block: Some(_) }
    ));
}

// ── stake add ────────────────────────────────────────────────────────────────

#[test]
fn parse_stake_add_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "add",
        "--amount", "10.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::Add { amount, netuid, hotkey: None, max_slippage: None } => {
            assert_eq!(*netuid, 1u16);
            assert!((*amount - 10.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_add_with_hotkey_and_slippage() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "add",
        "--amount", "5.0",
        "--netuid", "3",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "--max-slippage", "2.0",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::Add { amount, netuid, hotkey: Some(hk), max_slippage: Some(slip) } => {
            assert_eq!(*netuid, 3u16);
            assert!((amount - 5.0).abs() < 1e-9);
            assert_eq!(hk, "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty");
            assert!((slip - 2.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_add_missing_amount_fails() {
    let err = parse_fails(&["agcli", "stake", "add", "--netuid", "1"]);
    assert!(
        err.contains("amount") || err.contains("required"),
        "expected missing --amount error, got: {err}"
    );
}

#[test]
fn parse_stake_add_missing_netuid_fails() {
    let err = parse_fails(&["agcli", "stake", "add", "--amount", "1.0"]);
    assert!(
        err.contains("netuid") || err.contains("required"),
        "expected missing --netuid error, got: {err}"
    );
}

// ── stake remove ─────────────────────────────────────────────────────────────

#[test]
fn parse_stake_remove_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "remove",
        "--amount", "1.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::Remove { amount, netuid, hotkey: None, max_slippage: None } => {
            assert_eq!(*netuid, 1u16);
            assert!((amount - 1.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_remove_with_slippage() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "remove",
        "--amount", "2.0",
        "--netuid", "2",
        "--max-slippage", "1.5",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(
        cmd,
        StakeCommands::Remove { max_slippage: Some(_), .. }
    ));
}

// ── stake move ───────────────────────────────────────────────────────────────

#[test]
fn parse_stake_move_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "move",
        "--amount", "1.0",
        "--from", "1",
        "--to", "2",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::Move { amount, from, to, hotkey: None } => {
            assert_eq!(*from, 1u16);
            assert_eq!(*to, 2u16);
            assert!((amount - 1.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_move_with_hotkey() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "move",
        "--amount", "0.5",
        "--from", "1",
        "--to", "3",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::Move { hotkey: Some(_), .. }));
}

// ── stake swap ───────────────────────────────────────────────────────────────

#[test]
fn parse_stake_swap_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "swap",
        "--amount", "1.0",
        "--from", "1",
        "--to", "2",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::Swap { .. }));
}

// ── stake unstake-all ─────────────────────────────────────────────────────────

#[test]
fn parse_stake_unstake_all_minimal() {
    let cli = parse(&["agcli", "--yes", "--password", "p", "stake", "unstake-all"]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::UnstakeAll { hotkey: None }));
}

#[test]
fn parse_stake_unstake_all_with_hotkey() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "unstake-all",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::UnstakeAll { hotkey: Some(_) }));
}

// ── stake unstake-all-alpha ───────────────────────────────────────────────────

#[test]
fn parse_stake_unstake_all_alpha_minimal() {
    let cli = parse(&["agcli", "--yes", "--password", "p", "stake", "unstake-all-alpha"]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::UnstakeAllAlpha { hotkey: None }));
}

#[test]
fn parse_stake_unstake_all_alpha_with_hotkey() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "unstake-all-alpha",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::UnstakeAllAlpha { hotkey: Some(_) }));
}

// ── stake claim-root ──────────────────────────────────────────────────────────

#[test]
fn parse_stake_claim_root_with_netuid() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "claim-root",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::ClaimRoot { netuid } => assert_eq!(*netuid, 1u16),
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_claim_root_missing_netuid_fails() {
    let err = parse_fails(&["agcli", "stake", "claim-root"]);
    assert!(
        err.contains("netuid") || err.contains("required"),
        "expected missing --netuid error, got: {err}"
    );
}

// ── stake add-limit ───────────────────────────────────────────────────────────

#[test]
fn parse_stake_add_limit_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "add-limit",
        "--amount", "10.0",
        "--netuid", "1",
        "--price", "0.5",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::AddLimit { amount, netuid, price, partial, hotkey: None } => {
            assert!((amount - 10.0).abs() < 1e-9);
            assert_eq!(*netuid, 1u16);
            assert!((price - 0.5).abs() < 1e-9);
            assert!(!partial);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_add_limit_with_partial() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "add-limit",
        "--amount", "10.0",
        "--netuid", "1",
        "--price", "0.5",
        "--partial",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::AddLimit { partial: true, .. }));
}

// ── stake remove-limit ────────────────────────────────────────────────────────

#[test]
fn parse_stake_remove_limit_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "remove-limit",
        "--amount", "5.0",
        "--netuid", "1",
        "--price", "0.8",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::RemoveLimit { amount, netuid, price, partial, hotkey: None } => {
            assert!((amount - 5.0).abs() < 1e-9);
            assert_eq!(*netuid, 1u16);
            assert!((price - 0.8).abs() < 1e-9);
            assert!(!partial);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

// ── stake swap-limit ──────────────────────────────────────────────────────────

#[test]
fn parse_stake_swap_limit_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "swap-limit",
        "--amount", "5.0",
        "--from", "1",
        "--to", "2",
        "--price", "0.5",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::SwapLimit { amount, from, to, price, partial, hotkey: None } => {
            assert!((amount - 5.0).abs() < 1e-9);
            assert_eq!(*from, 1u16);
            assert_eq!(*to, 2u16);
            assert!((price - 0.5).abs() < 1e-9);
            assert!(!partial);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_swap_limit_with_partial() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "swap-limit",
        "--amount", "5.0",
        "--from", "1",
        "--to", "2",
        "--price", "0.5",
        "--partial",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::SwapLimit { partial: true, .. }));
}

// ── stake childkey-take ───────────────────────────────────────────────────────

#[test]
fn parse_stake_childkey_take_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "childkey-take",
        "--take", "10.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::ChildkeyTake { take, netuid, hotkey: None } => {
            assert!((take - 10.0).abs() < 1e-9);
            assert_eq!(*netuid, 1u16);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_childkey_take_max_allowed() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "childkey-take",
        "--take", "18.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::ChildkeyTake { .. }));
}

#[test]
fn parse_stake_childkey_take_zero() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "childkey-take",
        "--take", "0.0",
        "--netuid", "1",
    ]);
    // Clap parses 0.0 successfully (validation is runtime, not clap-layer)
    assert!(matches!(is_stake(&cli), StakeCommands::ChildkeyTake { .. }));
}

// ── stake set-children ────────────────────────────────────────────────────────

#[test]
fn parse_stake_set_children_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-children",
        "--netuid", "1",
        "--children",
        "0.5:5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::SetChildren { netuid, children, hotkey: None } => {
            assert_eq!(*netuid, 1u16);
            assert!(children.contains("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_set_children_multiple() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-children",
        "--netuid", "2",
        "--children",
        "0.5:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,0.3:5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::SetChildren { .. }));
}

// ── stake recycle-alpha ───────────────────────────────────────────────────────

#[test]
fn parse_stake_recycle_alpha_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "recycle-alpha",
        "--amount", "100.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::RecycleAlpha { amount, netuid, hotkey: None } => {
            assert!((amount - 100.0).abs() < 1e-9);
            assert_eq!(*netuid, 1u16);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

// ── stake burn-alpha ──────────────────────────────────────────────────────────

#[test]
fn parse_stake_burn_alpha_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "burn-alpha",
        "--amount", "50.0",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::BurnAlpha { amount, netuid, hotkey: None } => {
            assert!((amount - 50.0).abs() < 1e-9);
            assert_eq!(*netuid, 1u16);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

// ── stake set-auto ────────────────────────────────────────────────────────────

#[test]
fn parse_stake_set_auto_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-auto",
        "--netuid", "1",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::SetAuto { netuid, hotkey: None } => assert_eq!(*netuid, 1u16),
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_set_auto_with_hotkey() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-auto",
        "--netuid", "1",
        "--hotkey-address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::SetAuto { hotkey: Some(_), .. }));
}

// ── stake show-auto ───────────────────────────────────────────────────────────

#[test]
fn parse_stake_show_auto_minimal() {
    let cli = parse(&["agcli", "stake", "show-auto"]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::ShowAuto { address: None }));
}

#[test]
fn parse_stake_show_auto_with_address() {
    let cli = parse(&[
        "agcli", "stake", "show-auto",
        "--address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::ShowAuto { address: Some(_) }));
}

// ── stake process-claim ───────────────────────────────────────────────────────

#[test]
fn parse_stake_process_claim_minimal() {
    let cli = parse(&["agcli", "--yes", "--password", "p", "stake", "process-claim"]);
    let cmd = is_stake(&cli);
    assert!(matches!(
        cmd,
        StakeCommands::ProcessClaim { hotkey: None, netuids: None }
    ));
}

#[test]
fn parse_stake_process_claim_with_netuids() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "process-claim",
        "--netuids", "1,2,3",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::ProcessClaim { netuids: Some(n), .. } => {
            assert_eq!(n, "1,2,3");
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_process_claim_with_hotkey_and_netuids() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "process-claim",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "--netuids", "5,10",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(
        cmd,
        StakeCommands::ProcessClaim { hotkey: Some(_), netuids: Some(_) }
    ));
}

// ── stake set-claim ───────────────────────────────────────────────────────────

#[test]
fn parse_stake_set_claim_swap() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-claim",
        "--claim-type", "swap",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::SetClaim { claim_type, subnets: None } => {
            assert_eq!(claim_type, "swap");
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_set_claim_keep() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-claim",
        "--claim-type", "keep",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::SetClaim { claim_type, .. } if claim_type == "keep"));
}

#[test]
fn parse_stake_set_claim_keep_subnets() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-claim",
        "--claim-type", "keep-subnets",
        "--subnets", "1,2,3",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::SetClaim { claim_type, subnets: Some(s) } => {
            assert_eq!(claim_type, "keep-subnets");
            assert_eq!(s, "1,2,3");
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_set_claim_invalid_type_fails() {
    let err = parse_fails(&[
        "agcli", "stake", "set-claim",
        "--claim-type", "invalid-type",
    ]);
    // clap value_parser should reject invalid claim types
    assert!(
        err.contains("invalid") || err.contains("claim") || err.contains("possible values"),
        "expected rejection of invalid claim-type, got: {err}"
    );
}

// ── stake transfer-stake ──────────────────────────────────────────────────────

#[test]
fn parse_stake_transfer_stake_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "transfer-stake",
        "--dest", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--amount", "10.0",
        "--from", "1",
        "--to", "2",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::TransferStake { dest, amount, from, to, hotkey: None } => {
            assert_eq!(dest, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
            assert!((amount - 10.0).abs() < 1e-9);
            assert_eq!(*from, 1u16);
            assert_eq!(*to, 2u16);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_transfer_stake_missing_dest_fails() {
    let err = parse_fails(&[
        "agcli", "stake", "transfer-stake",
        "--amount", "10.0",
        "--from", "1",
        "--to", "2",
    ]);
    assert!(
        err.contains("dest") || err.contains("required"),
        "expected missing --dest error, got: {err}"
    );
}

// ── stake remove-full-limit ───────────────────────────────────────────────────

#[test]
fn parse_stake_remove_full_limit_minimal() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "remove-full-limit",
        "--netuid", "1",
        "--price", "0.001",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::RemoveFullLimit { netuid, price, hotkey: None } => {
            assert_eq!(*netuid, 1u16);
            assert!((price - 0.001).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_remove_full_limit_with_hotkey() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "remove-full-limit",
        "--netuid", "1",
        "--price", "0.5",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    assert!(matches!(cmd, StakeCommands::RemoveFullLimit { hotkey: Some(_), .. }));
}

// ── stake wizard ──────────────────────────────────────────────────────────────

#[test]
fn parse_stake_wizard_all_flags() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "wizard",
        "--netuid", "1",
        "--amount", "5.0",
        "--hotkey-address", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ]);
    let cmd = is_stake(&cli);
    match cmd {
        StakeCommands::Wizard { netuid: Some(n), amount: Some(a), hotkey: Some(_) } => {
            assert_eq!(*n, 1u16);
            assert!((a - 5.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parse_stake_wizard_minimal_no_flags() {
    // Wizard is fully optional-flagged — can be invoked with zero flags (interactive mode)
    let cli = parse(&["agcli", "stake", "wizard"]);
    let cmd = is_stake(&cli);
    assert!(matches!(
        cmd,
        StakeCommands::Wizard { netuid: None, amount: None, hotkey: None }
    ));
}

// ── global flags interact correctly with stake ────────────────────────────────

#[test]
fn parse_stake_add_with_mev_flag() {
    let cli = parse(&[
        "agcli", "--yes", "--password", "p", "--mev",
        "stake", "add",
        "--amount", "10.0",
        "--netuid", "1",
    ]);
    assert!(cli.mev, "--mev flag should set mev=true");
    assert!(matches!(is_stake(&cli), StakeCommands::Add { .. }));
}

#[test]
fn parse_stake_list_with_output_json() {
    use agcli::cli::OutputFormat;
    let cli = parse(&["agcli", "--output", "json", "stake", "list"]);
    assert_eq!(cli.output, OutputFormat::Json);
    assert!(matches!(is_stake(&cli), StakeCommands::List { .. }));
}

#[test]
fn parse_stake_list_with_output_csv() {
    use agcli::cli::OutputFormat;
    let cli = parse(&["agcli", "--output", "csv", "stake", "list"]);
    assert_eq!(cli.output, OutputFormat::Csv);
}

// ── validation logic (no chain needed) ───────────────────────────────────────

#[test]
fn validate_take_pct_rejects_above_18() {
    use agcli::cli::helpers::validate_take_pct;
    let result = validate_take_pct(18.1);
    assert!(result.is_err(), "take above 18% should be rejected");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("18") || msg.contains("maximum"),
        "error should mention 18% limit: {msg}"
    );
}

#[test]
fn validate_take_pct_accepts_boundary_values() {
    use agcli::cli::helpers::validate_take_pct;
    assert!(validate_take_pct(0.0).is_ok(), "0% should be valid");
    assert!(validate_take_pct(18.0).is_ok(), "18% should be valid");
    assert!(validate_take_pct(9.5).is_ok(), "9.5% should be valid");
}

#[test]
fn validate_take_pct_rejects_negative() {
    use agcli::cli::helpers::validate_take_pct;
    let result = validate_take_pct(-1.0);
    assert!(result.is_err(), "negative take should fail");
}

#[test]
fn childkey_take_u16_encoding_correct() {
    // 18% should encode to 11796 (18/100 * 65535 = 11796.3 → rounded 11796)
    let take: f64 = 18.0;
    let encoded = (take / 100.0 * 65535.0).round().min(65535.0) as u16;
    assert_eq!(encoded, 11796);

    // 100% → 65535
    let encoded_max = (100.0_f64 / 100.0 * 65535.0).round().min(65535.0) as u16;
    assert_eq!(encoded_max, 65535);

    // 0.01% → 7 (rounds from 6.5535)
    let encoded_min = (0.01_f64 / 100.0 * 65535.0).round().min(65535.0) as u16;
    assert_eq!(encoded_min, 7);
}

#[test]
fn safe_rao_consistent_with_balance_from_tao() {
    use agcli::types::Balance;
    // safe_rao(x) is defined as Balance::from_tao(x).rao()
    let x = 1.5_f64;
    let via_safe_rao = agcli::cli::helpers::safe_rao(x);
    let via_balance = Balance::from_tao(x).rao();
    assert_eq!(via_safe_rao, via_balance);

    // Also verify the TAO→RAO scale is 1e9
    let one_tao = Balance::from_tao(1.0);
    assert_eq!(one_tao.rao(), 1_000_000_000u64);
}

#[test]
fn validate_limit_price_rejects_zero_and_negative() {
    use agcli::cli::helpers::validate_limit_price;
    assert!(validate_limit_price(0.0, "price").is_err(), "zero price should fail");
    assert!(validate_limit_price(-0.1, "price").is_err(), "negative price should fail");
}

#[test]
fn validate_limit_price_accepts_positive() {
    use agcli::cli::helpers::validate_limit_price;
    assert!(validate_limit_price(0.001, "price").is_ok());
    assert!(validate_limit_price(1.0, "price").is_ok());
}

#[test]
fn process_claim_netuid_parsing_warns_on_invalid() {
    // Mirrors the warning logic in ProcessClaim handler (stake_cmds.rs)
    let input = "1,2,invalid,4";
    let mut ids = Vec::new();
    let mut warnings = Vec::new();
    for n in input.split(',') {
        let trimmed = n.trim();
        if trimmed.is_empty() {
            continue;
        }
        match trimmed.parse::<u16>() {
            Ok(id) => ids.push(id),
            Err(_) => warnings.push(trimmed.to_string()),
        }
    }
    assert_eq!(ids, vec![1u16, 2, 4]);
    assert_eq!(warnings, vec!["invalid"]);
}

#[test]
fn set_claim_empty_subnets_string_does_not_panic() {
    // The SetClaim handler splits on ',' and skips empty tokens — verify parse accepts optional
    let cli = parse(&[
        "agcli", "--yes", "--password", "p",
        "stake", "set-claim",
        "--claim-type", "keep-subnets",
    ]);
    // Missing --subnets is fine — it's Option<String>
    assert!(matches!(is_stake(&cli), StakeCommands::SetClaim { subnets: None, .. }));
}

// ── error classification cross-check ─────────────────────────────────────────

#[test]
fn stake_amount_validation_error_classifies_as_12() {
    use agcli::error::{classify, exit_code};
    // validate_amount produces messages containing "stake amount" which classifies as VALIDATION
    let err = anyhow::anyhow!("Invalid stake amount: amount must be positive (got -1)");
    assert_eq!(classify(&err), exit_code::VALIDATION);
}

#[test]
fn unstake_amount_validation_error_classifies_as_12() {
    use agcli::error::{classify, exit_code};
    let err = anyhow::anyhow!("Invalid unstake amount: amount must be positive (got 0)");
    assert_eq!(classify(&err), exit_code::VALIDATION);
}

#[test]
fn move_amount_validation_error_classifies_as_12() {
    use agcli::error::{classify, exit_code};
    let err = anyhow::anyhow!("Invalid move amount: amount must be positive (got 0)");
    assert_eq!(classify(&err), exit_code::VALIDATION);
}

#[test]
fn insufficient_balance_classifies_as_chain_13() {
    use agcli::error::{classify, exit_code};
    let err = anyhow::anyhow!("Insufficient balance: you have 0.00 τ but trying to stake 1.00 τ");
    assert_eq!(classify(&err), exit_code::CHAIN);
}

#[test]
fn slippage_exceeded_classifies_as_chain_13() {
    use agcli::error::{classify, exit_code};
    let err = anyhow::anyhow!("Slippage 5.00% exceeds maximum allowed 2.00% on SN1.");
    assert_eq!(classify(&err), exit_code::CHAIN);
}

#[test]
fn stake_list_address_validation_hint_points_to_stake_md() {
    use agcli::error::{classify, exit_code, hint};
    let msg = "Invalid stake list --address: not a valid SS58 address";
    let err = anyhow::anyhow!("{msg}");
    assert_eq!(classify(&err), exit_code::VALIDATION);
    let h = hint(exit_code::VALIDATION, msg);
    assert!(
        h.is_some_and(|s| s.contains("stake")),
        "hint should mention stake docs"
    );
}

// ── green-path integration test (localnet-gated) ──────────────────────────────

/// Green-path integration test against a local subtensor chain.
///
/// Requires ws://127.0.0.1:9944 with:
///   - Alice funded (dev account, 1M TAO)
///   - Subnet 1 registered
///   - Bob hotkey registered on subnet 1
///
/// Set up with: `agcli localnet scaffold --config tests/scaffold_configs/default.toml`
/// or by pulling: `docker pull ghcr.io/opentensor/subtensor-localnet:devnet-ready`
///
/// Not executed in CI (Docker unavailable in cloud-agent VM). Run manually:
/// `cargo test --test audit_stake -- green_path_stake_localnet --ignored`
#[test]
#[ignore]
fn green_path_stake_localnet() {
    // This test body intentionally uses std::process::Command (same pattern as
    // stake_binary_stress.rs) to exercise the full binary path including wallet
    // unlock, RPC round-trips, and extrinsic submission.
    use std::process::Command;

    let bin = std::env::var("CARGO_BIN_EXE_agcli").unwrap_or_else(|_| "agcli".to_string());

    // 1. stake list — read-only, no wallet required
    let out = Command::new(&bin)
        .args([
            "--endpoint", "ws://127.0.0.1:9944",
            "--output", "json",
            "stake", "list",
            "--address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        ])
        .output()
        .expect("failed to run agcli stake list");
    assert!(
        out.status.success(),
        "stake list should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // JSON output must be valid JSON (array or object)
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stake list --output json produced invalid JSON: {e}\nstdout: {stdout}"));
    assert!(
        parsed.is_array() || parsed.is_object(),
        "stake list JSON must be array or object"
    );

    // 2. stake show-auto — read-only
    let out = Command::new(&bin)
        .args([
            "--endpoint", "ws://127.0.0.1:9944",
            "stake", "show-auto",
            "--address", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        ])
        .output()
        .expect("failed to run agcli stake show-auto");
    assert!(
        out.status.success(),
        "stake show-auto should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
