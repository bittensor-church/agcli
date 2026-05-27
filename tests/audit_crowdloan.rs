//! Crowdloan CLI audit coverage.

use agcli::cli::{Cli, Commands, CrowdloanCommands};
use clap::Parser;

fn variant_name(cmd: &CrowdloanCommands) -> &'static str {
    match cmd {
        CrowdloanCommands::Create { .. } => "create",
        CrowdloanCommands::Contribute { .. } => "contribute",
        CrowdloanCommands::Withdraw { .. } => "withdraw",
        CrowdloanCommands::Finalize { .. } => "finalize",
        CrowdloanCommands::Refund { .. } => "refund",
        CrowdloanCommands::Dissolve { .. } => "dissolve",
        CrowdloanCommands::UpdateCap { .. } => "update-cap",
        CrowdloanCommands::UpdateEnd { .. } => "update-end",
        CrowdloanCommands::UpdateMinContribution { .. } => "update-min-contribution",
        CrowdloanCommands::List => "list",
        CrowdloanCommands::Info { .. } => "info",
        CrowdloanCommands::Contributors { .. } => "contributors",
    }
}

#[test]
fn parse_surface_all_crowdloan_subcommands() {
    let cases: [(&str, &[&str]); 12] = [
        (
            "create",
            &[
                "agcli",
                "crowdloan",
                "create",
                "--deposit",
                "25.0",
                "--min-contribution",
                "1.0",
                "--cap",
                "100.0",
                "--end-block",
                "250000",
                "--target",
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            ],
        ),
        (
            "contribute",
            &[
                "agcli",
                "crowdloan",
                "contribute",
                "--crowdloan-id",
                "7",
                "--amount",
                "5.5",
            ],
        ),
        (
            "withdraw",
            &["agcli", "crowdloan", "withdraw", "--crowdloan-id", "7"],
        ),
        (
            "finalize",
            &["agcli", "crowdloan", "finalize", "--crowdloan-id", "7"],
        ),
        (
            "refund",
            &["agcli", "crowdloan", "refund", "--crowdloan-id", "7"],
        ),
        (
            "dissolve",
            &["agcli", "crowdloan", "dissolve", "--crowdloan-id", "7"],
        ),
        (
            "update-cap",
            &[
                "agcli",
                "crowdloan",
                "update-cap",
                "--crowdloan-id",
                "7",
                "--cap",
                "150.0",
            ],
        ),
        (
            "update-end",
            &[
                "agcli",
                "crowdloan",
                "update-end",
                "--crowdloan-id",
                "7",
                "--end-block",
                "350000",
            ],
        ),
        (
            "update-min-contribution",
            &[
                "agcli",
                "crowdloan",
                "update-min-contribution",
                "--crowdloan-id",
                "7",
                "--min-contribution",
                "2.0",
            ],
        ),
        ("list", &["agcli", "crowdloan", "list"]),
        ("info", &["agcli", "crowdloan", "info", "--crowdloan-id", "7"]),
        (
            "contributors",
            &["agcli", "crowdloan", "contributors", "--crowdloan-id", "7"],
        ),
    ];

    for (expected, args) in cases {
        let cli = Cli::try_parse_from(args)
            .unwrap_or_else(|err| panic!("failed to parse crowdloan case '{expected}': {err}"));
        let parsed = match cli.command {
            Commands::Crowdloan(cmd) => cmd,
            other => panic!("expected crowdloan command for '{expected}', got {other:?}"),
        };
        assert_eq!(variant_name(&parsed), expected);
    }
}

#[tokio::test]
#[ignore = "requires docker + local subtensor runtime"]
async fn green_path_crowdloan_localnet() -> anyhow::Result<()> {
    use sp_core::Pair as _;
    use std::process::Command;

    if Command::new("docker").arg("version").output().is_err() {
        eprintln!("docker is unavailable; skipping localnet crowdloan green-path");
        return Ok(());
    }

    let cfg = agcli::localnet::LocalnetConfig {
        container_name: "agcli_crowdloan_audit".to_string(),
        port: 9974,
        wait_timeout: 120,
        ..Default::default()
    };

    let _ = agcli::localnet::stop(&cfg.container_name);

    let run_result = async {
        let info = agcli::localnet::start(&cfg).await?;
        let client = agcli::Client::connect(&info.endpoint).await?;

        let alice = sp_core::sr25519::Pair::from_string("//Alice", None)
            .map_err(|e| anyhow::anyhow!("failed to derive //Alice: {e}"))?;
        let bob = sp_core::sr25519::Pair::from_string("//Bob", None)
            .map_err(|e| anyhow::anyhow!("failed to derive //Bob: {e}"))?;

        let head = client.get_block_number().await?;
        let end_block = (head as u32).saturating_add(200);

        let _create_tx = client
            .crowdloan_create(
                &alice,
                agcli::Balance::from_tao(10.0).rao(),
                agcli::Balance::from_tao(1.0).rao(),
                agcli::Balance::from_tao(20.0).rao(),
                end_block,
                None,
            )
            .await?;

        let _contribute_tx = client
            .crowdloan_contribute(&bob, 0, agcli::Balance::from_tao(2.0))
            .await?;

        let info = client.get_crowdloan_info(0).await?;
        anyhow::ensure!(info.is_some(), "expected crowdloan #0 to exist");
        Ok::<(), anyhow::Error>(())
    }
    .await;

    let _ = agcli::localnet::stop(&cfg.container_name);
    run_result
}
