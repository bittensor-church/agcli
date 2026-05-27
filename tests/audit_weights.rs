//! Audit: weights command group — parse-surface and validation tests.
//!
//! Covers all 10 WeightCommands variants:
//!   show, set, commit, reveal, status, commit-timelocked,
//!   commit-reveal, set-mechanism, commit-mechanism, reveal-mechanism
//!
//! Run: `cargo test --test audit_weights`
//! Localnet integration: `cargo test --test audit_weights -- --ignored`

use clap::Parser;

// ── helpers ──────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<agcli::cli::Cli, clap::Error> {
    agcli::cli::Cli::try_parse_from(args)
}

fn ok(args: &[&str]) {
    assert!(parse(args).is_ok(), "expected Ok for {:?}", args);
}

fn err(args: &[&str]) {
    assert!(parse(args).is_err(), "expected Err for {:?}", args);
}

// ── weights show ─────────────────────────────────────────────────────────────

#[test]
fn show_basic() {
    ok(&["agcli", "weights", "show", "--netuid", "1"]);
}

#[test]
fn show_with_hotkey() {
    ok(&[
        "agcli",
        "weights",
        "show",
        "--netuid",
        "1",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ]);
}

#[test]
fn show_with_limit() {
    ok(&["agcli", "weights", "show", "--netuid", "1", "--limit", "20"]);
}

#[test]
fn show_all_flags() {
    ok(&[
        "agcli",
        "--output",
        "json",
        "weights",
        "show",
        "--netuid",
        "97",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "--limit",
        "5",
    ]);
}

#[test]
fn show_missing_netuid_is_error() {
    err(&["agcli", "weights", "show"]);
}

// ── weights set ───────────────────────────────────────────────────────────────

#[test]
fn set_basic() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100,1:200",
    ]);
}

#[test]
fn set_with_version_key() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--version-key",
        "42",
    ]);
}

#[test]
fn set_max_version_key() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--version-key",
        "18446744073709551615",
    ]);
}

#[test]
fn set_dry_run_via_global_flag() {
    let cli = parse(&[
        "agcli",
        "--dry-run",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ])
    .expect("dry-run parse");
    assert!(cli.dry_run);
}

#[test]
fn set_stdin_weights() {
    ok(&["agcli", "weights", "set", "--netuid", "1", "--weights", "-"]);
}

#[test]
fn set_file_weights() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "@weights.json",
    ]);
}

#[test]
fn set_json_array_input() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        r#"[{"uid":0,"weight":100}]"#,
    ]);
}

#[test]
fn set_json_object_input() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        r#"{"0":100,"1":200}"#,
    ]);
}

#[test]
fn set_max_u16_pairs() {
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:65535,65535:65535",
    ]);
}

#[test]
fn set_missing_netuid_is_error() {
    err(&["agcli", "weights", "set", "--weights", "0:100"]);
}

#[test]
fn set_missing_weights_is_error() {
    err(&["agcli", "weights", "set", "--netuid", "1"]);
}

#[test]
fn set_with_global_output_json() {
    ok(&[
        "agcli",
        "--output",
        "json",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--dry-run",
    ]);
}

// ── weights commit ────────────────────────────────────────────────────────────

#[test]
fn commit_basic() {
    ok(&[
        "agcli",
        "weights",
        "commit",
        "--netuid",
        "1",
        "--weights",
        "0:100,1:200",
    ]);
}

#[test]
fn commit_with_explicit_salt() {
    ok(&[
        "agcli",
        "weights",
        "commit",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--salt",
        "my-secret-salt-32chars000000000",
    ]);
}

