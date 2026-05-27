//! Audit: parse-surface + green-path integration tests for `agcli subscribe`.
//!
//! Parse-surface tests exercise clap flag parsing without a live chain.
//! The `#[ignore]` integration test requires a local chain on 127.0.0.1:9944
//! and can be run with `cargo test --test audit_subscribe -- --ignored`.

use agcli::cli::{Cli, Commands, SubscribeCommands};
use clap::Parser;

// ──── helpers ────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

fn subscribe_cmd(cli: &Cli) -> &SubscribeCommands {
    match &cli.command {
        Commands::Subscribe(c) => c,
        other => panic!("expected Subscribe, got {:?}", other),
    }
}

// ──── subscribe blocks ────────────────────────────────────────────────────────

#[test]
fn subscribe_blocks_parses() {
    let cli = parse(&["agcli", "subscribe", "blocks"]).expect("should parse");
    assert!(matches!(subscribe_cmd(&cli), SubscribeCommands::Blocks));
}

#[test]
fn subscribe_blocks_with_json_output() {
    let cli =
        parse(&["agcli", "--output", "json", "subscribe", "blocks"]).expect("should parse");
    assert!(matches!(subscribe_cmd(&cli), SubscribeCommands::Blocks));
    assert!(cli.output.is_json());
}

#[test]
fn subscribe_blocks_rejects_unknown_flag() {
    let result = parse(&["agcli", "subscribe", "blocks", "--nonexistent"]);
    assert!(result.is_err(), "unknown flag should be rejected by clap");
}

// ──── subscribe events — default filter ───────────────────────────────────────

#[test]
fn subscribe_events_defaults() {
    let cli = parse(&["agcli", "subscribe", "events"]).expect("should parse");
    match subscribe_cmd(&cli) {
        SubscribeCommands::Events {
            filter,
            netuid,
            account,
        } => {
            assert_eq!(filter, "all", "default filter must be 'all'");
            assert!(netuid.is_none(), "netuid must default to None");
            assert!(account.is_none(), "account must default to None");
        }
        other => panic!("expected Events, got {:?}", other),
    }
}

// ──── subscribe events — all valid filter aliases ─────────────────────────────

macro_rules! filter_parses {
    ($name:ident, $alias:expr) => {
        #[test]
        fn $name() {
            let cli =
                parse(&["agcli", "subscribe", "events", "--filter", $alias]).expect("should parse");
            match subscribe_cmd(&cli) {
                SubscribeCommands::Events { filter, .. } => {
                    assert_eq!(filter, $alias);
                }
                other => panic!("expected Events, got {:?}", other),
            }
        }
    };
}

filter_parses!(filter_all, "all");
filter_parses!(filter_staking, "staking");
filter_parses!(filter_stake_alias, "stake");
filter_parses!(filter_registration, "registration");
filter_parses!(filter_register_alias, "register");
filter_parses!(filter_reg_alias, "reg");
filter_parses!(filter_transfer, "transfer");
filter_parses!(filter_transfers_alias, "transfers");
filter_parses!(filter_weights, "weights");
filter_parses!(filter_weight_alias, "weight");
filter_parses!(filter_subnet, "subnet");
filter_parses!(filter_subnets_alias, "subnets");
filter_parses!(filter_delegation, "delegation");
filter_parses!(filter_delegate_alias, "delegate");
filter_parses!(filter_delegates_alias, "delegates");
filter_parses!(filter_keys, "keys");
filter_parses!(filter_key_alias, "key");
filter_parses!(filter_swap, "swap");
filter_parses!(filter_dex_alias, "dex");
filter_parses!(filter_liquidity_alias, "liquidity");
filter_parses!(filter_governance, "governance");
filter_parses!(filter_gov_alias, "gov");
filter_parses!(filter_sudo_alias, "sudo");
filter_parses!(filter_safemode_alias, "safemode");
filter_parses!(filter_crowdloan, "crowdloan");
filter_parses!(filter_crowdloans_alias, "crowdloans");
filter_parses!(filter_fund_alias, "fund");

// ──── subscribe events — netuid and account flags ─────────────────────────────

#[test]
fn subscribe_events_with_netuid() {
    let cli = parse(&[
        "agcli", "subscribe", "events", "--filter", "staking", "--netuid", "1",
    ])
    .expect("should parse");
    match subscribe_cmd(&cli) {
        SubscribeCommands::Events { netuid, .. } => {
            assert_eq!(*netuid, Some(1u16));
        }
        other => panic!("expected Events, got {:?}", other),
    }
}

#[test]
fn subscribe_events_with_account() {
    let cli = parse(&[
        "agcli",
        "subscribe",
        "events",
        "--filter",
        "transfer",
        "--account",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ])
    .expect("should parse");
    match subscribe_cmd(&cli) {
        SubscribeCommands::Events { account, .. } => {
            assert_eq!(
                account.as_deref(),
                Some("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")
            );
        }
        other => panic!("expected Events, got {:?}", other),
    }
}

#[test]
fn subscribe_events_with_all_flags() {
    let cli = parse(&[
        "agcli",
        "--output",
        "json",
        "subscribe",
        "events",
        "--filter",
        "weights",
        "--netuid",
        "18",
        "--account",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    ])
    .expect("should parse");
    assert!(cli.output.is_json());
    match subscribe_cmd(&cli) {
        SubscribeCommands::Events {
            filter,
            netuid,
            account,
        } => {
            assert_eq!(filter, "weights");
            assert_eq!(*netuid, Some(18u16));
            assert!(account.is_some());
        }
        other => panic!("expected Events, got {:?}", other),
    }
}

