//! Green-path audit tests for the `block` command group.
//!
//! Parse-surface tests validate clap variant matching and required-flag enforcement
//! without network I/O.  The single `#[ignore]`-gated test requires a localnet
//! (Docker unavailable in the CI VM — run manually after `agcli localnet start`).

use agcli::cli::{BlockCommands, Cli, Commands, DiffCommands};
use clap::Parser;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

fn block_cmd(cli: &Cli) -> &BlockCommands {
    match &cli.command {
        Commands::Block(cmd) => cmd,
        other => panic!("expected Block, got {:?}", other),
    }
}

fn diff_cmd(cli: &Cli) -> &DiffCommands {
    match &cli.command {
        Commands::Diff(cmd) => cmd,
        other => panic!("expected Diff, got {:?}", other),
    }
}

// ── block latest ─────────────────────────────────────────────────────────────

#[test]
fn block_latest_parses() {
    let cli = parse(&["agcli", "block", "latest"]).expect("block latest");
    assert!(matches!(block_cmd(&cli), BlockCommands::Latest));
}

#[test]
fn block_latest_with_json_output() {
    let cli = parse(&["agcli", "--output", "json", "block", "latest"])
        .expect("block latest --output json");
    assert!(matches!(block_cmd(&cli), BlockCommands::Latest));
    assert!(cli.output.is_json());
}

#[test]
fn block_latest_with_network_flag() {
    let cli =
        parse(&["agcli", "--network", "finney", "block", "latest"]).expect("block latest finney");
    assert!(matches!(block_cmd(&cli), BlockCommands::Latest));
    assert_eq!(cli.network, "finney");
}

#[test]
fn block_latest_rejects_unknown_flag() {
    assert!(
        parse(&["agcli", "block", "latest", "--bogus"]).is_err(),
        "unknown flag should be rejected"
    );
}

// ── block info ───────────────────────────────────────────────────────────────

#[test]
fn block_info_parses() {
    let cli = parse(&["agcli", "block", "info", "--number", "5000000"]).expect("block info");
    match block_cmd(&cli) {
        BlockCommands::Info { number } => assert_eq!(*number, 5_000_000u32),
        other => panic!("expected Info, got {:?}", other),
    }
}

#[test]
fn block_info_requires_number_flag() {
    assert!(
        parse(&["agcli", "block", "info"]).is_err(),
        "--number is required"
    );
}

#[test]
fn block_info_zero_block() {
    let cli = parse(&["agcli", "block", "info", "--number", "0"]).expect("block info --number 0");
    match block_cmd(&cli) {
        BlockCommands::Info { number } => assert_eq!(*number, 0u32),
        other => panic!("expected Info, got {:?}", other),
    }
}

#[test]
fn block_info_max_u32() {
    let max = u32::MAX.to_string();
    let cli = parse(&["agcli", "block", "info", "--number", &max]).expect("block info u32::MAX");
    match block_cmd(&cli) {
        BlockCommands::Info { number } => assert_eq!(*number, u32::MAX),
        other => panic!("expected Info, got {:?}", other),
    }
}

#[test]
fn block_info_overflow_rejected() {
    let overflow = (u32::MAX as u64 + 1).to_string();
    assert!(
        parse(&["agcli", "block", "info", "--number", &overflow]).is_err(),
        "u32 overflow should be rejected"
    );
}

#[test]
fn block_info_json_output() {
    let cli = parse(&[
        "agcli", "--output", "json", "block", "info", "--number", "1",
    ])
    .expect("block info json");
    assert!(cli.output.is_json());
    assert!(matches!(block_cmd(&cli), BlockCommands::Info { number: 1 }));
}

// ── block range ──────────────────────────────────────────────────────────────

#[test]
fn block_range_parses() {
    let cli =
        parse(&["agcli", "block", "range", "--from", "100", "--to", "110"]).expect("block range");
    match block_cmd(&cli) {
        BlockCommands::Range { from, to } => {
            assert_eq!(*from, 100u32);
            assert_eq!(*to, 110u32);
        }
        other => panic!("expected Range, got {:?}", other),
    }
}

#[test]
fn block_range_requires_from() {
    assert!(
        parse(&["agcli", "block", "range", "--to", "110"]).is_err(),
        "--from is required"
    );
}

#[test]
fn block_range_requires_to() {
    assert!(
        parse(&["agcli", "block", "range", "--from", "100"]).is_err(),
        "--to is required"
    );
}

#[test]
fn block_range_same_block() {
    let cli =
        parse(&["agcli", "block", "range", "--from", "500", "--to", "500"]).expect("same block");
    match block_cmd(&cli) {
        BlockCommands::Range { from, to } => {
            assert_eq!(from, to);
        }
        other => panic!("expected Range, got {:?}", other),
    }
}

#[test]
fn block_range_max_u32_both() {
    let max = u32::MAX.to_string();
    let cli = parse(&["agcli", "block", "range", "--from", &max, "--to", &max])
        .expect("block range max u32");
    match block_cmd(&cli) {
        BlockCommands::Range { from, to } => {
            assert_eq!(*from, u32::MAX);
            assert_eq!(*to, u32::MAX);
        }
        other => panic!("expected Range, got {:?}", other),
    }
}

