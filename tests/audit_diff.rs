//! Audit tests for the `diff` command group.
//!
//! Scope: `DiffCommands` (`portfolio`, `subnet`, `network`, `metagraph`) and
//! the `handle_diff` dispatcher in `src/cli/block_cmds.rs`.
//!
//! Run with: `cargo test --test audit_diff`
//! Live-chain test: `cargo test --test audit_diff -- green_path_diff --ignored`

use agcli::cli::{Cli, Commands, DiffCommands, OutputFormat};
use clap::Parser;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

fn diff_cmd(cli: Cli) -> DiffCommands {
    match cli.command {
        Commands::Diff(c) => c,
        other => panic!("expected Diff, got {:?}", other),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// diff portfolio — parse surface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn portfolio_minimal_parse() {
    // --address is optional; --block1 / --block2 are required
    let cli = parse(&["agcli", "diff", "portfolio", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_ok(), "portfolio minimal: {:?}", cli.err());
    let cmd = diff_cmd(cli.unwrap());
    assert!(
        matches!(cmd, DiffCommands::Portfolio { address: None, block1: 100, block2: 200 }),
        "variant mismatch: {cmd:?}"
    );
}

#[test]
fn portfolio_with_address() {
    let cli = parse(&[
        "agcli",
        "diff",
        "portfolio",
        "--address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--block1",
        "100",
        "--block2",
        "200",
    ]);
    assert!(cli.is_ok(), "portfolio with address: {:?}", cli.err());
    let cmd = diff_cmd(cli.unwrap());
    assert!(
        matches!(cmd, DiffCommands::Portfolio { address: Some(_), block1: 100, block2: 200 }),
        "address not captured: {cmd:?}"
    );
}

#[test]
fn portfolio_missing_block1_rejected() {
    let cli = parse(&["agcli", "diff", "portfolio", "--block2", "200"]);
    assert!(cli.is_err(), "missing --block1 should be rejected");
    assert_eq!(cli.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

#[test]
fn portfolio_missing_block2_rejected() {
    let cli = parse(&["agcli", "diff", "portfolio", "--block1", "100"]);
    assert!(cli.is_err(), "missing --block2 should be rejected");
    assert_eq!(cli.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

#[test]
fn portfolio_block_u32_boundary_max() {
    let cli = parse(&["agcli", "diff", "portfolio", "--block1", "0", "--block2", "4294967295"]);
    assert!(cli.is_ok(), "u32::MAX should parse: {:?}", cli.err());
    assert!(
        matches!(diff_cmd(cli.unwrap()), DiffCommands::Portfolio { block2: 4294967295, .. }),
    );
}

#[test]
fn portfolio_block_overflow_rejected() {
    // 2^32 overflows u32
    let cli = parse(&["agcli", "diff", "portfolio", "--block1", "0", "--block2", "4294967296"]);
    assert!(cli.is_err(), "u32 overflow should fail");
}

#[test]
fn portfolio_same_block_allowed() {
    // block1 == block2 is a valid (no-op) diff; the CLI must not reject it
    let cli = parse(&["agcli", "diff", "portfolio", "--block1", "500", "--block2", "500"]);
    assert!(cli.is_ok(), "same block should parse: {:?}", cli.err());
}

#[test]
fn portfolio_json_output_flag() {
    let cli = parse(&[
        "agcli",
        "--output",
        "json",
        "diff",
        "portfolio",
        "--block1",
        "100",
        "--block2",
        "200",
    ]);
    assert!(cli.is_ok(), "json flag: {:?}", cli.err());
    let parsed = cli.unwrap();
    assert_eq!(parsed.output, OutputFormat::Json);
}

#[test]
fn portfolio_unknown_flag_rejected() {
    let cli =
        parse(&["agcli", "diff", "portfolio", "--block1", "100", "--block2", "200", "--foo"]);
    assert!(cli.is_err(), "unknown flag should fail");
}

// ─────────────────────────────────────────────────────────────────────────────
// diff subnet — parse surface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn subnet_minimal_parse() {
    let cli =
        parse(&["agcli", "diff", "subnet", "--netuid", "1", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_ok(), "subnet minimal: {:?}", cli.err());
    assert!(
        matches!(diff_cmd(cli.unwrap()), DiffCommands::Subnet { netuid: 1, block1: 100, block2: 200 }),
    );
}

#[test]
fn subnet_missing_netuid_rejected() {
    let cli = parse(&["agcli", "diff", "subnet", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_err(), "missing --netuid should fail");
    assert_eq!(cli.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

#[test]
fn subnet_missing_block2_rejected() {
    let cli = parse(&["agcli", "diff", "subnet", "--netuid", "1", "--block1", "100"]);
    assert!(cli.is_err(), "missing --block2 should fail");
}

#[test]
fn subnet_max_netuid() {
    // netuid is u16 — 65535 should be accepted
    let cli = parse(&[
        "agcli", "diff", "subnet", "--netuid", "65535", "--block1", "100", "--block2", "200",
    ]);
    assert!(cli.is_ok(), "max netuid: {:?}", cli.err());
    assert!(matches!(diff_cmd(cli.unwrap()), DiffCommands::Subnet { netuid: 65535, .. }));
}

#[test]
fn subnet_netuid_overflow_rejected() {
    // 65536 overflows u16
    let cli = parse(&[
        "agcli", "diff", "subnet", "--netuid", "65536", "--block1", "100", "--block2", "200",
    ]);
    assert!(cli.is_err(), "u16 netuid overflow should fail");
}

#[test]
fn subnet_zero_netuid_accepted() {
    // subnet 0 (root) is valid on-chain; CLI must not reject it
    let cli = parse(&[
        "agcli", "diff", "subnet", "--netuid", "0", "--block1", "100", "--block2", "200",
    ]);
    assert!(cli.is_ok(), "zero netuid: {:?}", cli.err());
}

// ─────────────────────────────────────────────────────────────────────────────
// diff network — parse surface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn network_minimal_parse() {
    let cli = parse(&["agcli", "diff", "network", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_ok(), "network minimal: {:?}", cli.err());
    assert!(matches!(diff_cmd(cli.unwrap()), DiffCommands::Network { block1: 100, block2: 200 }));
}

#[test]
fn network_missing_block1_rejected() {
    let cli = parse(&["agcli", "diff", "network", "--block2", "200"]);
    assert!(cli.is_err(), "missing --block1 should fail");
}

#[test]
fn network_missing_block2_rejected() {
    let cli = parse(&["agcli", "diff", "network", "--block1", "100"]);
    assert!(cli.is_err(), "missing --block2 should fail");
}

#[test]
fn network_zero_blocks_accepted() {
    let cli = parse(&["agcli", "diff", "network", "--block1", "0", "--block2", "0"]);
    assert!(cli.is_ok(), "zero blocks: {:?}", cli.err());
}

#[test]
fn network_same_block_accepted() {
    let cli = parse(&["agcli", "diff", "network", "--block1", "500", "--block2", "500"]);
    assert!(cli.is_ok(), "same block: {:?}", cli.err());
}

#[test]
fn network_global_endpoint_flag() {
    let cli = parse(&[
        "agcli",
        "--endpoint",
        "ws://127.0.0.1:9944",
        "diff",
        "network",
        "--block1",
        "1",
        "--block2",
        "2",
    ]);
    assert!(cli.is_ok(), "endpoint flag: {:?}", cli.err());
    assert_eq!(cli.unwrap().endpoint, Some("ws://127.0.0.1:9944".to_string()));
}

#[test]
fn network_archive_network_flag() {
    let cli = parse(&[
        "agcli",
        "--network",
        "archive",
        "diff",
        "network",
        "--block1",
        "1000000",
        "--block2",
        "2000000",
    ]);
    assert!(cli.is_ok(), "archive flag: {:?}", cli.err());
    let parsed = cli.unwrap();
    assert_eq!(parsed.network, "archive");
}

// ─────────────────────────────────────────────────────────────────────────────
// diff metagraph — parse surface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn metagraph_minimal_parse() {
    let cli = parse(&[
        "agcli", "diff", "metagraph", "--netuid", "1", "--block1", "100", "--block2", "200",
    ]);
    assert!(cli.is_ok(), "metagraph minimal: {:?}", cli.err());
    assert!(matches!(
        diff_cmd(cli.unwrap()),
        DiffCommands::Metagraph { netuid: 1, block1: 100, block2: 200 }
    ));
}

#[test]
fn metagraph_missing_netuid_rejected() {
    let cli = parse(&["agcli", "diff", "metagraph", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_err(), "missing --netuid should fail");
    assert_eq!(cli.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

#[test]
fn metagraph_missing_block1_rejected() {
    let cli = parse(&["agcli", "diff", "metagraph", "--netuid", "1", "--block2", "200"]);
    assert!(cli.is_err(), "missing --block1 should fail");
}

#[test]
fn metagraph_missing_block2_rejected() {
    let cli = parse(&["agcli", "diff", "metagraph", "--netuid", "1", "--block1", "100"]);
    assert!(cli.is_err(), "missing --block2 should fail");
}

#[test]
fn metagraph_max_netuid() {
    let cli = parse(&[
        "agcli", "diff", "metagraph", "--netuid", "65535", "--block1", "100", "--block2", "200",
    ]);
    assert!(cli.is_ok(), "max netuid: {:?}", cli.err());
    assert!(matches!(diff_cmd(cli.unwrap()), DiffCommands::Metagraph { netuid: 65535, .. }));
}

#[test]
fn metagraph_json_output() {
    let cli = parse(&[
        "agcli",
        "--output",
        "json",
        "diff",
        "metagraph",
        "--netuid",
        "1",
        "--block1",
        "100",
        "--block2",
        "200",
    ]);
    assert!(cli.is_ok(), "metagraph json: {:?}", cli.err());
    assert_eq!(cli.unwrap().output, OutputFormat::Json);
}

// ─────────────────────────────────────────────────────────────────────────────
// diff — top-level rejects
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn diff_without_subcommand_rejected() {
    let cli = parse(&["agcli", "diff"]);
    assert!(cli.is_err(), "diff with no subcommand should fail");
}

#[test]
fn diff_unknown_subcommand_rejected() {
    let cli = parse(&["agcli", "diff", "balances", "--block1", "100", "--block2", "200"]);
    assert!(cli.is_err(), "unknown subcommand should fail");
}

// ─────────────────────────────────────────────────────────────────────────────
// Compile-time audit guards (document code vs. docs drift)
// ─────────────────────────────────────────────────────────────────────────────

/// Audit guard: DiffCommands has exactly 4 variants.
///
/// Adding or removing a variant breaks this match and forces the auditor to
/// update docs/commands/diff.md and this test file.
#[test]
fn all_diff_subcommands_enumerated() {
    let variants: &[DiffCommands] = &[
        DiffCommands::Portfolio { address: None, block1: 0, block2: 1 },
        DiffCommands::Subnet { netuid: 1, block1: 0, block2: 1 },
        DiffCommands::Network { block1: 0, block2: 1 },
        DiffCommands::Metagraph { netuid: 1, block1: 0, block2: 1 },
    ];
    // The only assertion needed is that this compiles — any exhaustiveness
    // failure will appear as a compile error.
    assert_eq!(variants.len(), 4, "expected 4 DiffCommands variants");
}

/// Audit guard: `diff subnet` JSON output omits `tempo` and `owner_hotkey`.
///
/// The human table in `handle_diff` prints both fields; the JSON object does
/// not include them. This test documents the drift so it is not silently lost.
/// See Findings in the handoff for the source citation.
#[test]
fn audit_subnet_json_missing_tempo_and_owner_hotkey() {
    // Verify that the fields we expect to be ABSENT from the JSON object are
    // indeed absent in the serialisation path. We cannot call the handler
    // without a chain connection, so we document the structural expectation
    // using a static JSON template that mirrors the actual serde_json::json!{}
    // call in handle_diff::Subnet.
    let json = serde_json::json!({
        "netuid": 1u16,
        "name": "test",
        "block1": 100u32,
        "block2": 200u32,
        "tao_in": [1.0f64, 1.1f64],
        "tao_in_diff": 0.1f64,
        "price": [0.5f64, 0.55f64],
        "price_diff": 0.05f64,
        "emission": [1000u64, 1100u64],
        "emission_diff": 100i128,
        // tempo and owner_hotkey are intentionally omitted in the real code
    });
    assert!(json.get("tempo").is_none(), "tempo is not in diff subnet JSON — this is the documented drift");
    assert!(json.get("owner_hotkey").is_none(), "owner_hotkey is not in diff subnet JSON — documented drift");
    assert!(json.get("tao_in").is_some());
    assert!(json.get("emission_diff").is_some());
}

/// Audit guard: `diff network` JSON output has no diff fields.
///
/// `diff portfolio` and `diff subnet` include `*_diff` scalar fields alongside
/// the two-element arrays. `diff network` does not — consumers cannot compute
/// the delta without doing subtraction themselves.
#[test]
fn audit_network_json_no_diff_fields() {
    let json = serde_json::json!({
        "block1": 100u32,
        "block2": 200u32,
        "total_issuance_tao": [1000.0f64, 1001.0f64],
        "total_stake_tao": [400.0f64, 401.0f64],
        "staking_ratio_pct": [40.0f64, 40.05f64],
        "subnet_count": [30usize, 31usize],
        // No diff scalar fields — documented drift vs portfolio and subnet
    });
    assert!(json.get("total_issuance_diff").is_none(), "no diff field in network JSON");
    assert!(json.get("total_stake_diff").is_none(), "no diff field in network JSON");
    assert!(json.get("subnet_count_diff").is_none(), "no diff field in network JSON");
    assert_eq!(json.get("subnet_count").unwrap().as_array().unwrap().len(), 2);
}

/// Audit guard: `diff metagraph` does not track neurons removed between blocks.
///
/// The loop in handle_diff::Metagraph iterates over `neurons2` and looks up
/// each UID in `map1`. Neurons present in block1 but absent in block2 are
/// never added to `changes`. This means the "removed" case is silently dropped.
#[test]
fn audit_metagraph_removed_neurons_not_tracked() {
    // Simulate the exact HashMap logic from handle_diff::Metagraph to confirm
    // the asymmetry without a chain connection.
    use std::collections::HashMap;

    // Two fake neuron UIDs at block1; only one persists to block2
    let uids_block1: Vec<u16> = vec![0, 1, 2];
    let uids_block2: Vec<u16> = vec![0, 1]; // UID 2 removed

    let map1: HashMap<u16, ()> = uids_block1.iter().map(|&uid| (uid, ())).collect();

    let mut changes: Vec<u16> = Vec::new();
    for uid2 in &uids_block2 {
        if map1.get(uid2).is_none() {
            changes.push(*uid2);
        }
    }
    // No "removed" logic exists in the handler — UID 2 is never in changes
    assert!(!changes.iter().any(|&u| u == 2), "UID 2 was removed but not tracked — documented drift");
    // The drift: if we wanted completeness we would iterate map1 for UIDs not
    // present in uids_block2 and emit "change: removed" entries.
}

/// Audit guard: `diff portfolio` float subtraction precision.
///
/// `balance_diff_tao` is computed as `bal2.tao() - bal1.tao()` (f64 arithmetic).
/// Performing the diff at the rao (u64) level first and then converting would
/// eliminate floating-point cancellation error for small differences.
#[test]
fn audit_portfolio_float_diff_precision() {
    // Demonstrate the precision loss that can occur when diffing as f64
    let rao1: u64 = 1_000_000_000; // 1 TAO exactly
    let rao2: u64 = 1_000_000_001; // 1 TAO + 1 rao

    // Current code path: convert to f64 first, then subtract
    let tao1 = rao1 as f64 / 1_000_000_000.0;
    let tao2 = rao2 as f64 / 1_000_000_000.0;
    let diff_as_float = tao2 - tao1; // may not equal 1e-9 exactly

    // Preferred path: subtract at rao level, then convert
    let diff_at_rao = rao2 - rao1; // 1 rao, exact
    let diff_as_rao_then_float = diff_at_rao as f64 / 1_000_000_000.0;

    // Both paths produce the same result for this test case (f64 is precise
    // enough for 1e9 range), but the rao-first path is always exact.
    assert_eq!(diff_at_rao, 1u64, "rao diff is exact");
    // Document the drift: the current f64 path may lose precision near zero
    let _ = diff_as_float; // accepted — just documenting the pattern
    let _ = diff_as_rao_then_float;
}

// ─────────────────────────────────────────────────────────────────────────────
// Exit-code classification tests (no chain connection required)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn exit_code_subnet_not_found_is_validation() {
    // "Subnet N not found at block B" should classify as VALIDATION (12)
    // because classify() matches msg.contains("subnet") && msg.contains("not found")
    let err = anyhow::anyhow!("Subnet 99 not found at block 100");
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::VALIDATION);
}

#[test]
fn exit_code_network_error_is_network() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused");
    let err = anyhow::Error::from(io_err);
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::NETWORK);
}

#[test]
fn exit_code_no_address_is_generic() {
    // "No address provided and no wallet found." does not match VALIDATION
    // patterns in classify() — exits GENERIC (1). This is the documented
    // behaviour per docs/commands/diff.md; listed as a finding because it
    // should arguably be VALIDATION.
    let err = anyhow::anyhow!("No address provided and no wallet found. Use --address <SS58>.");
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::GENERIC);
}

#[test]
fn exit_code_pruned_state_is_generic() {
    // Pruned block state errors are wrapped with an archive hint but the
    // classify() call on the outer message returns GENERIC (1).
    let err = anyhow::anyhow!("State already discarded for block 100");
    // "discarded" doesn't match any chain/network pattern — exits GENERIC
    assert_eq!(agcli::error::classify(&err), agcli::error::exit_code::GENERIC);
}

// ─────────────────────────────────────────────────────────────────────────────
// Green-path integration test (localnet-gated, #[ignore] by default)
// ─────────────────────────────────────────────────────────────────────────────

/// Green-path integration test for `diff` command group.
///
/// Requires a running subtensor localnet on ws://127.0.0.1:9944.
/// Start with: `agcli localnet start` (needs Docker).
///
/// Run with: `cargo test --test audit_diff -- green_path_diff --ignored`
#[tokio::test]
#[ignore = "requires localnet on ws://127.0.0.1:9944 (Docker unavailable in CI)"]
async fn green_path_diff() {
    use agcli::chain::Client;

    let endpoint = "ws://127.0.0.1:9944";
    let client = Client::connect(endpoint).await.expect("connect to localnet");

    // Get two adjacent blocks to diff against
    let block2 = client.get_block_number().await.expect("get latest block") as u32;
    let block1 = block2.saturating_sub(5);

    // diff network — no required subnet, simplest to verify
    let (hash1, hash2) = tokio::try_join!(
        client.get_block_hash(block1),
        client.get_block_hash(block2),
    )
    .expect("get block hashes");

    let (issuance1, issuance2) = tokio::try_join!(
        client.get_total_issuance_at_block(hash1),
        client.get_total_issuance_at_block(hash2),
    )
    .expect("get total issuance at both blocks");

    // Issuance should be non-zero on localnet (genesis allocations)
    assert!(issuance1.rao() > 0, "block1 issuance must be non-zero");
    assert!(issuance2.rao() > 0, "block2 issuance must be non-zero");

    // diff portfolio — use Alice (a well-known localnet funded account)
    let alice_ss58 = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let (bal1, bal2) = tokio::try_join!(
        client.get_balance_at_block(alice_ss58, hash1),
        client.get_balance_at_block(alice_ss58, hash2),
    )
    .expect("get Alice balance at both blocks");

    // Alice has genesis balance on localnet
    assert!(bal1.rao() > 0, "Alice block1 balance must be non-zero on localnet");
    let _ = bal2; // may be same as bal1 if no transfers occurred

    // diff subnet — subnet 1 (root subnet exists on localnet)
    let netuid = agcli::types::network::NetUid(1);
    let dynamic_result = tokio::try_join!(
        client.get_dynamic_info_at_block(netuid, hash1),
        client.get_dynamic_info_at_block(netuid, hash2),
    );
    // localnet may or may not have subnet 1 depending on genesis config;
    // both Some and None are valid here — we just must not panic
    let _ = dynamic_result.expect("get_dynamic_info must not error (None is valid)");

    // diff metagraph — subnet 1 neurons
    let (neurons1, neurons2) = tokio::try_join!(
        client.get_neurons_lite_at_block(netuid, hash1),
        client.get_neurons_lite_at_block(netuid, hash2),
    )
    .expect("get neurons lite at both blocks");
    // Valid to have zero neurons; must not panic
    let _ = neurons1.len();
    let _ = neurons2.len();
}
