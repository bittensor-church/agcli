use agcli::chain::Client;
use clap::Parser;
use sp_core::{sr25519, Pair as _};
use subxt::dynamic::Value;

#[test]
fn parse_surface_scheduler_subcommands() {
    let cases = vec![
        vec![
            "agcli",
            "scheduler",
            "schedule",
            "--when",
            "1200",
            "--pallet",
            "System",
            "--call",
            "remark",
            "--args",
            "[\"audit-scheduler\"]",
            "--priority",
            "32",
            "--repeat-every",
            "30",
            "--repeat-count",
            "3",
        ],
        vec![
            "agcli",
            "scheduler",
            "schedule",
            "--when",
            "2048",
            "--pallet",
            "SubtensorModule",
            "--call",
            "set_weights",
            "--args",
            "[1, [0,1], [65535,65535], 0]",
        ],
        vec![
            "agcli",
            "scheduler",
            "schedule-named",
            "--id",
            "audit_task_01",
            "--when",
            "4096",
            "--pallet",
            "System",
            "--call",
            "remark",
            "--args",
            "[\"named-audit\"]",
            "--priority",
            "128",
            "--repeat-every",
            "60",
            "--repeat-count",
            "2",
        ],
        vec![
            "agcli",
            "scheduler",
            "cancel",
            "--when",
            "4096",
            "--index",
            "0",
        ],
        vec![
            "agcli",
            "scheduler",
            "cancel-named",
            "--id",
            "audit_task_01",
        ],
    ];

    for argv in &cases {
        let parsed = agcli::cli::Cli::try_parse_from(argv);
        assert!(
            parsed.is_ok(),
            "failed to parse {:?}: {:?}",
            argv,
            parsed.err()
        );
    }
}

#[tokio::test]
#[ignore = "requires a running local subtensor chain and root signer (e.g. //Alice)"]
async fn green_path_scheduler_named_local_chain() -> anyhow::Result<()> {
    let endpoint = std::env::var("AGCLI_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".into());
    let client = Client::connect(&endpoint).await?;
    let alice = sr25519::Pair::from_string("//Alice", None)?;

    let now = client.get_block_number().await? as u32;
    let when = now.saturating_add(25);
    let task_id = [0x42_u8; 32];

    let schedule_hash = client
        .schedule_named_call(
            &alice,
            &task_id,
            when,
            None,
            128,
            "System",
            "remark",
            vec![Value::from_bytes(b"audit-scheduler-green-path".to_vec())],
        )
        .await?;
    assert!(
        !schedule_hash.is_empty(),
        "schedule tx hash should be non-empty"
    );

    let cancel_hash = client.cancel_named_scheduled(&alice, &task_id).await?;
    assert!(
        !cancel_hash.is_empty(),
        "cancel tx hash should be non-empty"
    );

    Ok(())
}
