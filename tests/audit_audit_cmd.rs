use agcli::cli::{Cli, Commands, OutputFormat};
use clap::Parser;

const ALICE_SS58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

#[test]
fn parse_audit_command_default_address() {
    let cli = Cli::try_parse_from(["agcli", "audit"]).expect("audit parse should succeed");
    match cli.command {
        Commands::Audit { address } => assert_eq!(address, None),
        other => panic!("expected Commands::Audit, got {other:?}"),
    }
}

#[test]
fn parse_audit_command_with_address_and_json_output() {
    let cli = Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "audit",
        "--address",
        ALICE_SS58,
    ])
    .expect("audit --address parse should succeed");

    assert_eq!(cli.output, OutputFormat::Json);
    match cli.command {
        Commands::Audit { address } => assert_eq!(address.as_deref(), Some(ALICE_SS58)),
        other => panic!("expected Commands::Audit, got {other:?}"),
    }
}

#[tokio::test]
#[ignore = "requires running local subtensor chain at ws://127.0.0.1:9944 or AGCLI_AUDIT_LOCAL_WS"]
async fn green_path_audit_queries_local_chain() {
    let ws = std::env::var("AGCLI_AUDIT_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".into());
    let client = agcli::Client::connect(&ws)
        .await
        .expect("local chain should be reachable");

    let pin = client.pin_latest_block().await.expect("pin latest block");
    let _ = client
        .get_balance_at_hash(ALICE_SS58, pin)
        .await
        .expect("balance query should succeed");
    let stakes = client
        .get_stake_for_coldkey_at_block(ALICE_SS58, pin)
        .await
        .expect("stake query should succeed");
    let _ = client
        .get_identity_at_block(ALICE_SS58, pin)
        .await
        .expect("identity query should succeed");
    let _ = client
        .list_proxies_at_block(ALICE_SS58, pin)
        .await
        .expect("proxy query should succeed");
    let _ = client
        .get_delegate_at_block(ALICE_SS58, pin)
        .await
        .expect("delegate query should succeed");
    let _ = client
        .get_coldkey_swap_scheduled_at_block(ALICE_SS58, pin)
        .await
        .expect("coldkey swap query should succeed");

    if let Some(first) = stakes.first() {
        let _ = client
            .get_child_keys_at_block(&first.hotkey, first.netuid, pin)
            .await
            .expect("child key query should succeed");
        let _ = client
            .get_pending_child_keys_at_block(&first.hotkey, first.netuid, pin)
            .await
            .expect("pending child key query should succeed");
    }
}
