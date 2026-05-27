//! Commitment command-group audit tests.
//! Run parse checks:
//!   cargo test --no-run --test audit_commitment
//!
//! Run ignored integration test manually:
//!   cargo test --test audit_commitment -- --ignored --nocapture

use clap::Parser;

#[test]
fn parse_surface_commitment_subcommands_with_realistic_args() {
    let scenarios: [&[&str]; 3] = [
        &[
            "agcli",
            "commitment",
            "set",
            "--netuid",
            "97",
            "--data",
            "endpoint:http://127.0.0.1:8091,version:1.0,protocol:http",
        ],
        &[
            "agcli",
            "--output",
            "json",
            "commitment",
            "get",
            "--netuid",
            "97",
            "--hotkey-address",
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        ],
        &[
            "agcli",
            "--output",
            "json",
            "commitment",
            "list",
            "--netuid",
            "97",
        ],
    ];

    for args in scenarios {
        let parsed = agcli::cli::Cli::try_parse_from(args);
        assert!(
            parsed.is_ok(),
            "failed to parse args {:?}: {:?}",
            args,
            parsed
        );
    }
}

#[tokio::test]
#[ignore = "requires a running local chain (default ws://127.0.0.1:9944)"]
async fn green_path_commitment_local_chain() {
    let ws = std::env::var("AGCLI_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let netuid = std::env::var("AGCLI_COMMITMENT_NETUID")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(1);

    let client = agcli::chain::Client::connect(&ws)
        .await
        .unwrap_or_else(|e| panic!("connect {} failed: {}", ws, e));

    let listed = client
        .get_all_commitments(netuid)
        .await
        .unwrap_or_else(|e| panic!("list commitments failed on netuid {}: {}", netuid, e));
    println!(
        "commitment list on {} netuid {} returned {} entries",
        ws,
        netuid,
        listed.len()
    );

    if std::env::var("AGCLI_AUDIT_COMMITMENT_WRITE")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }

    use sp_core::{crypto::Ss58Codec, sr25519, Pair};

    let signer = sr25519::Pair::from_string("//Alice", None)
        .unwrap_or_else(|e| panic!("failed to derive //Alice signer: {}", e));
    let signer_ss58 = signer.public().to_ss58check();
    let payload = "endpoint:http://127.0.0.1:8091,version:1.0";

    let tx = client
        .set_commitment(&signer, netuid, payload)
        .await
        .unwrap_or_else(|e| panic!("set commitment failed for {}: {}", signer_ss58, e));
    assert!(!tx.is_empty(), "expected non-empty tx hash");

    let fetched = client
        .get_commitment(netuid, &signer_ss58)
        .await
        .unwrap_or_else(|e| panic!("get commitment failed for {}: {}", signer_ss58, e));
    let (_block, fields) = fetched.unwrap_or_else(|| {
        panic!(
            "expected commitment for {} after set on netuid {}",
            signer_ss58, netuid
        )
    });
    assert!(
        fields
            .iter()
            .any(|f| f.contains("endpoint:http://127.0.0.1:8091")),
        "expected endpoint field in returned commitment fields: {:?}",
        fields
    );
}