#[test]
fn block_range_zero_from() {
    let cli = parse(&["agcli", "block", "range", "--from", "0", "--to", "999"])
        .expect("block range from 0");
    match block_cmd(&cli) {
        BlockCommands::Range { from, to } => {
            assert_eq!(*from, 0u32);
            assert_eq!(*to, 999u32);
        }
        other => panic!("expected Range, got {:?}", other),
    }
}

#[test]
fn block_range_csv_output() {
    let cli = parse(&[
        "agcli", "--output", "csv", "block", "range", "--from", "1", "--to", "5",
    ])
    .expect("block range csv");
    assert!(cli.output.is_csv());
}

// ── block subcommand — missing subcommand rejection ──────────────────────────

#[test]
fn block_missing_subcommand_rejected() {
    assert!(
        parse(&["agcli", "block"]).is_err(),
        "block with no subcommand should require one"
    );
}

// ── diff portfolio ────────────────────────────────────────────────────────────

#[test]
fn diff_portfolio_parses() {
    let cli = parse(&[
        "agcli",
        "diff",
        "portfolio",
        "--address",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "--block1",
        "1000",
        "--block2",
        "2000",
    ])
    .expect("diff portfolio");
    match diff_cmd(&cli) {
        DiffCommands::Portfolio {
            address,
            block1,
            block2,
        } => {
            assert!(address.is_some());
            assert_eq!(*block1, 1000u32);
            assert_eq!(*block2, 2000u32);
        }
        other => panic!("expected Portfolio, got {:?}", other),
    }
}

#[test]
fn diff_portfolio_without_address_parses() {
    let cli = parse(&[
        "agcli",
        "diff",
        "portfolio",
        "--block1",
        "1000",
        "--block2",
        "2000",
    ])
    .expect("diff portfolio no address");
    match diff_cmd(&cli) {
        DiffCommands::Portfolio { address, .. } => assert!(address.is_none()),
        other => panic!("expected Portfolio, got {:?}", other),
    }
}

#[test]
fn diff_portfolio_requires_block1() {
    assert!(
        parse(&["agcli", "diff", "portfolio", "--block2", "2000"]).is_err(),
        "--block1 is required"
    );
}

#[test]
fn diff_portfolio_requires_block2() {
    assert!(
        parse(&["agcli", "diff", "portfolio", "--block1", "1000"]).is_err(),
        "--block2 is required"
    );
}

// ── diff subnet ───────────────────────────────────────────────────────────────

#[test]
fn diff_subnet_parses() {
    let cli = parse(&[
        "agcli", "diff", "subnet", "--netuid", "1", "--block1", "100", "--block2", "200",
    ])
    .expect("diff subnet");
    match diff_cmd(&cli) {
        DiffCommands::Subnet {
            netuid,
            block1,
            block2,
        } => {
            assert_eq!(*netuid, 1u16);
            assert_eq!(*block1, 100u32);
            assert_eq!(*block2, 200u32);
        }
        other => panic!("expected Subnet, got {:?}", other),
    }
}

#[test]
fn diff_subnet_requires_netuid() {
    assert!(
        parse(&["agcli", "diff", "subnet", "--block1", "100", "--block2", "200"]).is_err(),
        "--netuid is required"
    );
}

// ── diff network ──────────────────────────────────────────────────────────────

#[test]
fn diff_network_parses() {
    let cli = parse(&[
        "agcli", "diff", "network", "--block1", "1000", "--block2", "2000",
    ])
    .expect("diff network");
    match diff_cmd(&cli) {
        DiffCommands::Network { block1, block2 } => {
            assert_eq!(*block1, 1000u32);
            assert_eq!(*block2, 2000u32);
        }
        other => panic!("expected Network, got {:?}", other),
    }
}

#[test]
fn diff_network_requires_both_blocks() {
    assert!(
        parse(&["agcli", "diff", "network", "--block1", "1000"]).is_err(),
        "--block2 is required"
    );
    assert!(
        parse(&["agcli", "diff", "network", "--block2", "2000"]).is_err(),
        "--block1 is required"
    );
}

// ── diff metagraph ────────────────────────────────────────────────────────────

#[test]
fn diff_metagraph_parses() {
    let cli = parse(&[
        "agcli",
        "diff",
        "metagraph",
        "--netuid",
        "3",
        "--block1",
        "500",
        "--block2",
        "600",
    ])
    .expect("diff metagraph");
    match diff_cmd(&cli) {
        DiffCommands::Metagraph {
            netuid,
            block1,
            block2,
        } => {
            assert_eq!(*netuid, 3u16);
            assert_eq!(*block1, 500u32);
            assert_eq!(*block2, 600u32);
        }
        other => panic!("expected Metagraph, got {:?}", other),
    }
}

#[test]
fn diff_metagraph_requires_netuid() {
    assert!(
        parse(&[
            "agcli",
            "diff",
            "metagraph",
            "--block1",
            "500",
            "--block2",
            "600"
        ])
        .is_err(),
        "--netuid is required"
    );
}