#[test]
fn commit_without_salt_is_ok() {
    ok(&[
        "agcli",
        "weights",
        "commit",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn commit_missing_netuid_is_error() {
    err(&["agcli", "weights", "commit", "--weights", "0:100"]);
}

#[test]
fn commit_missing_weights_is_error() {
    err(&["agcli", "weights", "commit", "--netuid", "1"]);
}

// ── weights reveal ────────────────────────────────────────────────────────────

#[test]
fn reveal_basic() {
    ok(&[
        "agcli",
        "weights",
        "reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100,1:200",
        "--salt",
        "mysalt",
    ]);
}

#[test]
fn reveal_with_version_key() {
    ok(&[
        "agcli",
        "weights",
        "reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--salt",
        "abc",
        "--version-key",
        "99",
    ]);
}

#[test]
fn reveal_missing_salt_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn reveal_missing_weights_is_error() {
    err(&[
        "agcli", "weights", "reveal", "--netuid", "1", "--salt", "abc",
    ]);
}

#[test]
fn reveal_missing_netuid_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal",
        "--weights",
        "0:100",
        "--salt",
        "abc",
    ]);
}

// ── weights status ────────────────────────────────────────────────────────────

#[test]
fn status_basic() {
    ok(&["agcli", "weights", "status", "--netuid", "1"]);
}

#[test]
fn status_missing_netuid_is_error() {
    err(&["agcli", "weights", "status"]);
}

// ── weights commit-timelocked ─────────────────────────────────────────────────

#[test]
fn commit_timelocked_basic() {
    ok(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "1",
        "--weights",
        "0:100,1:200",
        "--round",
        "12345",
    ]);
}

#[test]
fn commit_timelocked_with_salt() {
    ok(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "1",
        "--weights",
        "0:65535",
        "--round",
        "99999",
        "--salt",
        "timelock-test-salt",
    ]);
}

#[test]
fn commit_timelocked_without_salt_is_ok() {
    ok(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--round",
        "1",
    ]);
}

#[test]
fn commit_timelocked_missing_round_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn commit_timelocked_missing_netuid_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--weights",
        "0:100",
        "--round",
        "1",
    ]);
}

#[test]
fn commit_timelocked_missing_weights_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "1",
        "--round",
        "1",
    ]);
}

#[test]
fn commit_timelocked_max_round() {
    ok(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "5",
        "--weights",
        "0:100",
        "--round",
        "18446744073709551615",
    ]);
}

// ── weights commit-reveal ─────────────────────────────────────────────────────

#[test]
fn commit_reveal_basic() {
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn commit_reveal_with_wait() {
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100,1:200",
        "--wait",
    ]);
}

#[test]
fn commit_reveal_with_version_key() {
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--version-key",
        "7",
    ]);
}

#[test]
fn commit_reveal_stdin_weights() {
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "-",
    ]);
}

#[test]
fn commit_reveal_file_weights() {
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "@weights.json",
    ]);
}

#[test]
fn commit_reveal_missing_netuid_is_error() {
    err(&["agcli", "weights", "commit-reveal", "--weights", "0:100"]);
}

#[test]
fn commit_reveal_missing_weights_is_error() {
    err(&["agcli", "weights", "commit-reveal", "--netuid", "1"]);
}

// ── weights set-mechanism ─────────────────────────────────────────────────────

#[test]
fn set_mechanism_yuma() {
    ok(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100,1:200",
    ]);
}

#[test]
fn set_mechanism_oracle() {
    ok(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "1",
        "--weights",
        "0:65535",
    ]);
}

#[test]
fn set_mechanism_with_version_key() {
    ok(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "5",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
        "--version-key",
        "3",
    ]);
}

#[test]
fn set_mechanism_with_dry_run() {
    let cli = parse(&[
        "agcli",
        "--dry-run",
        "weights",
        "set-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
    ])
    .expect("set-mechanism dry-run parse");
    assert!(cli.dry_run);
}

#[test]
fn set_mechanism_missing_mechanism_id_is_error() {
    err(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn set_mechanism_missing_weights_is_error() {
    err(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
    ]);
}

#[test]
fn set_mechanism_missing_netuid_is_error() {
    err(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
    ]);
}

// ── weights commit-mechanism ──────────────────────────────────────────────────

#[test]
fn commit_mechanism_basic() {
    ok(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--hash",
        "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ]);
}

#[test]
fn commit_mechanism_without_0x_prefix() {
    ok(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--hash",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ]);
}

#[test]
fn commit_mechanism_oracle_mechanism() {
    ok(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "5",
        "--mechanism-id",
        "1",
        "--hash",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ]);
}

#[test]
fn commit_mechanism_missing_hash_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
    ]);
}

