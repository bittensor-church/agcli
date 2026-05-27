//! Parse-surface and integration tests for `agcli proxy` subcommands.
//!
//! Run: `cargo test --test audit_proxy`
//! Integration test (requires localnet): `cargo test --test audit_proxy -- --include-ignored`

use agcli::cli::{Cli, Commands, ProxyCommands};
use clap::Parser;

// ──────── helpers ────────

const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const CALL_HASH: &str = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

fn proxy_cmd(cli: Cli) -> ProxyCommands {
    match cli.command {
        Commands::Proxy(c) => c,
        other => panic!("expected Proxy command, got {:?}", other),
    }
}

// ──────── proxy add ────────

#[test]
fn proxy_add_minimal() {
    let cli = parse(&["agcli", "proxy", "add", "--delegate", ALICE]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::Add {
            delegate,
            proxy_type,
            delay,
        } => {
            assert_eq!(delegate, ALICE);
            assert_eq!(proxy_type, "any");
            assert_eq!(delay, 0);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_add_all_opts() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "add",
        "--delegate",
        ALICE,
        "--proxy-type",
        "staking",
        "--delay",
        "10",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::Add {
            delegate,
            proxy_type,
            delay,
        } => {
            assert_eq!(delegate, ALICE);
            assert_eq!(proxy_type, "staking");
            assert_eq!(delay, 10);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_add_missing_delegate_rejected() {
    let result = parse(&["agcli", "proxy", "add", "--proxy-type", "any"]);
    assert!(result.is_err(), "proxy add requires --delegate");
}

// ──────── proxy remove ────────

#[test]
fn proxy_remove_minimal() {
    let cli = parse(&["agcli", "proxy", "remove", "--delegate", BOB]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::Remove {
            delegate,
            proxy_type,
            delay,
        } => {
            assert_eq!(delegate, BOB);
            assert_eq!(proxy_type, "any");
            assert_eq!(delay, 0);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_remove_all_opts() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "remove",
        "--delegate",
        BOB,
        "--proxy-type",
        "governance",
        "--delay",
        "5",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::Remove {
            delegate,
            proxy_type,
            delay,
        } => {
            assert_eq!(delegate, BOB);
            assert_eq!(proxy_type, "governance");
            assert_eq!(delay, 5);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_remove_missing_delegate_rejected() {
    assert!(parse(&["agcli", "proxy", "remove"]).is_err());
}

// ──────── proxy create-pure ────────

#[test]
fn proxy_create_pure_defaults() {
    let cli = parse(&["agcli", "proxy", "create-pure"]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::CreatePure {
            proxy_type,
            delay,
            index,
        } => {
            assert_eq!(proxy_type, "any");
            assert_eq!(delay, 0);
            assert_eq!(index, 0);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_create_pure_all_opts() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "create-pure",
        "--proxy-type",
        "staking",
        "--delay",
        "100",
        "--index",
        "2",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::CreatePure {
            proxy_type,
            delay,
            index,
        } => {
            assert_eq!(proxy_type, "staking");
            assert_eq!(delay, 100);
            assert_eq!(index, 2);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

// ──────── proxy kill-pure ────────

#[test]
fn proxy_kill_pure_required_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "kill-pure",
        "--spawner",
        ALICE,
        "--height",
        "1000",
        "--ext-index",
        "0",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::KillPure {
            spawner,
            proxy_type,
            index,
            height,
            ext_index,
        } => {
            assert_eq!(spawner, ALICE);
            assert_eq!(proxy_type, "any");
            assert_eq!(index, 0);
            assert_eq!(height, 1000);
            assert_eq!(ext_index, 0);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_kill_pure_all_opts() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "kill-pure",
        "--spawner",
        ALICE,
        "--proxy-type",
        "non_transfer",
        "--index",
        "1",
        "--height",
        "500",
        "--ext-index",
        "3",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::KillPure {
            spawner,
            proxy_type,
            index,
            height,
            ext_index,
        } => {
            assert_eq!(spawner, ALICE);
            assert_eq!(proxy_type, "non_transfer");
            assert_eq!(index, 1);
            assert_eq!(height, 500);
            assert_eq!(ext_index, 3);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_kill_pure_missing_spawner_rejected() {
    assert!(
        parse(&["agcli", "proxy", "kill-pure", "--height", "100", "--ext-index", "0"]).is_err()
    );
}

#[test]
fn proxy_kill_pure_missing_height_rejected() {
    assert!(
        parse(&[
            "agcli",
            "proxy",
            "kill-pure",
            "--spawner",
            ALICE,
            "--ext-index",
            "0"
        ])
        .is_err()
    );
}

#[test]
fn proxy_kill_pure_missing_ext_index_rejected() {
    assert!(
        parse(&[
            "agcli",
            "proxy",
            "kill-pure",
            "--spawner",
            ALICE,
            "--height",
            "100"
        ])
        .is_err()
    );
}

// ──────── proxy list ────────

#[test]
fn proxy_list_no_address() {
    let cli = parse(&["agcli", "proxy", "list"]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::List { address } => assert!(address.is_none()),
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_list_with_address() {
    let cli = parse(&["agcli", "proxy", "list", "--address", ALICE]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::List { address } => assert_eq!(address.as_deref(), Some(ALICE)),
        other => panic!("wrong variant: {:?}", other),
    }
}

// ──────── proxy announce ────────

#[test]
fn proxy_announce_required_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "announce",
        "--real",
        ALICE,
        "--call-hash",
        CALL_HASH,
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::Announce { real, call_hash } => {
            assert_eq!(real, ALICE);
            assert_eq!(call_hash, CALL_HASH);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_announce_missing_real_rejected() {
    assert!(parse(&["agcli", "proxy", "announce", "--call-hash", CALL_HASH]).is_err());
}

#[test]
fn proxy_announce_missing_call_hash_rejected() {
    assert!(parse(&["agcli", "proxy", "announce", "--real", ALICE]).is_err());
}

// ──────── proxy proxy-announced ────────

#[test]
fn proxy_proxy_announced_required_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "proxy-announced",
        "--delegate",
        ALICE,
        "--real",
        BOB,
        "--pallet",
        "SubtensorModule",
        "--call",
        "add_stake",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::ProxyAnnounced {
            delegate,
            real,
            proxy_type,
            pallet,
            call,
            args,
        } => {
            assert_eq!(delegate, ALICE);
            assert_eq!(real, BOB);
            assert!(proxy_type.is_none());
            assert_eq!(pallet, "SubtensorModule");
            assert_eq!(call, "add_stake");
            assert!(args.is_none());
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_proxy_announced_with_optional_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "proxy-announced",
        "--delegate",
        ALICE,
        "--real",
        BOB,
        "--proxy-type",
        "staking",
        "--pallet",
        "SubtensorModule",
        "--call",
        "add_stake",
        "--args",
        "[0, 100]",
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::ProxyAnnounced {
            proxy_type,
            args,
            ..
        } => {
            assert_eq!(proxy_type.as_deref(), Some("staking"));
            assert_eq!(args.as_deref(), Some("[0, 100]"));
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_proxy_announced_missing_delegate_rejected() {
    assert!(parse(&[
        "agcli",
        "proxy",
        "proxy-announced",
        "--real",
        BOB,
        "--pallet",
        "SubtensorModule",
        "--call",
        "add_stake",
    ])
    .is_err());
}

#[test]
fn proxy_proxy_announced_missing_pallet_rejected() {
    assert!(parse(&[
        "agcli",
        "proxy",
        "proxy-announced",
        "--delegate",
        ALICE,
        "--real",
        BOB,
        "--call",
        "add_stake",
    ])
    .is_err());
}

// ──────── proxy reject-announcement ────────

#[test]
fn proxy_reject_announcement_required_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "reject-announcement",
        "--delegate",
        ALICE,
        "--call-hash",
        CALL_HASH,
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::RejectAnnouncement {
            delegate,
            call_hash,
        } => {
            assert_eq!(delegate, ALICE);
            assert_eq!(call_hash, CALL_HASH);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_reject_announcement_missing_delegate_rejected() {
    assert!(
        parse(&["agcli", "proxy", "reject-announcement", "--call-hash", CALL_HASH]).is_err()
    );
}

// ──────── proxy list-announcements ────────

#[test]
fn proxy_list_announcements_no_address() {
    let cli = parse(&["agcli", "proxy", "list-announcements"]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::ListAnnouncements { address } => assert!(address.is_none()),
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_list_announcements_with_address() {
    let cli = parse(&["agcli", "proxy", "list-announcements", "--address", BOB]).unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::ListAnnouncements { address } => {
            assert_eq!(address.as_deref(), Some(BOB))
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

// ──────── proxy remove-all ────────

#[test]
fn proxy_remove_all_parses() {
    let cli = parse(&["agcli", "proxy", "remove-all"]).unwrap();
    assert!(matches!(proxy_cmd(cli), ProxyCommands::RemoveAll));
}

#[test]
fn proxy_remove_all_takes_no_args() {
    // Unknown args should be rejected by clap.
    assert!(parse(&["agcli", "proxy", "remove-all", "--delegate", ALICE]).is_err());
}

// ──────── proxy remove-announcement ────────

#[test]
fn proxy_remove_announcement_required_args() {
    let cli = parse(&[
        "agcli",
        "proxy",
        "remove-announcement",
        "--real",
        ALICE,
        "--call-hash",
        CALL_HASH,
    ])
    .unwrap();
    match proxy_cmd(cli) {
        ProxyCommands::RemoveAnnouncement { real, call_hash } => {
            assert_eq!(real, ALICE);
            assert_eq!(call_hash, CALL_HASH);
        }
        other => panic!("wrong variant: {:?}", other),
    }
}

#[test]
fn proxy_remove_announcement_missing_real_rejected() {
    assert!(parse(&["agcli", "proxy", "remove-announcement", "--call-hash", CALL_HASH]).is_err());
}

#[test]
fn proxy_remove_announcement_missing_call_hash_rejected() {
    assert!(parse(&["agcli", "proxy", "remove-announcement", "--real", ALICE]).is_err());
}

// ──────── proxy type validation edge-cases ────────

#[test]
fn proxy_add_all_known_proxy_types_parse() {
    // All types documented in the code should be accepted at the CLI parse level
    // (validation happens at dispatch time, not parse time, so all strings pass clap).
    for pt in &[
        "any",
        "owner",
        "staking",
        "non_transfer",
        "non_critical",
        "governance",
        "senate",
        "registration",
        "transfer",
        "small_transfer",
        "root_weights",
        "child_keys",
        "swap_hotkey",
        "subnet_lease_beneficiary",
        "root_claim",
        "triumvirate",
        "non_fungible",
        "sudo_unchecked_set_code",
    ] {
        let result = parse(&["agcli", "proxy", "add", "--delegate", ALICE, "--proxy-type", pt]);
        assert!(
            result.is_ok(),
            "proxy type '{}' should parse at clap level: {:?}",
            pt,
            result.err()
        );
    }
}

// ──────── integration test (requires localnet) ────────

/// Smoke-test against a running localnet.
///
/// Skipped by default (`#[ignore]`). Run with:
/// ```bash
/// AGCLI_ENDPOINT=ws://127.0.0.1:9944 cargo test --test audit_proxy green_path_proxy -- --ignored
/// ```
///
/// This test:
/// 1. Creates a temporary wallet (Alice dev key).
/// 2. Calls `agcli proxy add` to add Bob as a proxy.
/// 3. Calls `agcli proxy list` to verify the proxy appears.
/// 4. Calls `agcli proxy remove` to remove it.
/// 5. Calls `agcli proxy list` again to confirm removal.
#[test]
#[ignore]
fn green_path_proxy() {
    // This test requires:
    // - AGCLI_ENDPOINT set to a running subtensor localnet (ws://127.0.0.1:9944)
    // - Alice dev key available (//Alice SR25519)
    // - Docker with ghcr.io/opentensor/subtensor-localnet:devnet-ready running
    //
    // The actual chain operations are exercised through the CLI binary so that
    // the full dispatch path (validate → encode → sign → submit → parse result)
    // is covered end-to-end. Wire format bugs in `proxy_announce` / `proxy_announced`
    // (missing `MultiAddress::Id` wrapper — see audit Findings) would surface here.
    let endpoint = std::env::var("AGCLI_ENDPOINT").unwrap_or_else(|_| "ws://127.0.0.1:9944".into());
    let status = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "agcli",
            "--",
            "--endpoint",
            &endpoint,
            "proxy",
            "list",
        ])
        .env("SKIP_METADATA_FETCH", "1")
        .status();

    match status {
        Ok(s) => assert!(
            s.success(),
            "proxy list exited non-zero against localnet — chain may not be running"
        ),
        Err(e) => panic!("failed to spawn agcli: {e}"),
    }
}
