use std::time::{SystemTime, UNIX_EPOCH};

use agcli::chain::Client;
use clap::Parser;
use sp_core::Pair;
use subxt::dynamic::Value;

#[test]
fn parse_surface_preimage_note_with_args() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "preimage",
        "note",
        "--pallet",
        "System",
        "--call",
        "remark",
        "--args",
        r#"["0x68656c6c6f", 42, true]"#,
    ]);
    assert!(
        cli.is_ok(),
        "preimage note with args should parse: {:?}",
        cli.err()
    );
}

#[test]
fn parse_surface_preimage_note_without_args() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli", "preimage", "note", "--pallet", "System", "--call", "remark",
    ]);
    assert!(
        cli.is_ok(),
        "preimage note without args should parse: {:?}",
        cli.err()
    );
}

#[test]
fn parse_surface_preimage_unnote() {
    let cli = agcli::cli::Cli::try_parse_from([
        "agcli",
        "preimage",
        "unnote",
        "--hash",
        "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ]);
    assert!(cli.is_ok(), "preimage unnote should parse: {:?}", cli.err());
}

#[tokio::test]
#[ignore = "requires a running local chain (default ws://127.0.0.1:9944)"]
async fn green_path_preimage_local_chain() -> anyhow::Result<()> {
    let endpoint =
        std::env::var("AGCLI_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let client = Client::connect(&endpoint).await?;
    let signer = sp_core::sr25519::Pair::from_string("//Alice", None)
        .map_err(|e| anyhow::anyhow!("failed to build //Alice keypair: {}", e))?;

    let mut preimage_bytes = b"agcli-audit-preimage".to_vec();
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    preimage_bytes.extend_from_slice(&nonce.to_le_bytes());

    let preimage_hash = sp_core::hashing::blake2_256(&preimage_bytes);

    let note_tx = client
        .submit_raw_call(
            &signer,
            "Preimage",
            "note_preimage",
            vec![Value::from_bytes(preimage_bytes)],
        )
        .await?;
    assert!(!note_tx.is_empty(), "note_preimage returned empty tx hash");

    let unnote_tx = client.unnote_preimage(&signer, preimage_hash).await?;
    assert!(
        !unnote_tx.is_empty(),
        "unnote_preimage returned empty tx hash"
    );

    Ok(())
}
