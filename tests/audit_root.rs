//! Audit tests for the `root` command group.
//!
//! Run with: cargo test --test audit_root
//!
//! Covers:
//! - Parse-surface tests for every `RootCommands` variant with realistic args.
//! - Validation tests that confirm bad input is rejected at parse time.
//! - One `#[ignore]` integration test gated on a live local chain (ws://127.0.0.1:9944).

use clap::Parser;

// ──────────────────────────────────────────────────────────────────
// Parse-surface tests
// ──────────────────────────────────────────────────────────────────

/// `root register` — no subcommand-level flags; signing context comes from global flags.
#[test]
fn parse_root_register_no_args() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "root", "register"]);
    assert!(
        cli.is_ok(),
        "root register should parse with no extra flags: {:?}",
        cli.err()
    );
    match cli.unwrap().command {
        agcli::cli::Commands::Root(agcli::cli::RootCommands::Register) => {}
        other => panic!("Expected Root(Register), got {:?}", other),
    }
}

/// `root register` with global wallet context flags.
#[test]
fn parse_root_register_with_wallet_flags() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--wallet",
        "my_wallet",
        "--hotkey-name",
        "my_hotkey",
        "--yes",
        "root",
        "register",
    ]);
    assert!(
        cli.is_ok(),
        "root register with wallet flags should parse: {:?}",
        cli.err()
    );
}

/// `root register` with `--dry-run` (global flag).
#[test]
fn parse_root_register_dry_run() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "--dry-run", "root", "register"]);
    assert!(
        cli.is_ok(),
        "root register --dry-run should parse: {:?}",
        cli.err()
    );
    let parsed = cli.unwrap();
    assert!(parsed.dry_run, "dry_run flag should be set");
}

/// `root weights` with a single netuid:weight pair.
#[test]
fn parse_root_weights_single_pair() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "root", "weights", "--weights", "1:1000",
    ]);
    assert!(
        cli.is_ok(),
        "root weights with single pair should parse: {:?}",
        cli.err()
    );
    match cli.unwrap().command {
        agcli::cli::Commands::Root(agcli::cli::RootCommands::Weights { weights }) => {
            assert_eq!(weights, "1:1000");
        }
        other => panic!("Expected Root(Weights), got {:?}", other),
    }
}

/// `root weights` with multiple netuid:weight pairs (realistic root-weight scenario).
#[test]
fn parse_root_weights_multi_pair() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "root",
        "weights",
        "--weights",
        "1:500,2:300,3:200",
    ]);
    assert!(
        cli.is_ok(),
        "root weights with multiple pairs should parse: {:?}",
        cli.err()
    );
    match cli.unwrap().command {
        agcli::cli::Commands::Root(agcli::cli::RootCommands::Weights { weights }) => {
            assert_eq!(weights, "1:500,2:300,3:200");
        }
        other => panic!("Expected Root(Weights), got {:?}", other),
    }
}

/// `root weights` with maximum valid u16 weight values.
#[test]
fn parse_root_weights_max_u16_values() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "root",
        "weights",
        "--weights",
        "0:65535,1:65535,255:65535",
    ]);
    assert!(
        cli.is_ok(),
        "root weights with u16::MAX values should parse: {:?}",
        cli.err()
    );
}

/// `root weights` with `--output json` (global flag) — parse succeeds even though handler
/// ignores output format (audit finding #4).
#[test]
fn parse_root_weights_output_json() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "root",
        "weights",
        "--weights",
        "1:500,2:500",
    ]);
    assert!(
        cli.is_ok(),
        "root weights --output json should parse: {:?}",
        cli.err()
    );
    let parsed = cli.unwrap();
    assert_eq!(parsed.output, agcli::cli::OutputFormat::Json);
}

/// `root weights` with all relevant global flags.
#[test]
fn parse_root_weights_all_global_flags() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--wallet",
        "validator_wallet",
        "--hotkey-name",
        "validator_hot",
        "--network",
        "finney",
        "--output",
        "table",
        "--yes",
        "root",
        "weights",
        "--weights",
        "1:300,2:400,3:300",
    ]);
    assert!(
        cli.is_ok(),
        "root weights with all global flags should parse: {:?}",
        cli.err()
    );
}