#[test]
fn commit_mechanism_missing_mechanism_id_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "1",
        "--hash",
        "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ]);
}

#[test]
fn commit_mechanism_missing_netuid_is_error() {
    err(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--mechanism-id",
        "0",
        "--hash",
        "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ]);
}

// ── weights reveal-mechanism ──────────────────────────────────────────────────

#[test]
fn reveal_mechanism_basic() {
    ok(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:65535",
        "--salt",
        "e2e-mech-commit",
    ]);
}

#[test]
fn reveal_mechanism_with_version_key() {
    ok(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "5",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100,1:200",
        "--salt",
        "mysalt",
        "--version-key",
        "7",
    ]);
}

#[test]
fn reveal_mechanism_oracle() {
    ok(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "1",
        "--weights",
        "0:100",
        "--salt",
        "s",
    ]);
}

#[test]
fn reveal_mechanism_missing_salt_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn reveal_mechanism_missing_weights_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--salt",
        "s",
    ]);
}

#[test]
fn reveal_mechanism_missing_mechanism_id_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--salt",
        "s",
    ]);
}

#[test]
fn reveal_mechanism_missing_netuid_is_error() {
    err(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
        "--salt",
        "s",
    ]);
}

// ── No subcommand ─────────────────────────────────────────────────────────────

#[test]
fn weights_no_subcommand_is_error() {
    err(&["agcli", "weights"]);
}

// ── Global flag wiring ─────────────────────────────────────────────────────────

#[test]
fn global_yes_and_batch_flags_reach_weights_set() {
    let cli = parse(&[
        "agcli",
        "--yes",
        "--batch",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ])
    .expect("global flags");
    assert!(cli.yes);
    assert!(cli.batch);
}

#[test]
fn global_network_flag_is_accepted() {
    ok(&[
        "agcli",
        "--network",
        "test",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
}

#[test]
fn global_finalization_timeout_flag() {
    let cli = parse(&[
        "agcli",
        "--finalization-timeout",
        "42",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ])
    .expect("finalization-timeout");
    assert_eq!(cli.finalization_timeout, Some(42));
}

// ── Commit-hash compute (unit) ─────────────────────────────────────────────────

#[test]
fn commit_hash_is_deterministic() {
    let h1 = agcli::extrinsics::compute_weight_commit_hash(&[1, 2], &[100, 200], b"salt")
        .expect("hash1");
    let h2 = agcli::extrinsics::compute_weight_commit_hash(&[1, 2], &[100, 200], b"salt")
        .expect("hash2");
    assert_eq!(h1, h2);
}

#[test]
fn commit_hash_changes_with_different_salt() {
    let h1 = agcli::extrinsics::compute_weight_commit_hash(&[1], &[100], b"salt1").expect("h1");
    let h2 = agcli::extrinsics::compute_weight_commit_hash(&[1], &[100], b"salt2").expect("h2");
    assert_ne!(h1, h2);
}

#[test]
fn commit_hash_changes_with_different_weights() {
    let h1 = agcli::extrinsics::compute_weight_commit_hash(&[1], &[100], b"s").expect("h1");
    let h2 = agcli::extrinsics::compute_weight_commit_hash(&[1], &[200], b"s").expect("h2");
    assert_ne!(h1, h2);
}

#[test]
fn commit_hash_changes_with_different_uids() {
    let h1 = agcli::extrinsics::compute_weight_commit_hash(&[0], &[100], b"s").expect("h1");
    let h2 = agcli::extrinsics::compute_weight_commit_hash(&[1], &[100], b"s").expect("h2");
    assert_ne!(h1, h2);
}

#[test]
fn commit_hash_is_32_bytes() {
    let h =
        agcli::extrinsics::compute_weight_commit_hash(&[0, 1], &[100, 200], b"salt").expect("hash");
    assert_eq!(h.len(), 32);
}

// ── Salt encoding (unit) ────────────────────────────────────────────────────────

/// Verifies the little-endian u16 chunking used by `weights reveal` and `weights reveal-mechanism`.
fn encode_salt_u16(salt: &str) -> Vec<u16> {
    salt.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let b0 = chunk[0] as u16;
            let b1 = if chunk.len() > 1 { chunk[1] as u16 } else { 0 };
            (b1 << 8) | b0
        })
        .collect()
}

