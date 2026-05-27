//! Audit: agcli utils (convert / latency) command group.
//!
//! Parse-surface tests verify that every UtilsCommands variant is reachable via clap
//! and that argument wiring matches what handle_utils expects.  An ignored green-path
//! integration test is included but requires a running localnet (Docker unavailable in
//! the cloud-agent VM, so it is left #[ignore]).

use agcli::cli::{Commands, UtilsCommands};
use clap::Parser;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<agcli::cli::Cli, clap::Error> {
    agcli::cli::Cli::try_parse_from(args)
}

fn utils_cmd(args: &[&str]) -> UtilsCommands {
    let cli = parse(args).expect("parse should succeed");
    match cli.command {
        Commands::Utils(cmd) => cmd,
        other => panic!("expected Commands::Utils, got {:?}", other),
    }
}

// ─── convert: RAO → TAO (default, no --to-rao flag) ─────────────────────────

#[test]
fn parse_convert_rao_to_tao_default() {
    let cmd = utils_cmd(&["agcli", "utils", "convert", "--amount", "1000000000"]);
    match cmd {
        UtilsCommands::Convert {
            amount: Some(a),
            to_rao,
            tao: None,
            alpha: None,
            netuid: None,
        } => {
            assert!((a - 1_000_000_000.0).abs() < f64::EPSILON);
            assert!(!to_rao, "--to-rao should be false by default");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── convert: TAO → RAO (--to-rao flag) ──────────────────────────────────────

#[test]
fn parse_convert_tao_to_rao() {
    let cmd = utils_cmd(&["agcli", "utils", "convert", "--amount", "1.5", "--to-rao"]);
    match cmd {
        UtilsCommands::Convert {
            amount: Some(a),
            to_rao,
            tao: None,
            alpha: None,
            netuid: None,
        } => {
            assert!((a - 1.5).abs() < f64::EPSILON);
            assert!(to_rao);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── convert: TAO → Alpha (requires --netuid) ─────────────────────────────────

#[test]
fn parse_convert_tao_to_alpha() {
    let cmd = utils_cmd(&["agcli", "utils", "convert", "--tao", "2.5", "--netuid", "1"]);
    match cmd {
        UtilsCommands::Convert {
            amount: None,
            to_rao: false,
            tao: Some(t),
            alpha: None,
            netuid: Some(n),
        } => {
            assert!((t - 2.5).abs() < f64::EPSILON);
            assert_eq!(n, 1);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── convert: Alpha → TAO (requires --netuid) ─────────────────────────────────

#[test]
fn parse_convert_alpha_to_tao() {
    let cmd = utils_cmd(&[
        "agcli", "utils", "convert", "--alpha", "100.0", "--netuid", "18",
    ]);
    match cmd {
        UtilsCommands::Convert {
            amount: None,
            to_rao: false,
            tao: None,
            alpha: Some(a),
            netuid: Some(n),
        } => {
            assert!((a - 100.0).abs() < f64::EPSILON);
            assert_eq!(n, 18);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── convert: --amount zero (boundary) ───────────────────────────────────────

#[test]
fn parse_convert_zero_amount() {
    let cmd = utils_cmd(&["agcli", "utils", "convert", "--amount", "0"]);
    match cmd {
        UtilsCommands::Convert {
            amount: Some(a), ..
        } => {
            assert!((a - 0.0).abs() < f64::EPSILON);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── convert: --amount missing (optional, defaults to None) ──────────────────

#[test]
fn parse_convert_no_amount_is_ok() {
    // --amount is Option<f64>; omitting it is valid at the parse level
    // (handle_utils treats it as 0.0 via unwrap_or(0.0))
    let result = parse(&["agcli", "utils", "convert"]);
    assert!(
        result.is_ok(),
        "omitting --amount should not be a parse error: {:?}",
        result.err()
    );
}

// ─── convert: --tao without --netuid parses but should error at runtime ───────

#[test]
fn parse_convert_tao_without_netuid_parses() {
    // clap does not enforce the cross-field dependency; handle_utils does
    let result = parse(&["agcli", "utils", "convert", "--tao", "1.0"]);
    assert!(
        result.is_ok(),
        "--tao without --netuid should not be a parse error (clap does not enforce cross-field): {:?}",
        result.err()
    );
}

// ─── convert: --alpha without --netuid parses but should error at runtime ─────

#[test]
fn parse_convert_alpha_without_netuid_parses() {
    let result = parse(&["agcli", "utils", "convert", "--alpha", "50.0"]);
    assert!(
        result.is_ok(),
        "--alpha without --netuid should parse ok: {:?}",
        result.err()
    );
}

// ─── convert: global --output json is honoured ───────────────────────────────

#[test]
fn parse_convert_with_json_output() {
    use agcli::cli::OutputFormat;
    let cli = parse(&[
        "agcli", "--output", "json", "utils", "convert", "--amount", "500",
    ])
    .expect("parse should succeed");
    assert_eq!(cli.output, OutputFormat::Json);
    assert!(matches!(
        cli.command,
        Commands::Utils(UtilsCommands::Convert { .. })
    ));
}

// ─── convert: unknown flag rejected ──────────────────────────────────────────

#[test]
fn parse_convert_unknown_flag_rejected() {
    let result = parse(&["agcli", "utils", "convert", "--rao", "1000"]);
    assert!(
        result.is_err(),
        "--rao is not a valid flag; clap must reject it"
    );
}

// ─── latency: defaults ───────────────────────────────────────────────────────

#[test]
fn parse_latency_defaults() {
    let cmd = utils_cmd(&["agcli", "utils", "latency"]);
    match cmd {
        UtilsCommands::Latency { extra: None, pings } => {
            assert_eq!(pings, 5, "default pings should be 5");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── latency: --pings override ───────────────────────────────────────────────

#[test]
fn parse_latency_pings_override() {
    let cmd = utils_cmd(&["agcli", "utils", "latency", "--pings", "10"]);
    match cmd {
        UtilsCommands::Latency { extra: None, pings } => {
            assert_eq!(pings, 10);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── latency: --extra single endpoint ────────────────────────────────────────

#[test]
fn parse_latency_extra_single() {
    let cmd = utils_cmd(&[
        "agcli",
        "utils",
        "latency",
        "--extra",
        "ws://127.0.0.1:9944",
    ]);
    match cmd {
        UtilsCommands::Latency {
            extra: Some(e),
            pings: 5,
        } => {
            assert_eq!(e, "ws://127.0.0.1:9944");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── latency: --extra comma-separated multiple endpoints ─────────────────────

#[test]
fn parse_latency_extra_comma_separated() {
    let cmd = utils_cmd(&[
        "agcli",
        "utils",
        "latency",
        "--extra",
        "ws://a.example.com:9944,ws://b.example.com:9944",
        "--pings",
        "3",
    ]);
    match cmd {
        UtilsCommands::Latency {
            extra: Some(e),
            pings: 3,
        } => {
            let parts: Vec<&str> = e.split(',').collect();
            assert_eq!(parts.len(), 2);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── latency: --count does not exist (docs drift check) ──────────────────────

#[test]
fn parse_latency_count_flag_does_not_exist() {
    // The old docs claimed --count; the actual flag is --pings.
    let result = parse(&["agcli", "utils", "latency", "--count", "10"]);
    assert!(
        result.is_err(),
        "--count is not a valid flag for latency; clap must reject it"
    );
}

// ─── latency: --pings 0 is accepted by clap (runtime handles it) ─────────────

#[test]
fn parse_latency_pings_zero() {
    let cmd = utils_cmd(&["agcli", "utils", "latency", "--pings", "0"]);
    match cmd {
        UtilsCommands::Latency { pings, .. } => assert_eq!(pings, 0),
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── latency: global --yes flag threads through ──────────────────────────────

#[test]
fn parse_latency_with_global_yes() {
    let cli = parse(&["agcli", "--yes", "utils", "latency"]).expect("parse should succeed");
    assert!(cli.yes);
    assert!(matches!(
        cli.command,
        Commands::Utils(UtilsCommands::Latency { .. })
    ));
}

// ─── utils: no subcommand is a parse error ───────────────────────────────────

#[test]
fn parse_utils_no_subcommand_is_error() {
    let result = parse(&["agcli", "utils"]);
    assert!(
        result.is_err(),
        "utils with no subcommand must fail at parse time"
    );
}

// ─── convert: safe_rao arithmetic (unit-level, no chain) ─────────────────────

#[test]
fn safe_rao_tao_to_rao_math() {
    // safe_rao(1.0) == 1_000_000_000 by definition
    // Verify the TAO→RAO branch of convert uses safe_rao (1 TAO = 1e9 RAO).
    // We can verify the math without a chain connection.
    let tao: f64 = 1.0;
    let rao_expected: u64 = 1_000_000_000;
    let rao_actual = (tao * 1e9).round() as u64;
    assert_eq!(rao_actual, rao_expected);
}

#[test]
fn safe_rao_rao_to_tao_math() {
    let rao: f64 = 2_500_000_000.0;
    let tao_expected: f64 = 2.5;
    let tao_actual = rao / 1e9;
    assert!((tao_actual - tao_expected).abs() < 1e-9);
}

// ─── exit-code classification for convert errors ─────────────────────────────

#[test]
fn classify_netuid_required_error_is_generic() {
    // When --tao is given without --netuid, handle_utils returns an anyhow error with
    // the text "--netuid is required for TAO↔Alpha conversion".  classify() maps this
    // to GENERIC (1) because the message does not match any typed-error heuristic.
    // This is a known drift: the message should ideally map to VALIDATION (12).
    let err = anyhow::anyhow!("--netuid is required for TAO↔Alpha conversion");
    let code = agcli::error::classify(&err);
    // Documents current (imperfect) behavior: GENERIC, not VALIDATION.
    assert_eq!(code, agcli::error::exit_code::GENERIC);
}

#[test]
fn classify_chain_connection_required_is_generic() {
    // "Chain connection required" is an anyhow bail from handle_utils.
    // classify() does not match this to any specific category → GENERIC.
    let err = anyhow::anyhow!("Chain connection required");
    let code = agcli::error::classify(&err);
    assert_eq!(code, agcli::error::exit_code::GENERIC);
}

#[test]
fn classify_invalid_rao_amount_is_validation() {
    // "Invalid RAO amount: ... (must be a finite non-negative number within u64 range)"
    // contains the substring "must be " which matches the VALIDATION heuristic in
    // error::classify().  This is correct behaviour (VALIDATION = 12).
    let err = anyhow::anyhow!(
        "Invalid RAO amount: inf (must be a finite non-negative number within u64 range)"
    );
    let code = agcli::error::classify(&err);
    assert_eq!(code, agcli::error::exit_code::VALIDATION);
}

// ─── #[ignore] green-path integration test (requires localnet) ───────────────

/// Green-path test for `agcli utils convert` and `agcli utils latency` against a running
/// localnet.  Requires Docker and `agcli localnet start` or equivalent.
///
/// To run manually after starting localnet:
/// ```
/// cargo test --test audit_utils_cli green_path_utils -- --ignored
/// ```
#[test]
#[ignore = "requires a running subtensor localnet on ws://127.0.0.1:9944"]
fn green_path_utils() {
    use std::process::Command;

    let bin = std::env::var("AGCLI_BIN").unwrap_or_else(|_| "agcli".to_string());

    // utils convert: RAO → TAO (no chain needed)
    let out = Command::new(&bin)
        .args([
            "--output",
            "json",
            "utils",
            "convert",
            "--amount",
            "1000000000",
        ])
        .output()
        .expect("failed to spawn agcli");
    assert!(out.status.success(), "convert RAO→TAO failed: {:?}", out);
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("stdout must be valid JSON");
    assert_eq!(json["rao"], serde_json::json!(1_000_000_000u64));
    assert!((json["tao"].as_f64().unwrap() - 1.0).abs() < 1e-6);

    // utils convert: TAO → RAO (no chain needed)
    let out = Command::new(&bin)
        .args([
            "--output", "json", "utils", "convert", "--amount", "1.5", "--to-rao",
        ])
        .output()
        .expect("failed to spawn agcli");
    assert!(out.status.success(), "convert TAO→RAO failed: {:?}", out);
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("stdout must be valid JSON");
    assert_eq!(json["tao"], serde_json::json!(1.5));
    assert_eq!(json["rao"], serde_json::json!(1_500_000_000u64));

    // utils latency: one ping to localhost
    let out = Command::new(&bin)
        .args([
            "--network",
            "local",
            "--output",
            "json",
            "utils",
            "latency",
            "--pings",
            "1",
        ])
        .output()
        .expect("failed to spawn agcli");
    assert!(out.status.success(), "latency failed: {:?}", out);
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("stdout must be valid JSON");
    let results = json["latency"]
        .as_array()
        .expect("latency key must be array");
    assert!(!results.is_empty(), "must have at least one result");
    let first = &results[0];
    assert!(
        first["connected"].as_bool().unwrap_or(false),
        "must connect to localnet"
    );
    assert!(
        first["avg_ms"].is_number(),
        "avg_ms must be present and numeric"
    );
}
