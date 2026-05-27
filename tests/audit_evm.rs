use agcli::chain::Client;
use agcli::cli::{Cli, Commands, EvmCommands};
use agcli::localnet::{self, LocalnetConfig};
use clap::Parser;
use sp_core::{sr25519, Pair as _};

fn docker_available() -> bool {
    std::process::Command::new("docker")
        .arg("version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[test]
fn parse_surface_evm_subcommands() {
    let call_cli = Cli::try_parse_from([
        "agcli",
        "evm",
        "call",
        "--source",
        "0x1111111111111111111111111111111111111111",
        "--target",
        "0x2222222222222222222222222222222222222222",
        "--input",
        "0xa9059cbb00000000000000000000000033333333333333333333333333333333333333330000000000000000000000000000000000000000000000000000000000000001",
        "--value",
        "0x0000000000000000000000000000000000000000000000000000000000000000",
        "--gas-limit",
        "120000",
        "--max-fee-per-gas",
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    ])
    .expect("evm call should parse");
    assert!(matches!(
        call_cli.command,
        Commands::Evm(EvmCommands::Call { .. })
    ));

    let withdraw_cli = Cli::try_parse_from([
        "agcli",
        "evm",
        "withdraw",
        "--address",
        "0x1111111111111111111111111111111111111111",
        "--amount",
        "1000000000",
    ])
    .expect("evm withdraw should parse");
    assert!(matches!(
        withdraw_cli.command,
        Commands::Evm(EvmCommands::Withdraw { .. })
    ));
}

#[test]
fn parse_surface_evm_call_defaults() {
    let cli = Cli::try_parse_from([
        "agcli",
        "evm",
        "call",
        "--source",
        "0x1111111111111111111111111111111111111111",
        "--target",
        "0x2222222222222222222222222222222222222222",
    ])
    .expect("evm call should parse with defaults");

    match cli.command {
        Commands::Evm(EvmCommands::Call {
            input,
            gas_limit,
            value,
            max_fee_per_gas,
            ..
        }) => {
            assert_eq!(input, "0x");
            assert_eq!(gas_limit, 21_000);
            assert_eq!(
                value,
                "0x0000000000000000000000000000000000000000000000000000000000000000"
            );
            assert_eq!(
                max_fee_per_gas,
                "0x0000000000000000000000000000000000000000000000000000000000000001"
            );
        }
        _ => panic!("expected evm call variant"),
    }
}

#[tokio::test]
#[ignore = "requires Docker localnet and a running subtensor container"]
async fn green_path_evm_localnet_call_and_withdraw() -> anyhow::Result<()> {
    if !docker_available() {
        return Ok(());
    }

    let cfg = LocalnetConfig {
        container_name: "agcli_localnet_evm_audit".to_string(),
        port: 9954,
        ..LocalnetConfig::default()
    };

    let info = localnet::start(&cfg).await?;
    let result = async {
        let client = Client::connect(&info.endpoint).await?;
        let signer = sr25519::Pair::from_string("//Alice", None)
            .map_err(|e| anyhow::anyhow!("failed to create Alice keypair: {e}"))?;

        // Runtime uses EnsureAddressTruncated for EVM origin checks.
        let mut source = [0u8; 20];
        source.copy_from_slice(&signer.public().0[..20]);

        let mut one_wei = [0u8; 32];
        one_wei[31] = 1;

        let call_hash = client
            .evm_call(
                &signer,
                source,
                source,
                Vec::new(),
                [0u8; 32],
                21_000,
                one_wei,
                None,
                None,
            )
            .await?;
        assert!(
            !call_hash.is_empty(),
            "evm call tx hash should be populated"
        );

        let withdraw_hash = client.evm_withdraw(&signer, source, 0).await?;
        assert!(
            !withdraw_hash.is_empty(),
            "evm withdraw tx hash should be populated"
        );

        Ok::<(), anyhow::Error>(())
    }
    .await;

    let _ = localnet::stop(&cfg.container_name);
    result
}
