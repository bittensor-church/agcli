use clap::Parser;

#[test]
fn parse_surface_subnet_all_subcommands() {
    let cases: Vec<Vec<&str>> = vec![
        vec!["agcli", "subnet", "list"],
        vec!["agcli", "subnet", "list", "--at-block", "500000"],
        vec!["agcli", "subnet", "show", "--netuid", "1"],
        vec!["agcli", "subnet", "info", "--netuid", "1"],
        vec!["agcli", "subnet", "hyperparams", "--netuid", "1", "--at-block", "400000"],
        vec![
            "agcli",
            "subnet",
            "metagraph",
            "--netuid",
            "1",
            "--uid",
            "3",
            "--full",
            "--save",
        ],
        vec!["agcli", "subnet", "cache-load", "--netuid", "1", "--block", "777"],
        vec!["agcli", "subnet", "cache-list", "--netuid", "1"],
        vec![
            "agcli",
            "subnet",
            "cache-diff",
            "--netuid",
            "1",
            "--from-block",
            "700",
            "--to-block",
            "777",
        ],
        vec!["agcli", "subnet", "cache-prune", "--netuid", "1", "--keep", "5"],
        vec![
            "agcli",
            "subnet",
            "probe",
            "--netuid",
            "1",
            "--uids",
            "0,1,2",
            "--timeout-ms",
            "2500",
            "--concurrency",
            "16",
        ],
        vec!["agcli", "subnet", "register"],
        vec!["agcli", "subnet", "create-cost"],
        vec![
            "agcli",
            "subnet",
            "register-with-identity",
            "--name",
            "audit-subnet",
            "--github",
            "opentensor/subtensor",
            "--contact",
            "ops@example.com",
            "--url",
            "https://example.com/subnet",
            "--discord",
            "discord.gg/example",
            "--description",
            "Audit subnet registration path",
            "--additional",
            "integration-audit",
        ],
        vec!["agcli", "subnet", "register-leased", "--end-block", "123456"],
        vec!["agcli", "subnet", "terminate-lease", "--netuid", "1"],
        vec!["agcli", "subnet", "root-dissolve", "--netuid", "1"],
        vec!["agcli", "subnet", "register-neuron", "--netuid", "1"],
        vec!["agcli", "subnet", "pow", "--netuid", "1", "--threads", "8"],
        vec!["agcli", "subnet", "dissolve", "--netuid", "1"],
        vec!["agcli", "subnet", "watch", "--netuid", "1", "--interval", "12"],
        vec!["agcli", "subnet", "liquidity"],
        vec!["agcli", "subnet", "liquidity", "--netuid", "1"],
        vec![
            "agcli",
            "subnet",
            "monitor",
            "--netuid",
            "1",
            "--interval",
            "24",
            "--json",
        ],
        vec!["agcli", "subnet", "health", "--netuid", "1"],
        vec!["agcli", "subnet", "emissions", "--netuid", "1"],
        vec!["agcli", "subnet", "cost", "--netuid", "1"],
        vec!["agcli", "subnet", "commits", "--netuid", "1"],
        vec![
            "agcli",
            "subnet",
            "commits",
            "--netuid",
            "1",
            "--hotkey-address",
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        ],
        vec!["agcli", "subnet", "set-param", "--netuid", "1", "--param", "list"],
        vec![
            "agcli",
            "subnet",
            "set-param",
            "--netuid",
            "1",
            "--param",
            "tempo",
            "--value",
            "360",
        ],
        vec!["agcli", "subnet", "set-symbol", "--netuid", "1", "--symbol", "SN1"],
        vec!["agcli", "subnet", "emission-split", "--netuid", "1"],
        vec!["agcli", "subnet", "trim", "--netuid", "1", "--max-uids", "256"],
        vec!["agcli", "subnet", "check-start", "--netuid", "1"],
        vec!["agcli", "subnet", "start", "--netuid", "1"],
        vec!["agcli", "subnet", "mechanism-count", "--netuid", "1"],
        vec![
            "agcli",
            "subnet",
            "set-mechanism-count",
            "--netuid",
            "1",
            "--count",
            "2",
        ],
        vec![
            "agcli",
            "subnet",
            "set-emission-split",
            "--netuid",
            "1",
            "--weights",
            "32768,32767",
        ],
        vec![
            "agcli",
            "subnet",
            "snipe",
            "--netuid",
            "1",
            "--max-cost",
            "1.5",
            "--max-attempts",
            "3",
            "--all-hotkeys",
            "--fast",
            "--watch",
        ],
    ];

    for argv in cases {
        let parsed = agcli::cli::Cli::try_parse_from(argv.clone());
        assert!(parsed.is_ok(), "failed to parse {:?}: {:?}", argv, parsed.err());
    }
}

#[tokio::test]
#[ignore = "requires local subtensor node (e.g. agcli localnet start)"]
async fn green_path_subnet_local_chain_smoke() {
    let endpoint =
        std::env::var("AGCLI_LOCAL_WS").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let client = agcli::chain::Client::connect(&endpoint)
        .await
        .expect("connect local chain");

    let _cost = client
        .get_subnet_registration_cost()
        .await
        .expect("query subnet registration cost");

    let subnets = client.get_all_subnets().await.expect("query all subnets");
    assert!(!subnets.is_empty(), "expected at least one subnet on local chain");
}
