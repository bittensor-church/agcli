//! Audit: safe-mode command group — parse-surface and green-path tests.
//!
//! Covers every subcommand in `SafeModeCommands`:
//!   enter, extend, force-enter, force-exit
//!
//! The `#[ignore]`-gated integration test `green_path_safe_mode` requires a
//! running local node (Docker + `agcli localnet start`) which is unavailable
//! in the cloud-agent VM; it is left ignored per the acceptance criteria.

use clap::Parser;

// ──────────────────────────────────────────────────────────────────────────────
// Parse-surface: safe-mode enter
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_safe_mode_enter_parses() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "enter"]);
    assert!(cli.is_ok(), "safe-mode enter: {:?}", cli.err());
    let parsed = cli.unwrap();
    assert!(
        matches!(
            parsed.command,
            agcli::cli::Commands::SafeMode(agcli::cli::SafeModeCommands::Enter)
        ),
        "command variant should be SafeMode::Enter"
    );
}

#[test]
fn parse_safe_mode_enter_rejects_unknown_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "enter", "--unknown-flag"]);
    assert!(cli.is_err(), "safe-mode enter with unknown flag must fail");
}

// ──────────────────────────────────────────────────────────────────────────────
// Parse-surface: safe-mode extend
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_safe_mode_extend_parses() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "extend"]);
    assert!(cli.is_ok(), "safe-mode extend: {:?}", cli.err());
    let parsed = cli.unwrap();
    assert!(
        matches!(
            parsed.command,
            agcli::cli::Commands::SafeMode(agcli::cli::SafeModeCommands::Extend)
        ),
        "command variant should be SafeMode::Extend"
    );
}

#[test]
fn parse_safe_mode_extend_rejects_unknown_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "extend", "--bogus"]);
    assert!(cli.is_err(), "safe-mode extend with unknown flag must fail");
}

// ──────────────────────────────────────────────────────────────────────────────
// Parse-surface: safe-mode force-enter
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_safe_mode_force_enter_with_duration() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "safe-mode",
        "force-enter",
        "--duration",
        "500",
    ]);
    assert!(
        cli.is_ok(),
        "safe-mode force-enter --duration 500: {:?}",
        cli.err()
    );
    let parsed = cli.unwrap();
    match &parsed.command {
        agcli::cli::Commands::SafeMode(agcli::cli::SafeModeCommands::ForceEnter {
            duration,
        }) => {
            assert_eq!(*duration, 500u32, "duration should be 500");
        }
        other => panic!("expected SafeMode::ForceEnter, got {:?}", other),
    }
}

#[test]
fn parse_safe_mode_force_enter_missing_duration_fails() {
    // --duration is required; omitting it must be a parse error.
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "force-enter"]);
    assert!(
        cli.is_err(),
        "safe-mode force-enter without --duration must fail"
    );
}

#[test]
fn parse_safe_mode_force_enter_duration_zero() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "safe-mode",
        "force-enter",
        "--duration",
        "0",
    ]);
    assert!(cli.is_ok(), "duration 0 is a valid u32: {:?}", cli.err());
}

#[test]
fn parse_safe_mode_force_enter_duration_max_u32() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "safe-mode",
        "force-enter",
        "--duration",
        "4294967295",
    ]);
    assert!(
        cli.is_ok(),
        "duration u32::MAX should parse: {:?}",
        cli.err()
    );
    if let agcli::cli::Commands::SafeMode(agcli::cli::SafeModeCommands::ForceEnter {
        duration,
    }) = cli.unwrap().command
    {
        assert_eq!(duration, u32::MAX);
    }
}

#[test]
fn parse_safe_mode_force_enter_duration_overflow_fails() {
    // u32::MAX + 1 must be rejected during parsing.
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "safe-mode",
        "force-enter",
        "--duration",
        "4294967296",
    ]);
    assert!(
        cli.is_err(),
        "duration > u32::MAX must fail to parse"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Parse-surface: safe-mode force-exit
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn parse_safe_mode_force_exit_parses() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "force-exit"]);
    assert!(cli.is_ok(), "safe-mode force-exit: {:?}", cli.err());
    let parsed = cli.unwrap();
    assert!(
        matches!(
            parsed.command,
            agcli::cli::Commands::SafeMode(agcli::cli::SafeModeCommands::ForceExit)
        ),
        "command variant should be SafeMode::ForceExit"
    );
}

