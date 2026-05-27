//! Audit: `agcli batch` (utility.batch_all extrinsic from JSON file)
//!
//! Run: cargo test --test audit_batch

use clap::Parser;

// ── Parse-surface tests ──────────────────────────────────────────────────────

#[test]
fn parse_batch_default_mode() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "batch", "--file", "calls.json"]);
    assert!(
        cli.is_ok(),
        "batch --file calls.json should parse: {:?}",
        cli.err()
    );
    let cli = cli.unwrap();
    match cli.command {
        agcli::cli::Commands::Batch {
            file,
            no_atomic,
            force,
        } => {
            assert_eq!(file, "calls.json");
            assert!(!no_atomic, "no_atomic should default false");
            assert!(!force, "force should default false");
        }
        other => panic!("unexpected command variant: {:?}", other),
    }
}

#[test]
fn parse_batch_no_atomic_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "batch", "--file", "calls.json", "--no-atomic"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    match cli.unwrap().command {
        agcli::cli::Commands::Batch {
            no_atomic, force, ..
        } => {
            assert!(no_atomic, "--no-atomic should be true");
            assert!(!force);
        }
        other => panic!("{:?}", other),
    }
}

#[test]
fn parse_batch_force_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "batch", "--file", "calls.json", "--force"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    match cli.unwrap().command {
        agcli::cli::Commands::Batch {
            no_atomic, force, ..
        } => {
            assert!(!no_atomic);
            assert!(force, "--force should be true");
        }
        other => panic!("{:?}", other),
    }
}

/// Both --no-atomic and --force are accepted by clap (no conflict declared).
/// Handler priority: force > no_atomic > default (batch_all).
#[test]
fn parse_batch_both_flags_accepted() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "batch",
        "--file",
        "calls.json",
        "--no-atomic",
        "--force",
    ]);
    assert!(
        cli.is_ok(),
        "clap accepts both --no-atomic and --force simultaneously: {:?}",
        cli.err()
    );
    match cli.unwrap().command {
        agcli::cli::Commands::Batch {
            no_atomic, force, ..
        } => {
            assert!(no_atomic);
            assert!(force);
        }
        other => panic!("{:?}", other),
    }
}

/// --file is required; omitting it must cause a parse error.
#[test]
fn parse_batch_missing_file_is_error() {
    let result = agcli::cli::Cli::try_parse_from(["agcli", "batch"]);
    assert!(
        result.is_err(),
        "batch without --file should fail to parse (required arg)"
    );
}

/// Global --yes flag must propagate to the Cli struct.
#[test]
fn parse_batch_with_global_yes() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "--yes", "batch", "--file", "calls.json"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    let cli = cli.unwrap();
    assert!(cli.yes, "--yes should be true on the Cli struct");
    assert!(matches!(cli.command, agcli::cli::Commands::Batch { .. }));
}

/// Global --output json must propagate.
#[test]
fn parse_batch_with_output_json() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "--output",
        "json",
        "batch",
        "--file",
        "calls.json",
    ]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    let cli = cli.unwrap();
    assert!(
        cli.output.is_json(),
        "--output json should make is_json() true"
    );
}

/// Global --batch (non-interactive mode) should parse alongside the batch command.
#[test]
fn parse_batch_with_global_batch_flag() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "--batch", "batch", "--file", "calls.json"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    assert!(cli.unwrap().batch, "global --batch flag should be true");
}

