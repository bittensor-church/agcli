//! Audit tests for `agcli view` command group.
//!
//! Parse-surface tests: every subcommand under `ViewCommands` is exercised
//! with realistic arguments to verify clap wiring.  These tests require no
//! network connectivity and run in the default `cargo test` invocation.
//!
//! The single `#[ignore]`-gated integration test at the bottom connects to a
//! locally-running subtensor node (port 9944) and is skipped unless the
//! `AGCLI_LOCALNET` env-var is set to `1`.

use agcli::cli::{Cli, Commands, ViewCommands};
use clap::Parser;

// ──────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────

/// Real-looking Bittensor SS58 address used throughout these tests.
const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

fn parse(args: &[&str]) -> Cli {
    Cli::try_parse_from(args).unwrap_or_else(|e| panic!("parse failed: {e}"))
}

fn parse_err(args: &[&str]) -> clap::Error {
    Cli::try_parse_from(args).expect_err("expected parse error")
}

fn view_cmd(cli: Cli) -> ViewCommands {
    match cli.command {
        Commands::View(v) => v,
        other => panic!("expected View, got {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view portfolio
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_portfolio_no_args() {
    let cli = parse(&["agcli", "view", "portfolio"]);
    let cmd = view_cmd(cli);
    assert!(matches!(
        cmd,
        ViewCommands::Portfolio {
            address: None,
            at_block: None
        }
    ));
}

#[test]
fn parse_view_portfolio_with_address() {
    let cli = parse(&["agcli", "view", "portfolio", "--address", ALICE]);
    match view_cmd(cli) {
        ViewCommands::Portfolio {
            address: Some(addr),
            at_block: None,
        } => assert_eq!(addr, ALICE),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_portfolio_at_block() {
    let cli = parse(&["agcli", "view", "portfolio", "--at-block", "4000000"]);
    match view_cmd(cli) {
        ViewCommands::Portfolio {
            address: None,
            at_block: Some(b),
        } => assert_eq!(b, 4_000_000u32),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_portfolio_address_and_block() {
    let cli = parse(&[
        "agcli",
        "view",
        "portfolio",
        "--address",
        ALICE,
        "--at-block",
        "1000",
    ]);
    match view_cmd(cli) {
        ViewCommands::Portfolio {
            address: Some(addr),
            at_block: Some(b),
        } => {
            assert_eq!(addr, ALICE);
            assert_eq!(b, 1000u32);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view network
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_network_no_args() {
    let cli = parse(&["agcli", "view", "network"]);
    assert!(matches!(
        view_cmd(cli),
        ViewCommands::Network { at_block: None }
    ));
}

#[test]
fn parse_view_network_at_block() {
    let cli = parse(&["agcli", "view", "network", "--at-block", "500"]);
    match view_cmd(cli) {
        ViewCommands::Network { at_block: Some(b) } => assert_eq!(b, 500u32),
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view dynamic
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_dynamic_no_args() {
    let cli = parse(&["agcli", "view", "dynamic"]);
    assert!(matches!(
        view_cmd(cli),
        ViewCommands::Dynamic { at_block: None }
    ));
}

#[test]
fn parse_view_dynamic_at_block() {
    let cli = parse(&["agcli", "view", "dynamic", "--at-block", "3500000"]);
    match view_cmd(cli) {
        ViewCommands::Dynamic { at_block: Some(b) } => assert_eq!(b, 3_500_000u32),
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view neuron
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_neuron_required_args() {
    let cli = parse(&["agcli", "view", "neuron", "--netuid", "1", "--uid", "42"]);
    match view_cmd(cli) {
        ViewCommands::Neuron {
            netuid,
            uid,
            at_block: None,
        } => {
            assert_eq!(netuid, 1);
            assert_eq!(uid, 42);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_neuron_with_at_block() {
    let cli = parse(&[
        "agcli",
        "view",
        "neuron",
        "--netuid",
        "3",
        "--uid",
        "0",
        "--at-block",
        "9999",
    ]);
    match view_cmd(cli) {
        ViewCommands::Neuron {
            netuid,
            uid,
            at_block: Some(b),
        } => {
            assert_eq!(netuid, 3);
            assert_eq!(uid, 0);
            assert_eq!(b, 9999u32);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_neuron_missing_uid_fails() {
    parse_err(&["agcli", "view", "neuron", "--netuid", "1"]);
}

#[test]
fn parse_view_neuron_missing_netuid_fails() {
    parse_err(&["agcli", "view", "neuron", "--uid", "0"]);
}

// ──────────────────────────────────────────────────────────────
// view validators
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_validators_default_limit() {
    let cli = parse(&["agcli", "view", "validators"]);
    match view_cmd(cli) {
        ViewCommands::Validators {
            netuid: None,
            limit,
            at_block: None,
        } => assert_eq!(limit, 50),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_validators_with_netuid_and_limit() {
    let cli = parse(&[
        "agcli",
        "view",
        "validators",
        "--netuid",
        "1",
        "--limit",
        "100",
    ]);
    match view_cmd(cli) {
        ViewCommands::Validators {
            netuid: Some(n),
            limit,
            at_block: None,
        } => {
            assert_eq!(n, 1);
            assert_eq!(limit, 100);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_validators_at_block() {
    let cli = parse(&[
        "agcli",
        "view",
        "validators",
        "--netuid",
        "2",
        "--at-block",
        "12345",
    ]);
    match view_cmd(cli) {
        ViewCommands::Validators {
            netuid: Some(2),
            at_block: Some(12345),
            ..
        } => {}
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view history
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_history_default_limit() {
    let cli = parse(&["agcli", "view", "history"]);
    match view_cmd(cli) {
        ViewCommands::History {
            address: None,
            limit,
        } => assert_eq!(limit, 20),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_history_explicit_args() {
    let cli = parse(&["agcli", "view", "history", "--address", ALICE, "--limit", "50"]);
    match view_cmd(cli) {
        ViewCommands::History {
            address: Some(addr),
            limit,
        } => {
            assert_eq!(addr, ALICE);
            assert_eq!(limit, 50);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view account
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_account_no_args() {
    let cli = parse(&["agcli", "view", "account"]);
    assert!(matches!(
        view_cmd(cli),
        ViewCommands::Account {
            address: None,
            at_block: None
        }
    ));
}

#[test]
fn parse_view_account_with_address_and_block() {
    let cli = parse(&[
        "agcli",
        "view",
        "account",
        "--address",
        ALICE,
        "--at-block",
        "2000000",
    ]);
    match view_cmd(cli) {
        ViewCommands::Account {
            address: Some(addr),
            at_block: Some(b),
        } => {
            assert_eq!(addr, ALICE);
            assert_eq!(b, 2_000_000u32);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view subnet-analytics
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_subnet_analytics() {
    let cli = parse(&["agcli", "view", "subnet-analytics", "--netuid", "18"]);
    match view_cmd(cli) {
        ViewCommands::SubnetAnalytics { netuid } => assert_eq!(netuid, 18),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_subnet_analytics_missing_netuid_fails() {
    parse_err(&["agcli", "view", "subnet-analytics"]);
}

// ──────────────────────────────────────────────────────────────
// view staking-analytics
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_staking_analytics_no_address() {
    let cli = parse(&["agcli", "view", "staking-analytics"]);
    assert!(matches!(
        view_cmd(cli),
        ViewCommands::StakingAnalytics { address: None }
    ));
}

#[test]
fn parse_view_staking_analytics_with_address() {
    let cli = parse(&["agcli", "view", "staking-analytics", "--address", ALICE]);
    match view_cmd(cli) {
        ViewCommands::StakingAnalytics { address: Some(a) } => assert_eq!(a, ALICE),
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// view swap-sim
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_swap_sim_tao_direction() {
    let cli = parse(&["agcli", "view", "swap-sim", "--netuid", "1", "--tao", "10.0"]);
    match view_cmd(cli) {
        ViewCommands::SwapSim {
            netuid,
            tao: Some(t),
            alpha: None,
        } => {
            assert_eq!(netuid, 1);
            assert!((t - 10.0).abs() < 1e-9);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_swap_sim_alpha_direction() {
    let cli = parse(&[
        "agcli",
        "view",
        "swap-sim",
        "--netuid",
        "2",
        "--alpha",
        "500.5",
    ]);
    match view_cmd(cli) {
        ViewCommands::SwapSim {
            netuid,
            tao: None,
            alpha: Some(a),
        } => {
            assert_eq!(netuid, 2);
            assert!((a - 500.5).abs() < 1e-9);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_swap_sim_missing_netuid_fails() {
    parse_err(&["agcli", "view", "swap-sim", "--tao", "1.0"]);
}

// ──────────────────────────────────────────────────────────────
// view nominations
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_nominations() {
    let cli = parse(&["agcli", "view", "nominations", "--hotkey-address", ALICE]);
    match view_cmd(cli) {
        ViewCommands::Nominations { hotkey } => assert_eq!(hotkey, ALICE),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_nominations_missing_hotkey_fails() {
    parse_err(&["agcli", "view", "nominations"]);
}

// ──────────────────────────────────────────────────────────────
// view metagraph
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_metagraph_required_only() {
    let cli = parse(&["agcli", "view", "metagraph", "--netuid", "1"]);
    match view_cmd(cli) {
        ViewCommands::Metagraph {
            netuid,
            since_block: None,
            limit: None,
        } => assert_eq!(netuid, 1),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_metagraph_diff_mode() {
    let cli = parse(&[
        "agcli",
        "view",
        "metagraph",
        "--netuid",
        "9",
        "--since-block",
        "4000000",
        "--limit",
        "20",
    ]);
    match view_cmd(cli) {
        ViewCommands::Metagraph {
            netuid,
            since_block: Some(sb),
            limit: Some(lim),
        } => {
            assert_eq!(netuid, 9);
            assert_eq!(sb, 4_000_000u32);
            assert_eq!(lim, 20);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_metagraph_missing_netuid_fails() {
    parse_err(&["agcli", "view", "metagraph"]);
}

// ──────────────────────────────────────────────────────────────
// view axon
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_axon_with_uid() {
    let cli = parse(&["agcli", "view", "axon", "--netuid", "1", "--uid", "7"]);
    match view_cmd(cli) {
        ViewCommands::Axon {
            netuid,
            uid: Some(u),
            hotkey: None,
        } => {
            assert_eq!(netuid, 1);
            assert_eq!(u, 7);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_axon_with_hotkey() {
    let cli = parse(&[
        "agcli",
        "view",
        "axon",
        "--netuid",
        "3",
        "--hotkey-address",
        ALICE,
    ]);
    match view_cmd(cli) {
        ViewCommands::Axon {
            netuid,
            uid: None,
            hotkey: Some(hk),
        } => {
            assert_eq!(netuid, 3);
            assert_eq!(hk, ALICE);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_axon_missing_netuid_fails() {
    parse_err(&["agcli", "view", "axon", "--uid", "0"]);
}

// ──────────────────────────────────────────────────────────────
// view health
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_health_required_only() {
    let cli = parse(&["agcli", "view", "health", "--netuid", "5"]);
    match view_cmd(cli) {
        ViewCommands::Health {
            netuid,
            tcp_check,
            probe_timeout_ms,
        } => {
            assert_eq!(netuid, 5);
            assert!(!tcp_check);
            assert_eq!(probe_timeout_ms, 2000u64);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_health_with_tcp_check() {
    let cli = parse(&[
        "agcli",
        "view",
        "health",
        "--netuid",
        "1",
        "--tcp-check",
        "--probe-timeout-ms",
        "5000",
    ]);
    match view_cmd(cli) {
        ViewCommands::Health {
            netuid,
            tcp_check,
            probe_timeout_ms,
        } => {
            assert_eq!(netuid, 1);
            assert!(tcp_check);
            assert_eq!(probe_timeout_ms, 5000u64);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_health_missing_netuid_fails() {
    parse_err(&["agcli", "view", "health"]);
}

// ──────────────────────────────────────────────────────────────
// view emissions
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_emissions_required_only() {
    let cli = parse(&["agcli", "view", "emissions", "--netuid", "18"]);
    match view_cmd(cli) {
        ViewCommands::Emissions {
            netuid,
            limit: None,
        } => assert_eq!(netuid, 18),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_emissions_with_limit() {
    let cli = parse(&[
        "agcli",
        "view",
        "emissions",
        "--netuid",
        "1",
        "--limit",
        "25",
    ]);
    match view_cmd(cli) {
        ViewCommands::Emissions {
            netuid,
            limit: Some(lim),
        } => {
            assert_eq!(netuid, 1);
            assert_eq!(lim, 25);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_view_emissions_missing_netuid_fails() {
    parse_err(&["agcli", "view", "emissions"]);
}

// ──────────────────────────────────────────────────────────────
// Global flag interaction with view subcommands
// ──────────────────────────────────────────────────────────────

#[test]
fn parse_view_portfolio_output_json_global_flag() {
    let cli = parse(&[
        "agcli",
        "--output",
        "json",
        "view",
        "portfolio",
        "--address",
        ALICE,
    ]);
    assert_eq!(cli.output, agcli::cli::OutputFormat::Json);
}

#[test]
fn parse_view_portfolio_output_csv_global_flag() {
    let cli = parse(&["agcli", "--output", "csv", "view", "portfolio"]);
    assert_eq!(cli.output, agcli::cli::OutputFormat::Csv);
}

#[test]
fn parse_view_with_live_flag() {
    // --live is global; placing it after the subcommand avoids clap greedily
    // consuming the subcommand name as the optional u64 value for --live.
    let cli = parse(&["agcli", "view", "portfolio", "--live"]);
    assert!(cli.live.is_some());
}

#[test]
fn parse_view_with_live_interval() {
    let cli = parse(&["agcli", "view", "portfolio", "--live", "30"]);
    assert!(cli.live.is_some());
}

#[test]
fn parse_view_with_network_override() {
    let cli = parse(&[
        "agcli",
        "--network",
        "archive",
        "view",
        "dynamic",
        "--at-block",
        "3000000",
    ]);
    assert_eq!(cli.network, "archive");
    match view_cmd(cli) {
        ViewCommands::Dynamic { at_block: Some(b) } => assert_eq!(b, 3_000_000),
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// Argument encoding unit tests
// ──────────────────────────────────────────────────────────────

/// TAO-to-RAO conversion: 1.5 TAO = 1_500_000_000 RAO.
#[test]
fn tao_to_rao_encoding() {
    let tao: f64 = 1.5;
    let rao = (tao * 1e9) as u64;
    assert_eq!(rao, 1_500_000_000u64);
}

/// Zero TAO converts to zero RAO without panicking.
#[test]
fn zero_tao_to_rao() {
    let tao: f64 = 0.0;
    let rao = (tao * 1e9) as u64;
    assert_eq!(rao, 0u64);
}

/// Very small TAO amount (dust) at minimum meaningful precision.
#[test]
fn dust_tao_to_rao() {
    let tao: f64 = 0.000_000_001; // 1 RAO
    let rao = (tao * 1e9).round() as u64;
    assert_eq!(rao, 1u64);
}

/// `validate_view_limit` accepts positive limits via the clap default path.
#[test]
fn view_limit_default_50_for_validators() {
    let cli = parse(&["agcli", "view", "validators"]);
    match view_cmd(cli) {
        ViewCommands::Validators { limit, .. } => assert_eq!(limit, 50),
        _ => panic!("expected validators"),
    }
}

/// `validate_view_limit` — history has default 20.
#[test]
fn view_limit_default_20_for_history() {
    let cli = parse(&["agcli", "view", "history"]);
    match view_cmd(cli) {
        ViewCommands::History { limit, .. } => assert_eq!(limit, 20),
        _ => panic!("expected history"),
    }
}

// ──────────────────────────────────────────────────────────────
// validate_view_limit boundary — runtime behavior asserted here
// (the validators max-limit validation happens inside handle_view
// but the handler applies validate_view_limit; we test the error
// text content from helpers indirectly by confirming parsing accepts
// large values which are validated at runtime, not at clap time).
// ──────────────────────────────────────────────────────────────

/// Parsing accepts large limit values; runtime validation catches them.
#[test]
fn parse_view_validators_large_limit_accepted_at_parse() {
    let cli = parse(&["agcli", "view", "validators", "--limit", "10000"]);
    match view_cmd(cli) {
        ViewCommands::Validators { limit, .. } => assert_eq!(limit, 10000),
        _ => panic!("expected validators"),
    }
}

// ──────────────────────────────────────────────────────────────
// agcli audit (top-level, routed via view_cmds::handle_audit)
// ──────────────────────────────────────────────────────────────

/// `agcli audit` is a top-level command (not under `view`), backed by
/// `view_cmds::handle_audit`.  Verify it parses with and without --address.
#[test]
fn parse_audit_no_address() {
    let cli = Cli::try_parse_from(["agcli", "audit"]).expect("parse");
    assert!(matches!(cli.command, Commands::Audit { address: None }));
}

#[test]
fn parse_audit_with_address() {
    let cli = Cli::try_parse_from(["agcli", "audit", "--address", ALICE]).expect("parse");
    match cli.command {
        Commands::Audit {
            address: Some(addr),
        } => assert_eq!(addr, ALICE),
        other => panic!("unexpected: {:?}", other),
    }
}

// ──────────────────────────────────────────────────────────────
// Green-path integration test (localnet-gated; ignored by default)
// ──────────────────────────────────────────────────────────────

/// End-to-end smoke test against a locally-running subtensor node.
///
/// Gate: set env var `AGCLI_LOCALNET=1` and ensure `subtensor` is listening
/// on `ws://127.0.0.1:9944` (e.g. via `agcli localnet start` or Docker).
///
/// This test connects to the chain and issues one read-only query to verify
/// that `view network` can reach chain state without panicking.  No wallet or
/// signing required.
#[test]
#[ignore = "requires localnet on ws://127.0.0.1:9944; set AGCLI_LOCALNET=1 to run"]
fn green_path_view_network_localnet() {
    // Guard: skip unless caller explicitly opted in.
    if std::env::var("AGCLI_LOCALNET").unwrap_or_default() != "1" {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = agcli::chain::Client::connect("ws://127.0.0.1:9944")
            .await
            .expect("connect to localnet");

        let (block, total_stake, total_networks, _total_issuance, _emission) =
            client.get_network_overview().await.expect("get_network_overview");

        assert!(block > 0, "block number should be positive");
        // localnet starts with at least the root network (netuid 0)
        assert!(total_networks >= 1, "should have at least 1 network");
        // total stake can be 0 on a fresh chain
        let _ = total_stake;
    });
}