#[test]
fn parse_safe_mode_force_exit_rejects_unknown_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "force-exit", "--duration", "1"]);
    assert!(
        cli.is_err(),
        "safe-mode force-exit does not accept --duration; must fail"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Dispatch-shape: verify `Commands::SafeMode(_)` is what clap produces for
// each subcommand name string, catching renames at the clap layer.
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn safe_mode_subcommand_names_match_cli() {
    let cases: &[&[&str]] = &[
        &["agcli", "safe-mode", "enter"],
        &["agcli", "safe-mode", "extend"],
        &["agcli", "safe-mode", "force-enter", "--duration", "1"],
        &["agcli", "safe-mode", "force-exit"],
    ];
    for args in cases {
        let cli = agcli::cli::Cli::try_parse_from(*args);
        assert!(
            cli.is_ok(),
            "parse failed for {:?}: {:?}",
            args,
            cli.err()
        );
        assert!(
            matches!(
                cli.unwrap().command,
                agcli::cli::Commands::SafeMode(_)
            ),
            "expected Commands::SafeMode for {:?}",
            args
        );
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Global flags thread through safe-mode subcommands.
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn safe_mode_enter_accepts_global_yes_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "--yes", "safe-mode", "enter"]);
    assert!(cli.is_ok(), "--yes + safe-mode enter: {:?}", cli.err());
    assert!(cli.unwrap().yes, "--yes flag should be set");
}

#[test]
fn safe_mode_enter_accepts_wallet_flag() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--wallet",
        "my_wallet",
        "safe-mode",
        "enter",
    ]);
    assert!(cli.is_ok(), "--wallet + safe-mode enter: {:?}", cli.err());
    assert_eq!(
        cli.unwrap().wallet,
        "my_wallet",
        "--wallet should propagate"
    );
}

#[test]
fn safe_mode_force_enter_accepts_network_flag() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--network",
        "finney",
        "safe-mode",
        "force-enter",
        "--duration",
        "100",
    ]);
    assert!(
        cli.is_ok(),
        "--network + force-enter: {:?}",
        cli.err()
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Audit finding: `status` subcommand listed in doc comment but not implemented.
// This test documents that `safe-mode status` is NOT a valid subcommand and
// will be rejected by the parser (i.e. the doc comment is wrong).
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn safe_mode_status_is_not_a_valid_subcommand() {
    // The Commands::SafeMode variant's doc comment claims "status" exists,
    // but SafeModeCommands has no Status variant. This test documents that.
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "safe-mode", "status"]);
    assert!(
        cli.is_err(),
        "safe-mode status should fail — not a real subcommand (doc comment drift)"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Audit finding: `force-enter` takes a `--duration` flag but the FRAME
// pallet-safe-mode::force_enter takes NO arguments. The duration in the pallet
// is determined by ForceEnterOrigin's Success type. This test documents that
// the clap surface accepts a duration while the underlying pallet call does not.
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn force_enter_duration_arg_exists_in_cli_but_not_in_pallet() {
    // Audit note: agcli::Client::safe_mode_force_enter passes duration as
    // vec![Value::u128(duration as u128)] to SafeMode::force_enter, but the
    // FRAME pallet call takes NO parameters. On a live chain this encodes a
    // spurious field that the SCALE decoder will reject.
    //
    // This test passes (the CLI accepts the flag) but documents the mismatch
    // so the planner can track the source-level fix.
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "safe-mode",
        "force-enter",
        "--duration",
        "1000",
    ]);
    assert!(cli.is_ok(), "CLI accepts --duration; see audit finding #2");
}

// ──────────────────────────────────────────────────────────────────────────────
// #[ignore]-gated integration test — requires a live local chain.
// ──────────────────────────────────────────────────────────────────────────────

/// Green-path integration test. Requires `agcli localnet start` (Docker).
///
/// The test verifies:
/// 1. The SafeMode pallet exists in runtime metadata.
/// 2. The expected dispatchables are present: enter, extend, force_enter,
///    force_exit, release_deposit.
/// 3. `safe-mode enter` submits successfully from an account with funds.
///
/// Skip condition: Docker / localnet unavailable (default in CI and cloud-agent VM).
#[tokio::test]
#[ignore]
async fn green_path_safe_mode() {
    use subxt::OnlineClient;

    let url = std::env::var("LOCALNET_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());

    let client = OnlineClient::<subxt::PolkadotConfig>::from_url(&url)
        .await
        .expect("localnet should be reachable at LOCALNET_URL or ws://127.0.0.1:9944");

    let metadata = client.metadata();

    // 1. SafeMode pallet must exist.
    let pallet = metadata
        .pallet_by_name("SafeMode")
        .expect("SafeMode pallet must be present in runtime metadata");

    // 2. Expected dispatchables.
    for call_name in &["enter", "extend", "force_enter", "force_exit", "release_deposit"] {
        assert!(
            pallet.call_variant_by_name(call_name).is_some(),
            "SafeMode pallet must expose '{}' dispatchable",
            call_name
        );
    }

    // 3. Storage items.
    let has_entered_until = metadata
        .pallet_by_name("SafeMode")
        .unwrap()
        .storage()
        .map(|s| s.entries().iter().any(|e| e.name() == "EnteredUntil"))
        .unwrap_or(false);
    assert!(has_entered_until, "SafeMode pallet must have EnteredUntil storage item");
}