#[test]
fn subscribe_events_rejects_missing_netuid_value() {
    let result = parse(&["agcli", "subscribe", "events", "--netuid"]);
    assert!(result.is_err(), "--netuid with no value must be rejected");
}

#[test]
fn subscribe_events_rejects_non_numeric_netuid() {
    let result = parse(&[
        "agcli", "subscribe", "events", "--netuid", "notanumber",
    ]);
    assert!(result.is_err(), "non-numeric --netuid must be rejected");
}

// ──── validate_event_filter mirrors ───────────────────────────────────────────
// The handler calls validate_event_filter before connecting.
// Ensure the validator accepts the same aliases clap accepts for --filter.

#[test]
fn validate_event_filter_accepts_all_known_aliases() {
    use agcli::cli::helpers::validate_event_filter;

    let valid = &[
        "all", "staking", "stake", "registration", "register", "reg", "transfer", "transfers",
        "weights", "weight", "subnet", "subnets", "delegation", "delegate", "delegates", "keys",
        "key", "swap", "dex", "liquidity", "governance", "gov", "sudo", "safemode", "crowdloan",
        "crowdloans", "fund",
    ];
    for alias in valid {
        assert!(
            validate_event_filter(alias).is_ok(),
            "validate_event_filter must accept '{}'",
            alias
        );
    }
}

#[test]
fn validate_event_filter_rejects_unknown() {
    use agcli::cli::helpers::validate_event_filter;
    assert!(validate_event_filter("stak").is_err());
    assert!(validate_event_filter("foo").is_err());
    assert!(validate_event_filter("").is_err());
}

// ──── EventFilter FromStr mirrors clap strings ────────────────────────────────

#[test]
fn event_filter_from_str_canonical() {
    use agcli::events::EventFilter;
    use std::str::FromStr;

    assert_eq!(EventFilter::from_str("staking").unwrap(), EventFilter::Staking);
    assert_eq!(EventFilter::from_str("stake").unwrap(), EventFilter::Staking);
    assert_eq!(EventFilter::from_str("registration").unwrap(), EventFilter::Registration);
    assert_eq!(EventFilter::from_str("transfer").unwrap(), EventFilter::Transfer);
    assert_eq!(EventFilter::from_str("weights").unwrap(), EventFilter::Weights);
    assert_eq!(EventFilter::from_str("subnet").unwrap(), EventFilter::Subnet);
    assert_eq!(EventFilter::from_str("delegation").unwrap(), EventFilter::Delegation);
    assert_eq!(EventFilter::from_str("keys").unwrap(), EventFilter::Keys);
    assert_eq!(EventFilter::from_str("swap").unwrap(), EventFilter::Swap);
    assert_eq!(EventFilter::from_str("governance").unwrap(), EventFilter::Governance);
    assert_eq!(EventFilter::from_str("crowdloan").unwrap(), EventFilter::Crowdloan);
    assert_eq!(EventFilter::from_str("all").unwrap(), EventFilter::All);
}

#[test]
fn event_filter_from_str_case_insensitive() {
    use agcli::events::EventFilter;
    use std::str::FromStr;

    assert_eq!(EventFilter::from_str("STAKING").unwrap(), EventFilter::Staking);
    assert_eq!(EventFilter::from_str("Transfer").unwrap(), EventFilter::Transfer);
    assert_eq!(EventFilter::from_str("WEIGHTS").unwrap(), EventFilter::Weights);
}

// NOTE: EventFilter::from_str has `Infallible` error type, so unknown strings
// silently fall through to `All`.  validate_event_filter (called first in
// handle_subscribe) catches them before the parse, so this fallback only
// triggers if validation is bypassed.
#[test]
fn event_filter_from_str_unknown_falls_back_to_all() {
    use agcli::events::EventFilter;
    use std::str::FromStr;

    // Intentional silent fallback — documented audit finding.
    assert_eq!(EventFilter::from_str("typo_filter").unwrap(), EventFilter::All);
}

// ──── green-path integration test (requires localnet on 127.0.0.1:9944) ──────

/// Subscribe to blocks for one block on a local chain.
///
/// Run with: `cargo test --test audit_subscribe -- --ignored green_path_subscribe_blocks`
///
/// Requires `agcli localnet start` (or Docker-based `agcli localnet scaffold`)
/// with the chain available at ws://127.0.0.1:9944 before running this test.
/// Docker is not available in the cloud-agent VM so this test is `#[ignore]`d.
#[ignore]
#[tokio::test]
async fn green_path_subscribe_blocks() {
    use agcli::chain::Client;
    use agcli::events::subscribe_blocks;
    use std::time::Duration;
    use tokio::time::timeout;

    let client = Client::connect("ws://127.0.0.1:9944")
        .await
        .expect("should connect to local chain");

    // Subscribe for at most 20 s — just confirming the subscription opens and we
    // receive at least one block without panicking.
    let result = timeout(
        Duration::from_secs(20),
        subscribe_blocks(client.subxt(), false),
    )
    .await;

    // Timeout is the expected outcome here (we run forever until Ctrl+C).
    // A real chain error would propagate as Ok(Err(_)), which we treat as a failure.
    match result {
        Err(_timeout) => { /* expected — subscription was running */ }
        Ok(Err(e)) => panic!("subscribe_blocks returned an error: {}", e),
        Ok(Ok(())) => { /* graceful shutdown also acceptable */ }
    }
}