/// --file must accept paths with spaces when quoted (arg passing).
#[test]
fn parse_batch_file_path_with_special_chars() {
    let cli = agcli::cli::Cli::try_parse_from(["agcli", "batch", "--file", "/tmp/my calls.json"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    match cli.unwrap().command {
        agcli::cli::Commands::Batch { file, .. } => {
            assert_eq!(file, "/tmp/my calls.json");
        }
        other => panic!("{:?}", other),
    }
}

/// --file with absolute path must be accepted verbatim.
#[test]
fn parse_batch_absolute_file_path() {
    let cli =
        agcli::cli::Cli::try_parse_from(["agcli", "batch", "--file", "/etc/agcli/batch_ops.json"]);
    assert!(cli.is_ok(), "{:?}", cli.err());
    match cli.unwrap().command {
        agcli::cli::Commands::Batch { file, .. } => {
            assert_eq!(file, "/etc/agcli/batch_ops.json");
        }
        other => panic!("{:?}", other),
    }
}

// ── validate_batch_file unit tests ───────────────────────────────────────────

#[test]
fn validate_batch_rejects_empty_array() {
    let result = agcli::cli::helpers::validate_batch_file("[]", "test.json");
    assert!(result.is_err(), "empty array must be rejected");
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("empty"), "error should mention 'empty': {msg}");
}

#[test]
fn validate_batch_rejects_non_array_object() {
    let result =
        agcli::cli::helpers::validate_batch_file(r#"{"pallet": "SubtensorModule"}"#, "test.json");
    assert!(result.is_err(), "non-array JSON must be rejected");
}

#[test]
fn validate_batch_rejects_non_array_string() {
    let result = agcli::cli::helpers::validate_batch_file(r#""hello""#, "test.json");
    assert!(result.is_err(), "string JSON must be rejected");
}

#[test]
fn validate_batch_rejects_invalid_json() {
    let result = agcli::cli::helpers::validate_batch_file("{bad json", "test.json");
    assert!(result.is_err(), "invalid JSON must be rejected");
}

#[test]
fn validate_batch_rejects_missing_pallet_field() {
    let json = r#"[{"call": "add_stake", "args": []}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("pallet"),
        "error should reference 'pallet': {msg}"
    );
}

#[test]
fn validate_batch_rejects_missing_call_field() {
    let json = r#"[{"pallet": "SubtensorModule", "args": []}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("call"), "error should reference 'call': {msg}");
}

#[test]
fn validate_batch_rejects_missing_args_field() {
    let json = r#"[{"pallet": "SubtensorModule", "call": "add_stake"}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("args"), "error should reference 'args': {msg}");
}

#[test]
fn validate_batch_rejects_args_not_array() {
    let json = r#"[{"pallet": "SubtensorModule", "call": "add_stake", "args": "bad"}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(result.is_err(), "non-array args must be rejected");
}

#[test]
fn validate_batch_rejects_too_many_calls() {
    // Construct 1001 minimal valid calls, one more than the 1000-call cap.
    let single = r#"{"pallet":"Balances","call":"transfer_allow_death","args":[]}"#;
    let calls = std::iter::repeat(single)
        .take(1001)
        .collect::<Vec<_>>()
        .join(",");
    let json = format!("[{}]", calls);
    let result = agcli::cli::helpers::validate_batch_file(&json, "test.json");
    assert!(result.is_err(), "1001 calls must exceed the 1000-call cap");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("too many"),
        "error should mention 'too many': {msg}"
    );
}

#[test]
fn validate_batch_accepts_exactly_1000_calls() {
    let single = r#"{"pallet":"Balances","call":"transfer_allow_death","args":[]}"#;
    let calls = std::iter::repeat(single)
        .take(1000)
        .collect::<Vec<_>>()
        .join(",");
    let json = format!("[{}]", calls);
    let result = agcli::cli::helpers::validate_batch_file(&json, "test.json");
    assert!(
        result.is_ok(),
        "1000 calls should be accepted: {:?}",
        result.err()
    );
}

#[test]
fn validate_batch_accepts_valid_single_call() {
    let json = r#"[{"pallet": "SubtensorModule", "call": "add_stake", "args": [1, 2, 3]}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(
        result.is_ok(),
        "valid single call should be accepted: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn validate_batch_accepts_multi_call_mix() {
    let json = r#"[
        {"pallet": "SubtensorModule", "call": "add_stake", "args": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1, 1000000000]},
        {"pallet": "Balances", "call": "transfer_allow_death", "args": ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 5000000000]},
        {"pallet": "SubtensorModule", "call": "set_weights", "args": [1, [0, 1], [100, 200], 0]}
    ]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(result.is_ok(), "{:?}", result.err());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn validate_batch_accepts_empty_args_array() {
    let json = r#"[{"pallet": "System", "call": "remark", "args": []}]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(
        result.is_ok(),
        "empty args array must be accepted: {:?}",
        result.err()
    );
}