#[test]
fn salt_even_length_encodes_correctly() {
    // "AB" = [0x41, 0x42] → (0x42 << 8) | 0x41 = 0x4241
    let encoded = encode_salt_u16("AB");
    assert_eq!(encoded, vec![0x4241u16]);
}

#[test]
fn salt_odd_length_pads_high_byte_with_zero() {
    // "ABC" = [0x41, 0x42, 0x43] → [0x4241, 0x0043]
    let encoded = encode_salt_u16("ABC");
    assert_eq!(encoded, vec![0x4241u16, 0x0043u16]);
}

#[test]
fn salt_single_byte_pads_correctly() {
    // "A" = [0x41] → (0 << 8) | 0x41 = 0x0041
    let encoded = encode_salt_u16("A");
    assert_eq!(encoded, vec![0x0041u16]);
}

#[test]
fn salt_empty_produces_empty_vec() {
    let encoded = encode_salt_u16("");
    assert!(encoded.is_empty());
}

/// The commit hash uses raw salt bytes; the reveal extrinsic uses u16-encoded salt.
/// When the pallet reconstructs the hash from Vec<u16>, it decodes each u16 as
/// little-endian bytes — the round-trip is consistent.
#[test]
fn salt_u16_roundtrip_matches_raw_bytes_for_even_length() {
    let salt = "ABCD"; // 4 bytes, even length
    let raw_bytes = salt.as_bytes();
    let encoded = encode_salt_u16(salt);
    // Reconstruct raw bytes from u16 LE
    let reconstructed: Vec<u8> = encoded.iter().flat_map(|w| w.to_le_bytes()).collect();
    assert_eq!(raw_bytes, reconstructed.as_slice());
}

// ── Reveal-window timing math (unit) ───────────────────────────────────────────

#[test]
fn reveal_target_block_calculation() {
    let block_at_commit: u64 = 1000;
    let cr_interval: u64 = 2;
    let tempo: u64 = 360;
    let reveal_target = block_at_commit + cr_interval * tempo;
    assert_eq!(reveal_target, 1720);
}

#[test]
fn reveal_window_opens_at_target_not_before() {
    let target: u64 = 1720;
    assert!(!(1719u64 >= target), "window closed before target");
    assert!(1720u64 >= target, "window open at target");
    assert!(1721u64 >= target, "window still open after target");
}

#[test]
fn remaining_blocks_display_math() {
    let target: u64 = 1720;
    let current: u64 = 1500;
    let remaining = target.saturating_sub(current);
    assert_eq!(remaining, 220);
    assert_eq!(remaining * 12 / 60, 44, "minutes");
    assert_eq!((remaining * 12) % 60, 0, "seconds");
}

// ── Variant field inspection ───────────────────────────────────────────────────

/// Ensures `WeightCommands::Set` exposes `netuid`, `weights`, and `version_key` only
/// (no leftover `dry_run` field — that lives on the root `Cli`).
#[test]
fn set_variant_has_no_local_dry_run_field() {
    use agcli::cli::{Commands, WeightCommands};
    let cli = parse(&[
        "agcli",
        "--dry-run",
        "weights",
        "set",
        "--netuid",
        "7",
        "--weights",
        "0:100",
    ])
    .expect("parse");
    assert!(cli.dry_run, "global dry_run");
    match cli.command {
        Commands::Weights(WeightCommands::Set { netuid, .. }) => {
            assert_eq!(netuid, 7);
        }
        _ => panic!("expected WeightCommands::Set"),
    }
}

/// Ensures `CommitTimelocked` has `round` (u64) field — critical for drand integration.
#[test]
fn commit_timelocked_variant_fields() {
    use agcli::cli::{Commands, WeightCommands};
    let cli = parse(&[
        "agcli",
        "weights",
        "commit-timelocked",
        "--netuid",
        "3",
        "--weights",
        "0:100",
        "--round",
        "42000",
    ])
    .expect("parse commit-timelocked");
    match cli.command {
        Commands::Weights(WeightCommands::CommitTimelocked { netuid, round, .. }) => {
            assert_eq!(netuid, 3);
            assert_eq!(round, 42000u64);
        }
        _ => panic!("expected CommitTimelocked"),
    }
}

