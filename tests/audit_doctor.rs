use agcli::chain::Client;
use agcli::cli::{self, Cli, Commands};
use clap::Parser;

#[test]
fn doctor_parse_surface_accepts_realistic_invocations() {
    let cases = [
        ["agcli", "doctor"].as_slice(),
        ["agcli", "--network", "test", "doctor"].as_slice(),
        ["agcli", "doctor", "--output", "json"].as_slice(),
        [
            "agcli",
            "--endpoint",
            "ws://127.0.0.1:9944",
            "--wallet-dir",
            "~/.bittensor/wallets",
            "--wallet",
            "default",
            "doctor",
        ]
        .as_slice(),
    ];

    for args in cases {
        let parsed = Cli::try_parse_from(args).expect("doctor CLI parse should succeed");
        assert!(
            matches!(parsed.command, Commands::Doctor),
            "expected Commands::Doctor for args: {args:?}"
        );
    }
}

#[test]
fn doctor_has_no_nested_subcommands() {
    let err = Cli::try_parse_from(["agcli", "doctor", "status"])
        .expect_err("doctor should reject unknown nested subcommands");
    let rendered = err.to_string();
    assert!(
        rendered.contains("unexpected argument") || rendered.contains("unrecognized subcommand"),
        "unexpected clap error text: {rendered}"
    );
}

#[tokio::test]
#[ignore = "requires a running local chain at ws://127.0.0.1:9944"]
async fn doctor_green_path_local_chain() {
    let cli = Cli::try_parse_from([
        "agcli",
        "--network",
        "local",
        "--endpoint",
        "ws://127.0.0.1:9944",
        "--wallet-dir",
        "/tmp",
        "--wallet",
        "agcli-audit",
        "--output",
        "json",
        "doctor",
    ])
    .expect("doctor CLI parse should succeed");

    let network = cli.resolve_network();
    let client = Client::connect_network(&network)
        .await
        .expect("local chain should be reachable");

    let head = client
        .get_block_number()
        .await
        .expect("block-number probe should succeed");
    let _subnets = client
        .get_total_networks()
        .await
        .expect("total-networks probe should succeed");
    assert!(head > 0, "local chain should have produced at least one block");

    cli::commands::execute(cli)
        .await
        .expect("doctor command should complete on local chain");
}
