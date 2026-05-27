use agcli::cli::{Cli, Commands};
use clap::Parser;

const CANONICAL_TOPICS: &[&str] = &[
    "tempo",
    "commit-reveal",
    "yuma",
    "rate-limits",
    "weights",
    "stake-weight",
    "amm",
    "bootstrap",
    "alpha",
    "emission",
    "registration",
    "subnets",
    "validators",
    "miners",
    "immunity",
    "delegation",
    "childkeys",
    "root",
    "proxy",
    "coldkey-swap",
    "governance",
    "senate",
    "mev-shield",
    "limits",
    "hyperparams",
    "axon",
    "take",
    "recycle",
    "pow",
    "archive",
    "diff",
    "owner-workflow",
];

const ALIAS_TOPICS: &[(&str, &str)] = &[
    ("commitreveal", "commit-reveal"),
    ("cr", "commit-reveal"),
    ("yumaconsensus", "yuma"),
    ("ratelimit", "rate-limits"),
    ("ratelimits", "rate-limits"),
    ("weightsratelimit", "rate-limits"),
    ("settingweights", "weights"),
    ("setweights", "weights"),
    ("weightsetting", "weights"),
    ("stakeweight", "stake-weight"),
    ("stakeweightminimum", "stake-weight"),
    ("1000", "stake-weight"),
    ("dynamictao", "amm"),
    ("dtao", "amm"),
    ("pool", "amm"),
    ("alphatoken", "alpha"),
    ("emissions", "emission"),
    ("register", "registration"),
    ("subnet", "subnets"),
    ("validator", "validators"),
    ("miner", "miners"),
    ("immunityperiod", "immunity"),
    ("delegate", "delegation"),
    ("nominate", "delegation"),
    ("childkey", "childkeys"),
    ("rootnetwork", "root"),
    ("coldkeyswap", "coldkey-swap"),
    ("coldkey", "coldkey-swap"),
    ("ckswap", "coldkey-swap"),
    ("gov", "governance"),
    ("proposals", "governance"),
    ("triumvirate", "senate"),
    ("mevshield", "mev-shield"),
    ("mev", "mev-shield"),
    ("mevprotection", "mev-shield"),
    ("networklimits", "limits"),
    ("chainlimits", "limits"),
    ("hyperparameters", "hyperparams"),
    ("params", "hyperparams"),
    ("axoninfo", "axon"),
    ("serving", "axon"),
    ("delegatetake", "take"),
    ("validatortake", "take"),
    ("recyclealpha", "recycle"),
    ("burn", "recycle"),
    ("burnalpha", "recycle"),
    ("powregistration", "pow"),
    ("proofofwork", "pow"),
    ("archivenode", "archive"),
    ("historical", "archive"),
    ("wayback", "archive"),
    ("compare", "diff"),
    ("historicaldiff", "diff"),
    ("ownerworkflow", "owner-workflow"),
    ("ow", "owner-workflow"),
    ("subnetowner", "owner-workflow"),
    ("ownerguide", "owner-workflow"),
];

#[test]
fn parse_explain_surface_for_all_canonical_topics() {
    for topic in CANONICAL_TOPICS {
        let cli = Cli::try_parse_from(["agcli", "explain", "--topic", *topic])
            .unwrap_or_else(|e| panic!("failed to parse canonical topic {topic}: {e}"));
        match &cli.command {
            Commands::Explain {
                topic: parsed,
                full,
            } => {
                assert_eq!(parsed.as_deref(), Some(*topic));
                assert!(!full);
            }
            _ => panic!("expected explain command for topic {topic}"),
        }
    }
}

#[test]
fn parse_explain_surface_for_all_alias_topics() {
    for (alias, canonical) in ALIAS_TOPICS {
        let cli = Cli::try_parse_from(["agcli", "explain", "--topic", *alias])
            .unwrap_or_else(|e| panic!("failed to parse alias topic {alias}: {e}"));
        match &cli.command {
            Commands::Explain {
                topic: parsed,
                full,
            } => {
                assert_eq!(parsed.as_deref(), Some(*alias));
                assert!(!full);
            }
            _ => panic!("expected explain command for alias {alias} -> {canonical}"),
        }
    }
}

#[test]
fn parse_explain_list_and_full_modes() {
    let list_cli = Cli::try_parse_from(["agcli", "explain"]).expect("explain list should parse");
    match &list_cli.command {
        Commands::Explain { topic, full } => {
            assert!(topic.is_none());
            assert!(!full);
        }
        _ => panic!("expected explain command for list mode"),
    }

    let full_index =
        Cli::try_parse_from(["agcli", "explain", "--full"]).expect("explain --full should parse");
    match &full_index.command {
        Commands::Explain { topic, full } => {
            assert!(topic.is_none());
            assert!(*full);
        }
        _ => panic!("expected explain command for full index mode"),
    }

    let full_topic = Cli::try_parse_from([
        "agcli", "--output", "json", "explain", "--topic", "weights", "--full",
    ])
    .expect("explain --topic weights --full should parse");
    match &full_topic.command {
        Commands::Explain { topic, full } => {
            assert_eq!(topic.as_deref(), Some("weights"));
            assert!(*full);
        }
        _ => panic!("expected explain command for full topic mode"),
    }
}

#[tokio::test]
#[ignore = "requires docker localnet runtime"]
async fn green_path_explain_localnet_smoke() -> anyhow::Result<()> {
    let mut cfg = agcli::localnet::LocalnetConfig::default();
    cfg.container_name = format!("agcli-audit-explain-{}", std::process::id());
    cfg.wait_timeout = 180;

    let info = agcli::localnet::start(&cfg).await?;
    let test_result = async {
        let client = agcli::Client::connect(&info.endpoint).await?;
        let block = client.get_block_number().await?;
        assert!(block > 0, "expected localnet block height > 0, got {block}");

        let cli = Cli::try_parse_from([
            "agcli",
            "--network",
            "local",
            "explain",
            "--topic",
            "tempo",
            "--output",
            "json",
        ])?;
        match &cli.command {
            Commands::Explain { topic, full } => {
                assert_eq!(topic.as_deref(), Some("tempo"));
                assert!(!full);
            }
            _ => panic!("expected explain command in localnet smoke test"),
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;

    let _ = agcli::localnet::stop(&info.container_name);
    test_result
}