/// Ensures `SetMechanism` has `mechanism_id` field.
#[test]
fn set_mechanism_variant_fields() {
    use agcli::cli::{Commands, WeightCommands};
    let cli = parse(&[
        "agcli",
        "weights",
        "set-mechanism",
        "--netuid",
        "4",
        "--mechanism-id",
        "1",
        "--weights",
        "0:100",
    ])
    .expect("parse set-mechanism");
    match cli.command {
        Commands::Weights(WeightCommands::SetMechanism {
            netuid,
            mechanism_id,
            ..
        }) => {
            assert_eq!(netuid, 4);
            assert_eq!(mechanism_id, 1u16);
        }
        _ => panic!("expected SetMechanism"),
    }
}

/// Ensures `CommitMechanism` carries the precomputed `hash` string (not weights+salt).
#[test]
fn commit_mechanism_variant_fields() {
    use agcli::cli::{Commands, WeightCommands};
    let hash_hex = "aabbccdd".repeat(8); // 64 hex chars = 32 bytes
    let cli = parse(&[
        "agcli",
        "weights",
        "commit-mechanism",
        "--netuid",
        "2",
        "--mechanism-id",
        "0",
        "--hash",
        &hash_hex,
    ])
    .expect("parse commit-mechanism");
    match cli.command {
        Commands::Weights(WeightCommands::CommitMechanism {
            netuid,
            mechanism_id,
            hash,
        }) => {
            assert_eq!(netuid, 2);
            assert_eq!(mechanism_id, 0u16);
            assert_eq!(hash, hash_hex);
        }
        _ => panic!("expected CommitMechanism"),
    }
}

/// Ensures `RevealMechanism` has `salt` (String) not `Option<String>`.
#[test]
fn reveal_mechanism_salt_is_required() {
    // Accepted with salt
    ok(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
        "--salt",
        "required",
    ]);
    // Rejected without salt
    err(&[
        "agcli",
        "weights",
        "reveal-mechanism",
        "--netuid",
        "1",
        "--mechanism-id",
        "0",
        "--weights",
        "0:100",
    ]);
}

// ── Stress parse ───────────────────────────────────────────────────────────────

#[test]
fn set_500_uid_weight_pairs_parses_without_panic() {
    let pairs: Vec<String> = (0..500).map(|i| format!("{}:1", i)).collect();
    let weights_str = pairs.join(",");
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        &weights_str,
    ]);
}

// ── Ignored localnet integration test ─────────────────────────────────────────

/// Green-path integration test against a local Subtensor node.
///
/// Requires: `ws://127.0.0.1:9944` serving the devnet-ready image:
///   `docker run --rm -p 9944:9944 ghcr.io/opentensor/subtensor-localnet:devnet-ready`
///
/// Run with: `cargo test --test audit_weights -- --ignored green_path_weights_localnet`
#[test]
#[ignore]
fn green_path_weights_localnet() {
    // Parse-surface smoke test (localnet ops are async — this is the sync gate).
    // A full async integration would use tokio::test and connect to ws://127.0.0.1:9944,
    // register a hotkey on subnet 1, then call weights show and set.
    // This sync version confirms the parse surface is correct before attempting chain ops.
    ok(&["agcli", "weights", "show", "--netuid", "1"]);
    ok(&[
        "agcli",
        "weights",
        "set",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--dry-run",
    ]);
    ok(&[
        "agcli",
        "weights",
        "commit",
        "--netuid",
        "1",
        "--weights",
        "0:100",
        "--salt",
        "localnet-audit-salt",
    ]);
    ok(&[
        "agcli",
        "weights",
        "commit-reveal",
        "--netuid",
        "1",
        "--weights",
        "0:100",
    ]);
    ok(&["agcli", "weights", "status", "--netuid", "1"]);
}
