use agcli::cli::Cli;
use agcli::localnet::{self, LocalnetConfig, DEFAULT_IMAGE};
use clap::Parser;

fn assert_parses(args: &[&str]) {
    let parsed = Cli::try_parse_from(args);
    assert!(
        parsed.is_ok(),
        "failed to parse {:?}: {:?}",
        args,
        parsed.err()
    );
}

#[test]
fn parse_surface_localnet_start() {
    assert_parses(&[
        "agcli",
        "localnet",
        "start",
        "--image",
        "ghcr.io/opentensor/subtensor-localnet:devnet-ready",
        "--container",
        "audit-localnet",
        "--port",
        "9950",
        "--wait",
        "false",
        "--timeout",
        "90",
    ]);
}

#[test]
fn parse_surface_localnet_stop() {
    assert_parses(&["agcli", "localnet", "stop", "--container", "audit-localnet"]);
}

#[test]
fn parse_surface_localnet_status() {
    assert_parses(&[
        "agcli",
        "localnet",
        "status",
        "--container",
        "audit-localnet",
        "--port",
        "9950",
    ]);
}

#[test]
fn parse_surface_localnet_reset() {
    assert_parses(&[
        "agcli",
        "localnet",
        "reset",
        "--image",
        "ghcr.io/opentensor/subtensor-localnet:devnet-ready",
        "--container",
        "audit-localnet",
        "--port",
        "9950",
        "--timeout",
        "120",
    ]);
}

#[test]
fn parse_surface_localnet_logs() {
    assert_parses(&[
        "agcli",
        "localnet",
        "logs",
        "--container",
        "audit-localnet",
        "--tail",
        "200",
    ]);
}

#[test]
fn parse_surface_localnet_scaffold() {
    assert_parses(&[
        "agcli",
        "localnet",
        "scaffold",
        "--config",
        "examples/scaffold.toml",
        "--image",
        "ghcr.io/opentensor/subtensor-localnet:devnet-ready",
        "--port",
        "9951",
        "--no-start",
    ]);
}

fn docker_available() -> bool {
    std::process::Command::new("docker")
        .args(["version"])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

struct ContainerGuard(String);

impl Drop for ContainerGuard {
    fn drop(&mut self) {
        let _ = localnet::stop(&self.0);
    }
}

#[tokio::test]
#[ignore = "requires Docker daemon and local subtensor image"]
async fn green_path_localnet_start_status_stop() -> anyhow::Result<()> {
    if !docker_available() {
        eprintln!("[audit_localnet] docker unavailable; skipping ignored integration test.");
        return Ok(());
    }

    let container = format!("agcli_audit_localnet_{}", std::process::id());
    let port = 9952;
    let _ = localnet::stop(&container);

    let cfg = LocalnetConfig {
        image: DEFAULT_IMAGE.to_string(),
        container_name: container.clone(),
        port,
        wait: true,
        wait_timeout: 120,
    };
    let info = localnet::start(&cfg).await?;
    let _guard = ContainerGuard(container.clone());

    assert_eq!(info.container_name, container);
    assert_eq!(info.port, port);
    assert!(info.block_height > 0);

    let status = localnet::status(&cfg.container_name, cfg.port).await?;
    assert!(status.running);
    assert_eq!(status.endpoint.as_deref(), Some("ws://127.0.0.1:9952"));
    assert!(status.block_height.is_some());

    localnet::stop(&cfg.container_name)?;
    let status_after_stop = localnet::status(&cfg.container_name, cfg.port).await?;
    assert!(!status_after_stop.running);

    Ok(())
}
