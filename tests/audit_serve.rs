//! Audit: `agcli serve` command group — parse-surface + localnet green-path.
//!
//! Run: cargo test --test audit_serve
//!
//! The `#[ignore]` test at the bottom requires a running subtensor localnet
//! (Docker, port 9944). All other tests are pure parse-surface and run offline.

use clap::Parser;

// ─── helpers ───────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<agcli::cli::Cli, clap::Error> {
    agcli::cli::Cli::try_parse_from(args)
}

fn assert_parses(args: &[&str]) {
    parse(args).unwrap_or_else(|e| panic!("expected parse success for {:?}: {}", args, e));
}

fn assert_fails(args: &[&str]) {
    assert!(
        parse(args).is_err(),
        "expected parse failure for {:?}",
        args
    );
}

// ─── serve axon ────────────────────────────────────────────────────────────

#[test]
fn parse_serve_axon_required_args() {
    assert_parses(&[
        "agcli", "serve", "axon", "--netuid", "1", "--ip", "1.2.3.4", "--port", "8091",
    ]);
}

#[test]
fn parse_serve_axon_with_protocol_and_version() {
    assert_parses(&[
        "agcli",
        "serve",
        "axon",
        "--netuid",
        "1",
        "--ip",
        "10.0.0.1",
        "--port",
        "8091",
        "--protocol",
        "4",
        "--version",
        "720",
    ]);
}