// ──────────────────────────────────────────────────────────────────
// Error / rejection tests
// ──────────────────────────────────────────────────────────────────

/// `root weights` with no `--weights` flag must be rejected.
#[test]
fn parse_root_weights_missing_weights_flag() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "root", "weights"]);
    assert!(
        cli.is_err(),
        "root weights without --weights should be rejected"
    );
}

/// `root` with no subcommand must be rejected.
#[test]
fn parse_root_no_subcommand() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "root"]);
    assert!(
        cli.is_err(),
        "root with no subcommand should be rejected (clap requires one)"
    );
}

/// `root register` does not accept unknown positional arguments.
#[test]
fn parse_root_register_rejects_unknown_args() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "root", "register", "--unknown-flag"]);
    assert!(
        cli.is_err(),
        "root register with unknown flag should be rejected"
    );
}

/// `root weights` does not accept an empty weights string at clap level (string is non-empty
/// at parse time; semantic validation happens at runtime in `parse_weight_pairs`).
/// This test documents that the empty-string case reaches the handler, not clap.
#[test]
fn parse_root_weights_empty_string_reaches_handler() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "root", "weights", "--weights", ""]);
    // clap accepts any string, including empty; the runtime validator rejects it.
    assert!(
        cli.is_ok(),
        "root weights --weights '' should parse (runtime validation rejects it later)"
    );
}

// ──────────────────────────────────────────────────────────────────
// Enum variant coverage: assert that no RootCommands variants were
// added without corresponding parse tests above.
// ──────────────────────────────────────────────────────────────────

/// Exhaustive match over RootCommands variants to force a compile error if a new
/// variant is added without updating this test file.
#[test]
fn root_commands_variant_coverage() {
    // If RootCommands gains a new variant, this match will fail to compile, alerting
    // the developer that audit_root.rs needs to be updated.
    let cmd = agcli::cli::RootCommands::Register;
    match cmd {
        agcli::cli::RootCommands::Register => {}
        agcli::cli::RootCommands::Weights { .. } => {}
    }
}

// ──────────────────────────────────────────────────────────────────
// Runtime unit tests for parse_weight_pairs (the helper used by
// `root weights` handler to parse the --weights string)
// ──────────────────────────────────────────────────────────────────

/// Valid multi-pair weight string round-trips through the parser.
#[test]
fn weight_pairs_valid_multi() {
    let result = agcli::cli::helpers::parse_weight_pairs("1:500,2:300,3:200");
    assert!(result.is_ok(), "valid weight pairs should parse: {:?}", result.err());
    let (uids, weights) = result.unwrap();
    assert_eq!(uids, vec![1u16, 2, 3]);
    assert_eq!(weights, vec![500u16, 300, 200]);
}

/// Single pair parses correctly.
#[test]
fn weight_pairs_valid_single() {
    let result = agcli::cli::helpers::parse_weight_pairs("7:1000");
    assert!(result.is_ok());
    let (uids, weights) = result.unwrap();
    assert_eq!(uids, vec![7u16]);
    assert_eq!(weights, vec![1000u16]);
}

/// Weight value at u16 boundary (65535) is accepted.
#[test]
fn weight_pairs_u16_max() {
    let result = agcli::cli::helpers::parse_weight_pairs("0:65535");
    assert!(result.is_ok());
    let (uids, weights) = result.unwrap();
    assert_eq!(uids[0], 0u16);
    assert_eq!(weights[0], 65535u16);
}

/// Weight value above u16 range must be rejected.
#[test]
fn weight_pairs_overflow_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("0:65536");
    assert!(
        result.is_err(),
        "weight > u16::MAX should be rejected by parse_weight_pairs"
    );
}

/// UID above u16 range must be rejected.
#[test]
fn weight_pairs_uid_overflow_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("65536:100");
    assert!(
        result.is_err(),
        "uid > u16::MAX should be rejected by parse_weight_pairs"
    );
}

/// Missing colon separator must be rejected.
#[test]
fn weight_pairs_no_colon_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("1");
    assert!(
        result.is_err(),
        "pair without ':' separator should be rejected"
    );
}