#[test]
fn validate_batch_rejects_non_object_call_entry() {
    let json = r#"["not-an-object"]"#;
    let result = agcli::cli::helpers::validate_batch_file(json, "test.json");
    assert!(
        result.is_err(),
        "array entries that are not objects must be rejected"
    );
}

// ── Exit-code classification tests for batch error messages ─────────────────

#[test]
fn exit_code_batch_file_not_found_is_io() {
    // "Failed to read batch file '...'" originates from std::fs::read_to_string
    // returning std::io::ErrorKind::NotFound; classify() downcasts the io::Error.
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
    let err = anyhow::anyhow!(io_err).context("Failed to read batch file 'missing.json'");
    assert_eq!(
        agcli::error::classify(&err),
        agcli::error::exit_code::IO,
        "missing batch file should be exit code IO (14)"
    );
}

#[test]
fn exit_code_invalid_json_in_batch_file_is_validation() {
    // serde_json::Error is downcast to VALIDATION (12) in classify().
    let parse_err: serde_json::Error =
        serde_json::from_str::<serde_json::Value>("{bad").unwrap_err();
    let err = anyhow::anyhow!(parse_err).context("Invalid JSON in batch file 'bad.json'");
    assert_eq!(
        agcli::error::classify(&err),
        agcli::error::exit_code::VALIDATION,
        "JSON parse error should be VALIDATION (12)"
    );
}

#[test]
fn exit_code_batch_too_many_calls_is_generic() {
    // validate_batch_file bails with a plain anyhow message that doesn't match
    // any classify() pattern — it lands at GENERIC (1). This is a known gap.
    let err = anyhow::anyhow!("Batch file 'x.json' has too many calls (1001, max 1000).");
    assert_eq!(
        agcli::error::classify(&err),
        agcli::error::exit_code::GENERIC,
        "plain validation bail! lands at GENERIC (1) — classify() gap"
    );
}

#[test]
fn exit_code_toomanycalls_on_chain_is_chain() {
    // If the chain itself rejects with TooManyCalls (utility pallet), the
    // message should contain "toomanycalls" (lowercased) → CHAIN (13).
    let err = anyhow::anyhow!("Extrinsic failed: utility::TooManyCalls");
    assert_eq!(
        agcli::error::classify(&err),
        agcli::error::exit_code::CHAIN,
        "chain TooManyCalls must be exit code CHAIN (13)"
    );
}

// ── Green-path integration test (ignored — requires localnet Docker) ─────────

/// Green-path: submit a System.remark batch to a local subtensor node.
///
/// Requires:
///   - Docker running the localnet image (not available in cloud-agent VM).
///   - `AGCLI_LOCALNET_WS` env var set (defaults to ws://127.0.0.1:9944).
///   - `agcli` binary available in PATH (built via `cargo build --bin agcli`).
///
/// To run locally:
///   1. `agcli localnet start`
///   2. `AGCLI_LOCALNET_WS=ws://127.0.0.1:9944 cargo test --test audit_batch green_path_batch -- --ignored`
#[test]
#[ignore]
fn green_path_batch() {
    let ws =
        std::env::var("AGCLI_LOCALNET_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());

    // Write a minimal System.remark batch JSON.
    let tmpdir = tempfile::tempdir().expect("tempdir");
    let batch_file = tmpdir.path().join("batch.json");
    std::fs::write(
        &batch_file,
        r#"[{"pallet":"System","call":"remark","args":["0x68656c6c6f"]}]"#,
    )
    .expect("write batch file");

    // agcli handle_batch is not pub; exercise via the binary.
    // The binary is built to `target/debug/agcli` by `cargo build --bin agcli`.
    let bin = std::env::var("AGCLI_BIN").unwrap_or_else(|_| "target/debug/agcli".to_string());

    let output = std::process::Command::new(&bin)
        .args([
            "--yes",
            "--output",
            "json",
            "--network",
            &ws,
            "batch",
            "--file",
            batch_file.to_str().unwrap(),
        ])
        .output()
        .unwrap_or_else(|e| panic!("failed to run agcli binary '{}': {}", bin, e));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "agcli batch should exit 0 on localnet.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // JSON output must include tx_hash.
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout should be valid JSON");
    assert!(
        json.get("tx_hash").is_some(),
        "JSON output must contain tx_hash; got: {stdout}"
    );
}