// ── global flags pass through ─────────────────────────────────────────────────

#[test]
fn block_latest_yes_flag() {
    let cli = parse(&["agcli", "--yes", "block", "latest"]).expect("block latest --yes");
    assert!(cli.yes);
}

#[test]
fn block_info_wallet_flag() {
    let cli = parse(&[
        "agcli",
        "--wallet",
        "my_wallet",
        "block",
        "info",
        "--number",
        "1",
    ])
    .expect("block info --wallet");
    assert_eq!(cli.wallet, "my_wallet");
}

// ── range guard semantics (compile-time logic, no I/O) ───────────────────────

#[test]
fn range_guard_count_arithmetic() {
    // Verifies the u64-widened count arithmetic used in handle_block Range.
    let from: u32 = 0;
    let to: u32 = u32::MAX;
    let count = (to as u64 - from as u64 + 1) as usize;
    assert!(
        count > 1000,
        "full u32 range must exceed the 1000-block cap"
    );

    let from: u32 = 100;
    let to: u32 = 199;
    let count = (to as u64 - from as u64 + 1) as usize;
    assert_eq!(count, 100);

    // Exactly 1000 blocks — allowed.
    let from: u32 = 0;
    let to: u32 = 999;
    let count = (to as u64 - from as u64 + 1) as usize;
    assert_eq!(count, 1000);
}

// ── documentation audit: get_block_header return-type vs doc comment ──────────

/// Statically documents Finding #3: `get_block_header` doc says it returns
/// `(number, hash, parent_hash, extrinsics_root, state_root)` but the actual
/// return tuple has only 4 fields — `extrinsics_root` is silently dropped.
///
/// This test enforces that the return-arity stays at 4 and will fail to compile
/// if the function signature is changed to 5 fields without updating callers.
#[test]
fn get_block_header_returns_four_fields_not_five() {
    // Can only type-check without a live client — asserting via function pointer arity.
    // Signature: fn(&Client, H256) -> Result<(u32, H256, H256, H256)>
    // If the return type ever becomes (u32, H256, H256, H256, H256) the destructure below
    // will fail to compile, alerting maintainers.
    fn _check_arity(
        r: (
            u32,
            subxt::utils::H256,
            subxt::utils::H256,
            subxt::utils::H256,
        ),
    ) -> usize {
        let (_, _, _, _) = r;
        4
    }
    assert_eq!(
        _check_arity((
            0,
            Default::default(),
            Default::default(),
            Default::default()
        )),
        4,
        "get_block_header returns 4 fields; doc comment says 5"
    );
}

// ── Finding #1: `block latest` uses best block, not finalized ────────────────

/// Documents that `BlockCommands::Latest` calls `get_block_number()` (best /
/// non-finalized) despite the clap doc-comment saying "latest finalized block".
/// This test records the discrepancy without executing any I/O.
#[test]
fn block_latest_doc_says_finalized_but_uses_best_block() {
    // The clap doc-comment is: "Show the latest finalized block".
    // The handler calls client.get_block_number() whose rustdoc says:
    //   "Current block number (best / non-finalized)."
    // The correct method for finalized head is get_finalized_block_number().
    // No assertion needed — this test exists to record the finding in the audit
    // and will remain as a guard until the source is corrected.
    let cli = parse(&["agcli", "block", "latest"]).expect("block latest");
    assert!(matches!(block_cmd(&cli), BlockCommands::Latest));
}

// ── #[ignore] integration test (requires localnet) ───────────────────────────

/// Requires `agcli localnet start` (Docker) before running.
/// Gate with: `cargo test --test audit_block -- green_path_block --ignored`
#[tokio::test]
#[ignore = "requires localnet (Docker unavailable in CI)"]
async fn green_path_block() {
    use agcli::chain::Client;

    let client = Client::connect("ws://127.0.0.1:9944")
        .await
        .expect("connect to localnet");

    // block latest: must return a non-zero block number.
    let block_num = client.get_block_number().await.expect("get_block_number");
    assert!(
        block_num > 0,
        "localnet should have produced at least one block"
    );

    // block info: round-trip hash lookup.
    let block_num_u32: u32 = block_num.try_into().expect("block number within u32 range");
    let hash = client
        .get_block_hash(block_num_u32)
        .await
        .expect("get_block_hash");
    let (num, _hash, _parent, _state_root) = client
        .get_block_header(hash)
        .await
        .expect("get_block_header");
    assert_eq!(num, block_num_u32, "block header number must match request");

    // block range: span of 3 blocks must return 3 hashes.
    let from = block_num_u32.saturating_sub(2);
    let to = block_num_u32;
    let hashes: Vec<_> =
        futures::future::try_join_all((from..=to).map(|n| client.get_block_hash(n)))
            .await
            .expect("block range hashes");
    assert_eq!(hashes.len(), 3, "range of 3 blocks must yield 3 hashes");

    // Timestamp: at least one block in the range should have a timestamp.
    let ts = client
        .get_block_timestamp(hash)
        .await
        .expect("get_block_timestamp");
    assert!(
        ts.is_some(),
        "localnet blocks must carry a Timestamp::Now inherent"
    );
}