#[test]
fn parse_serve_axon_protocol_default_is_4() {
    let cli = parse(&[
        "agcli", "serve", "axon", "--netuid", "1", "--ip", "1.2.3.4", "--port", "8091",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Serve(agcli::cli::ServeCommands::Axon {
            protocol,
            version,
            netuid,
            ..
        }) => {
            assert_eq!(protocol, 4, "protocol default should be 4");
            assert_eq!(version, 0, "version default should be 0");
            assert_eq!(netuid, 1);
        }
        _ => panic!("expected Serve(Axon)"),
    }
}

#[test]
fn parse_serve_axon_missing_ip_fails() {
    assert_fails(&["agcli", "serve", "axon", "--netuid", "1", "--port", "8091"]);
}

#[test]
fn parse_serve_axon_missing_port_fails() {
    assert_fails(&["agcli", "serve", "axon", "--netuid", "1", "--ip", "1.2.3.4"]);
}

#[test]
fn parse_serve_axon_missing_netuid_fails() {
    assert_fails(&[
        "agcli", "serve", "axon", "--ip", "1.2.3.4", "--port", "8091",
    ]);
}

// no --ip-type flag exists (audit finding: IPv6 not exposed)
#[test]
fn parse_serve_axon_no_ip_type_flag() {
    assert_fails(&[
        "agcli",
        "serve",
        "axon",
        "--netuid",
        "1",
        "--ip",
        "::1",
        "--port",
        "8091",
        "--ip-type",
        "6",
    ]);
}

// ─── serve reset ───────────────────────────────────────────────────────────

#[test]
fn parse_serve_reset_required_netuid() {
    assert_parses(&["agcli", "serve", "reset", "--netuid", "1"]);
}

#[test]
fn parse_serve_reset_missing_netuid_fails() {
    assert_fails(&["agcli", "serve", "reset"]);
}

#[test]
fn parse_serve_reset_netuid_field() {
    let cli = parse(&["agcli", "serve", "reset", "--netuid", "7"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Serve(agcli::cli::ServeCommands::Reset { netuid }) => {
            assert_eq!(netuid, 7);
        }
        _ => panic!("expected Serve(Reset)"),
    }
}

// ─── serve batch-axon ──────────────────────────────────────────────────────

#[test]
fn parse_serve_batch_axon_required_file() {
    assert_parses(&["agcli", "serve", "batch-axon", "--file", "/tmp/axons.json"]);
}

#[test]
fn parse_serve_batch_axon_missing_file_fails() {
    assert_fails(&["agcli", "serve", "batch-axon"]);
}

#[test]
fn parse_serve_batch_axon_file_field() {
    let cli = parse(&["agcli", "serve", "batch-axon", "--file", "/tmp/axons.json"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Serve(agcli::cli::ServeCommands::BatchAxon { file }) => {
            assert_eq!(file, "/tmp/axons.json");
        }
        _ => panic!("expected Serve(BatchAxon)"),
    }
}

// ─── serve prometheus ──────────────────────────────────────────────────────

#[test]
fn parse_serve_prometheus_required_args() {
    assert_parses(&[
        "agcli",
        "serve",
        "prometheus",
        "--netuid",
        "1",
        "--ip",
        "1.2.3.4",
        "--port",
        "9090",
    ]);
}

#[test]
fn parse_serve_prometheus_missing_netuid_fails() {
    // Audit finding: docs example omits --netuid but CLI requires it
    assert_fails(&[
        "agcli",
        "serve",
        "prometheus",
        "--ip",
        "1.2.3.4",
        "--port",
        "9090",
    ]);
}

#[test]
fn parse_serve_prometheus_missing_ip_fails() {
    assert_fails(&[
        "agcli",
        "serve",
        "prometheus",
        "--netuid",
        "1",
        "--port",
        "9090",
    ]);
}

#[test]
fn parse_serve_prometheus_version_default_is_0() {
    let cli = parse(&[
        "agcli",
        "serve",
        "prometheus",
        "--netuid",
        "1",
        "--ip",
        "1.2.3.4",
        "--port",
        "9090",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Serve(agcli::cli::ServeCommands::Prometheus {
            version,
            netuid,
            port,
            ..
        }) => {
            assert_eq!(version, 0, "default version should be 0");
            assert_eq!(netuid, 1);
            assert_eq!(port, 9090);
        }
        _ => panic!("expected Serve(Prometheus)"),
    }
}

// ─── serve axon-tls ────────────────────────────────────────────────────────

#[test]
fn parse_serve_axon_tls_required_args() {
    assert_parses(&[
        "agcli",
        "serve",
        "axon-tls",
        "--netuid",
        "1",
        "--ip",
        "1.2.3.4",
        "--port",
        "8091",
        "--cert",
        "/tmp/cert.pem",
    ]);
}

#[test]
fn parse_serve_axon_tls_missing_cert_fails() {
    assert_fails(&[
        "agcli", "serve", "axon-tls", "--netuid", "1", "--ip", "1.2.3.4", "--port", "8091",
    ]);
}

#[test]
fn parse_serve_axon_tls_missing_netuid_fails() {
    assert_fails(&[
        "agcli",
        "serve",
        "axon-tls",
        "--ip",
        "1.2.3.4",
        "--port",
        "8091",
        "--cert",
        "/tmp/cert.pem",
    ]);
}

#[test]
fn parse_serve_axon_tls_fields() {
    let cli = parse(&[
        "agcli",
        "serve",
        "axon-tls",
        "--netuid",
        "2",
        "--ip",
        "192.168.1.1",
        "--port",
        "8091",
        "--protocol",
        "4",
        "--version",
        "100",
        "--cert",
        "/tmp/cert.der",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Serve(agcli::cli::ServeCommands::AxonTls {
            netuid,
            ip,
            port,
            protocol,
            version,
            cert,
        }) => {
            assert_eq!(netuid, 2);
            assert_eq!(ip, "192.168.1.1");
            assert_eq!(port, 8091);
            assert_eq!(protocol, 4);
            assert_eq!(version, 100);
            assert_eq!(cert, "/tmp/cert.der");
        }
        _ => panic!("expected Serve(AxonTls)"),
    }
}

// ─── global flags interact correctly with serve ────────────────────────────

#[test]
fn parse_serve_axon_with_global_yes_and_wallet() {
    // global wallet flag is --wallet (short -w), hotkey is --hotkey-name (alias --hotkey)
    assert_parses(&[
        "agcli",
        "--yes",
        "--wallet",
        "mywallet",
        "--hotkey-name",
        "myhotkey",
        "serve",
        "axon",
        "--netuid",
        "1",
        "--ip",
        "1.2.3.4",
        "--port",
        "8091",
    ]);
}

#[test]
fn parse_serve_axon_with_network_flag() {
    let cli = parse(&[
        "agcli",
        "--network",
        "test",
        "serve",
        "axon",
        "--netuid",
        "1",
        "--ip",
        "1.2.3.4",
        "--port",
        "8091",
    ])
    .unwrap();
    assert_eq!(cli.network, "test");
}

// ─── unknown subcommand under serve fails ──────────────────────────────────

#[test]
fn parse_serve_unknown_subcommand_fails() {
    assert_fails(&["agcli", "serve", "grpc", "--netuid", "1"]);
}

// ─── validate_ipv4 / validate_port used in handlers ────────────────────────
//
// These tests exercise the public helper functions that the serve handler calls
// before touching the wallet or chain. They run fully offline.

#[test]
fn helper_validate_ipv4_valid() {
    let result = agcli::cli::helpers::validate_ipv4("1.2.3.4");
    assert!(
        result.is_ok(),
        "valid IPv4 should parse: {:?}",
        result.err()
    );
}

#[test]
fn helper_validate_ipv4_broadcast_rejected() {
    let result = agcli::cli::helpers::validate_ipv4("255.255.255.255");
    assert!(result.is_err(), "broadcast address should be rejected");
}

#[test]
fn helper_validate_ipv4_unspecified_rejected() {
    let result = agcli::cli::helpers::validate_ipv4("0.0.0.0");
    assert!(result.is_err(), "0.0.0.0 should be rejected");
}

#[test]
fn helper_validate_port_zero_rejected() {
    let result = agcli::cli::helpers::validate_port(0, "axon");
    assert!(result.is_err(), "port 0 should be rejected");
}

#[test]
fn helper_validate_port_valid() {
    assert!(agcli::cli::helpers::validate_port(8091, "axon").is_ok());
    assert!(agcli::cli::helpers::validate_port(65535, "axon").is_ok());
}

#[test]
fn helper_validate_batch_axon_json_minimal() {
    let json = r#"[{"netuid":1,"ip":"1.2.3.4","port":8091}]"#;
    let result = agcli::cli::helpers::validate_batch_axon_json(json);
    assert!(
        result.is_ok(),
        "minimal batch entry should be valid: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn helper_validate_batch_axon_json_multiple_entries() {
    let json = r#"[
        {"netuid":1,"ip":"1.2.3.4","port":8091},
        {"netuid":2,"ip":"10.0.0.1","port":8092,"protocol":4,"version":720}
    ]"#;
    let result = agcli::cli::helpers::validate_batch_axon_json(json);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn helper_validate_batch_axon_json_missing_port_fails() {
    let json = r#"[{"netuid":1,"ip":"1.2.3.4"}]"#;
    let result = agcli::cli::helpers::validate_batch_axon_json(json);
    assert!(result.is_err(), "missing port should fail validation");
}

#[test]
fn helper_validate_batch_axon_json_invalid_ip_fails() {
    let json = r#"[{"netuid":1,"ip":"999.0.0.1","port":8091}]"#;
    let result = agcli::cli::helpers::validate_batch_axon_json(json);
    assert!(result.is_err(), "invalid IP should fail validation");
}

#[test]
fn helper_validate_batch_axon_json_empty_array() {
    // Audit finding: validate_batch_axon_json rejects an empty array (returns an error).
    // The handler would print "Batch serving 0 axon updates" for zero entries, but
    // the validator gate means an empty file fails before reaching that path.
    let json = r#"[]"#;
    let result = agcli::cli::helpers::validate_batch_axon_json(json);
    assert!(
        result.is_err(),
        "empty array is rejected by validate_batch_axon_json"
    );
}

// ─── localnet green-path (requires Docker + running subtensor localnet) ─────
//
// Gated with #[ignore]: run with `cargo test --test audit_serve -- --ignored`
// after starting localnet via `agcli localnet start` or Docker directly.
//
// What this exercises:
//   1. Connect to ws://127.0.0.1:9944
//   2. Create a temporary coldkey + hotkey pair via Wallet API
//   3. Register the hotkey on SN0 (root net is always present)
//   4. Call serve_axon and verify the Tx hash is returned
//   5. Call serve_prometheus and verify the Tx hash is returned
//
// Known localnet constraints:
//   - Root net (netuid=0) always exists; other subnets must be created first.
//   - The localnet Alice/Bob accounts have balances; registration requires burn.
//   - Docker is not installed in the cloud-agent VM; this test is left ignored.

#[ignore]
#[tokio::test]
async fn green_path_serve_axon_localnet() {
    let url = "ws://127.0.0.1:9944";
    let client = agcli::chain::Client::connect(url)
        .await
        .expect("connect to localnet");

    // Use Alice's well-known test mnemonic so we don't need to register first.
    // Alice is pre-funded and registered in the localnet genesis.
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    let pair = agcli::wallet::keypair::pair_from_mnemonic(mnemonic).expect("derive alice pair");

    let ip_u128 = agcli::cli::helpers::validate_ipv4("1.2.3.4").expect("parse IP");
    let axon = agcli::types::chain_data::AxonInfo {
        block: 0,
        version: 1,
        ip: ip_u128.to_string(),
        port: 8091,
        ip_type: 4,
        protocol: 4,
    };

    let hash = client
        .serve_axon(&pair, agcli::types::network::NetUid(0), &axon)
        .await
        .expect("serve_axon on root net");
    assert!(!hash.is_empty(), "tx hash should be non-empty: {hash}");

    let hash2 = client
        .serve_prometheus(&pair, agcli::types::network::NetUid(0), 0, ip_u128, 9090, 4)
        .await
        .expect("serve_prometheus on root net");
    assert!(
        !hash2.is_empty(),
        "prometheus tx hash should be non-empty: {hash2}"
    );
}
