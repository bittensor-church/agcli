use agcli::cli::{Cli, Commands, DrandCommands};
use clap::Parser;

#[test]
fn parse_surface_drand_subcommands() {
    let parse_cases: &[&[&str]] = &[&[
        "agcli",
        "--network",
        "local",
        "--wallet",
        "default",
        "drand",
        "write-pulse",
        "--payload",
        "0x0123456789abcdef",
        "--signature",
        "0xabcdef0123456789",
    ]];

    for args in parse_cases {
        let parsed = Cli::try_parse_from(*args);
        assert!(parsed.is_ok(), "failed to parse {:?}: {:?}", args, parsed.err());
    }

    let parsed = Cli::try_parse_from(parse_cases[0]).expect("drand write-pulse should parse");
    match parsed.command {
        Commands::Drand(DrandCommands::WritePulse { payload, signature }) => {
            assert_eq!(payload, "0x0123456789abcdef");
            assert_eq!(signature, "0xabcdef0123456789");
        }
        other => panic!("expected drand write-pulse command, got: {:?}", other),
    }
}

#[test]
fn parse_drand_write_pulse_requires_payload_and_signature() {
    let missing_payload = Cli::try_parse_from([
        "agcli",
        "drand",
        "write-pulse",
        "--signature",
        "0x01",
    ]);
    assert!(missing_payload.is_err());

    let missing_signature = Cli::try_parse_from(["agcli", "drand", "write-pulse", "--payload", "0x01"]);
    assert!(missing_signature.is_err());
}

#[tokio::test]
#[ignore = "requires a running local chain endpoint"]
async fn green_path_drand_local_chain_round_query() -> anyhow::Result<()> {
    let endpoint = std::env::var("AGCLI_LOCALNET_WS")
        .or_else(|_| std::env::var("AGCLI_ENDPOINT"))
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());

    let client = agcli::Client::connect(&endpoint)
        .await
        .map_err(|e| anyhow::anyhow!("failed to connect to local chain at {endpoint}: {e}"))?;

    let _last_round = client.get_drand_last_round().await.map_err(|e| {
        anyhow::anyhow!("connected to {endpoint}, but drand round query failed: {e}")
    })?;

    Ok(())
}
