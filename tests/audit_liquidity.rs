use agcli::cli::{Cli, Commands, LiquidityCommands};
use agcli::types::NetUid;
use clap::Parser;

const ALICE_SS58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

fn parse_liquidity(args: &[&str]) -> LiquidityCommands {
    let cli = Cli::try_parse_from(args).unwrap_or_else(|e| {
        panic!("failed to parse args {:?}: {}", args, e);
    });
    match cli.command {
        Commands::Liquidity(cmd) => cmd,
        other => panic!("expected liquidity command, got {:?}", other),
    }
}

#[test]
fn parse_surface_all_liquidity_subcommands() {
    let add = parse_liquidity(&[
        "agcli",
        "--network",
        "local",
        "liquidity",
        "add",
        "--netuid",
        "7",
        "--price-low",
        "0.85",
        "--price-high",
        "1.40",
        "--amount",
        "2500000000",
        "--hotkey-address",
        ALICE_SS58,
    ]);
    match add {
        LiquidityCommands::Add {
            netuid,
            price_low,
            price_high,
            amount,
            hotkey,
        } => {
            assert_eq!(netuid, 7);
            assert!((price_low - 0.85).abs() < f64::EPSILON);
            assert!((price_high - 1.40).abs() < f64::EPSILON);
            assert_eq!(amount, 2_500_000_000);
            assert_eq!(hotkey.as_deref(), Some(ALICE_SS58));
        }
        other => panic!("expected Add, got {:?}", other),
    }

    let remove = parse_liquidity(&[
        "agcli",
        "liquidity",
        "remove",
        "--netuid",
        "7",
        "--position-id",
        "42",
        "--hotkey-address",
        ALICE_SS58,
    ]);
    match remove {
        LiquidityCommands::Remove {
            netuid,
            position_id,
            hotkey,
        } => {
            assert_eq!(netuid, 7);
            assert_eq!(position_id, 42);
            assert_eq!(hotkey.as_deref(), Some(ALICE_SS58));
        }
        other => panic!("expected Remove, got {:?}", other),
    }

    let modify = parse_liquidity(&[
        "agcli",
        "liquidity",
        "modify",
        "--netuid",
        "7",
        "--position-id",
        "42",
        "--delta",
        "-500000",
        "--hotkey-address",
        ALICE_SS58,
    ]);
    match modify {
        LiquidityCommands::Modify {
            netuid,
            position_id,
            delta,
            hotkey,
        } => {
            assert_eq!(netuid, 7);
            assert_eq!(position_id, 42);
            assert_eq!(delta, -500_000);
            assert_eq!(hotkey.as_deref(), Some(ALICE_SS58));
        }
        other => panic!("expected Modify, got {:?}", other),
    }

    let toggle = parse_liquidity(&["agcli", "liquidity", "toggle", "--netuid", "7", "--enable"]);
    match toggle {
        LiquidityCommands::Toggle { netuid, enable } => {
            assert_eq!(netuid, 7);
            assert!(enable);
        }
        other => panic!("expected Toggle, got {:?}", other),
    }
}

#[test]
fn parse_surface_toggle_defaults_to_disable() {
    let toggle = parse_liquidity(&["agcli", "liquidity", "toggle", "--netuid", "9"]);
    match toggle {
        LiquidityCommands::Toggle { netuid, enable } => {
            assert_eq!(netuid, 9);
            assert!(!enable);
        }
        other => panic!("expected Toggle, got {:?}", other),
    }
}

#[tokio::test]
#[ignore = "requires a running local subtensor node and dev keys"]
async fn green_path_liquidity_toggle_local_chain() {
    let endpoint =
        std::env::var("AGCLI_AUDIT_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let netuid = std::env::var("AGCLI_AUDIT_LIQUIDITY_NETUID")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(1);

    let client = agcli::chain::Client::connect(&endpoint)
        .await
        .expect("connect local chain");
    let alice = agcli::wallet::keypair::pair_from_uri("//Alice").expect("derive //Alice key");

    let hash = client
        .toggle_user_liquidity(&alice, NetUid(netuid), true)
        .await
        .expect("toggle user liquidity should succeed on a scaffolded localnet");

    assert!(!hash.is_empty(), "toggle tx hash should be non-empty");
}