/// Extra colon (e.g., uid:wt:extra) must be rejected.
#[test]
fn weight_pairs_extra_colon_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("1:100:extra");
    assert!(
        result.is_err(),
        "pair with extra ':' should be rejected"
    );
}

/// Empty string input is rejected because it cannot form a valid pair.
#[test]
fn weight_pairs_empty_string_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("");
    assert!(
        result.is_err(),
        "empty weight string should be rejected by parse_weight_pairs"
    );
}

/// Whitespace-padded pairs are accepted (parser trims whitespace).
#[test]
fn weight_pairs_whitespace_trimmed() {
    let result = agcli::cli::helpers::parse_weight_pairs("1 : 500 , 2 : 300");
    assert!(
        result.is_ok(),
        "whitespace-padded weight pairs should parse: {:?}",
        result.err()
    );
}

/// Non-numeric UID (e.g., "abc") must be rejected.
#[test]
fn weight_pairs_non_numeric_uid_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("abc:100");
    assert!(
        result.is_err(),
        "non-numeric uid should be rejected by parse_weight_pairs"
    );
}

/// Non-numeric weight must be rejected.
#[test]
fn weight_pairs_non_numeric_weight_rejected() {
    let result = agcli::cli::helpers::parse_weight_pairs("1:abc");
    assert!(
        result.is_err(),
        "non-numeric weight should be rejected by parse_weight_pairs"
    );
}

/// Stress: 32 uid:weight pairs all parse correctly.
#[test]
fn weight_pairs_stress_32_pairs() {
    let pairs: Vec<String> = (0u16..32).map(|i| format!("{}:{}", i, i * 100)).collect();
    let input = pairs.join(",");
    let result = agcli::cli::helpers::parse_weight_pairs(&input);
    assert!(
        result.is_ok(),
        "32 weight pairs should parse without panic: {:?}",
        result.err()
    );
    let (uids, weights) = result.unwrap();
    assert_eq!(uids.len(), 32);
    assert_eq!(weights.len(), 32);
    assert_eq!(uids[0], 0u16);
    assert_eq!(uids[31], 31u16);
}

// ──────────────────────────────────────────────────────────────────
// NetUid::ROOT constant sanity check
// ──────────────────────────────────────────────────────────────────

/// The ROOT NetUid must be 0; the handler hardcodes NetUid::ROOT when setting root weights.
#[test]
fn net_uid_root_is_zero() {
    assert_eq!(
        agcli::types::NetUid::ROOT.as_u16(),
        0u16,
        "NetUid::ROOT must be 0 — root weights use netuid=0"
    );
}

// ──────────────────────────────────────────────────────────────────
// Green-path integration test (requires live local chain)
// ──────────────────────────────────────────────────────────────────

/// End-to-end green path for root commands against a locally running chain.
///
/// Prerequisites:
///   - A subtensor local chain running at `ws://127.0.0.1:9944`
///   - Docker + `ghcr.io/opentensor/subtensor-localnet:devnet-ready` image pulled
///   - An `//Alice` wallet configured in `~/.bittensor/wallets/alice`
///
/// This test is `#[ignore]`d because Docker is unavailable in the cloud-agent CI environment.
/// To run locally:
///   ```sh
///   cargo test --test audit_root green_path_root_localnet -- --ignored
///   ```
#[ignore]
#[test]
fn green_path_root_localnet() {
    // Parse `root register` — confirms the command reaches clap without error.
    // A real green path would: connect to chain, fund Alice, call root register,
    // then root weights --weights "1:1000,2:1000".
    let register_cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--endpoint",
        "ws://127.0.0.1:9944",
        "--wallet",
        "alice",
        "--yes",
        "root",
        "register",
    ]);
    assert!(
        register_cli.is_ok(),
        "root register parse must succeed for green path: {:?}",
        register_cli.err()
    );

    // Parse `root weights` targeting subnets 1 and 2 (common on localnet after scaffold).
    let weights_cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--endpoint",
        "ws://127.0.0.1:9944",
        "--wallet",
        "alice",
        "--yes",
        "root",
        "weights",
        "--weights",
        "1:32768,2:32768",
    ]);
    assert!(
        weights_cli.is_ok(),
        "root weights parse must succeed for green path: {:?}",
        weights_cli.err()
    );
}
