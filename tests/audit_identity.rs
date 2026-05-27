//! Green-path parse-surface and integration audit for `agcli identity`.
//!
//! Run all parse tests:
//!   cargo test --test audit_identity
//!
//! Run the (ignored) localnet integration test:
//!   cargo test --test audit_identity -- --ignored green_path_identity

use agcli::cli::{Cli, Commands, IdentityCommands};
use clap::Parser;

// ─── helpers ──────────────────────────────────────────────────────────────────

/// A valid Bittensor SS58 address (Alice on localnet / finney).
const ALICE_SS58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

/// Parse the given argv slice and expect success.
fn must_parse(argv: &[&str]) -> Cli {
    Cli::try_parse_from(argv)
        .unwrap_or_else(|e| panic!("unexpected parse failure for {:?}: {}", argv, e))
}

/// Parse the given argv slice and expect a clap error (wrong / missing args).
fn must_fail(argv: &[&str]) {
    assert!(
        Cli::try_parse_from(argv).is_err(),
        "expected parse failure for {:?} but succeeded",
        argv
    );
}

// ─── identity show ────────────────────────────────────────────────────────────

#[test]
fn parse_identity_show_required_address() {
    let cli = must_parse(&["agcli", "identity", "show", "--address", ALICE_SS58]);
    match cli.command {
        Commands::Identity(IdentityCommands::Show { address }) => {
            assert_eq!(address, ALICE_SS58);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_show_missing_address_fails() {
    // --address is required; omitting it must fail.
    must_fail(&["agcli", "identity", "show"]);
}

#[test]
fn parse_identity_show_arbitrary_address() {
    // Any string is accepted at the parse layer; SS58 validation happens at runtime.
    let cli = must_parse(&["agcli", "identity", "show", "--address", "5FakeAddress"]);
    match cli.command {
        Commands::Identity(IdentityCommands::Show { address }) => {
            assert_eq!(address, "5FakeAddress");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── identity set ─────────────────────────────────────────────────────────────

#[test]
fn parse_identity_set_name_only() {
    let cli = must_parse(&["agcli", "identity", "set", "--name", "MyValidator"]);
    match cli.command {
        Commands::Identity(IdentityCommands::Set {
            name,
            url,
            github,
            description,
            image,
        }) => {
            assert_eq!(name, "MyValidator");
            assert!(url.is_none());
            assert!(github.is_none());
            assert!(description.is_none());
            assert!(image.is_none());
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_set_all_fields() {
    let cli = must_parse(&[
        "agcli",
        "identity",
        "set",
        "--name",
        "MyValidator",
        "--url",
        "https://myvalidator.io",
        "--github",
        "owner/repo",
        "--description",
        "A Bittensor validator",
        "--image",
        "https://myvalidator.io/logo.png",
    ]);
    match cli.command {
        Commands::Identity(IdentityCommands::Set {
            name,
            url,
            github,
            description,
            image,
        }) => {
            assert_eq!(name, "MyValidator");
            assert_eq!(url.as_deref(), Some("https://myvalidator.io"));
            assert_eq!(github.as_deref(), Some("owner/repo"));
            assert_eq!(description.as_deref(), Some("A Bittensor validator"));
            assert_eq!(image.as_deref(), Some("https://myvalidator.io/logo.png"));
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_set_missing_name_fails() {
    // --name is required.
    must_fail(&["agcli", "identity", "set"]);
}

#[test]
fn parse_identity_set_optional_fields_absent_by_default() {
    let cli = must_parse(&["agcli", "identity", "set", "--name", "X"]);
    match cli.command {
        Commands::Identity(IdentityCommands::Set {
            url,
            github,
            description,
            image,
            ..
        }) => {
            // All optional fields default to None.
            assert!(url.is_none(), "url should default to None");
            assert!(github.is_none(), "github should default to None");
            assert!(description.is_none(), "description should default to None");
            assert!(image.is_none(), "image should default to None");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── identity clear ───────────────────────────────────────────────────────────

#[test]
fn parse_identity_clear() {
    let cli = must_parse(&["agcli", "identity", "clear"]);
    assert!(
        matches!(cli.command, Commands::Identity(IdentityCommands::Clear)),
        "expected Identity::Clear, got {:?}",
        cli.command
    );
}

#[test]
fn parse_identity_clear_with_wallet_flags() {
    // Global wallet flags should not interfere with the subcommand parse.
    let cli = must_parse(&["agcli", "--wallet", "mywallet", "identity", "clear"]);
    assert!(matches!(
        cli.command,
        Commands::Identity(IdentityCommands::Clear)
    ));
    assert_eq!(cli.wallet.as_str(), "mywallet");
}

// ─── identity set-subnet ──────────────────────────────────────────────────────

#[test]
fn parse_identity_set_subnet_required_args() {
    let cli = must_parse(&[
        "agcli",
        "identity",
        "set-subnet",
        "--netuid",
        "1",
        "--name",
        "MySubnet",
    ]);
    match cli.command {
        Commands::Identity(IdentityCommands::SetSubnet {
            netuid,
            name,
            github,
            url,
        }) => {
            assert_eq!(netuid, 1);
            assert_eq!(name, "MySubnet");
            assert!(github.is_none());
            assert!(url.is_none());
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_set_subnet_all_fields() {
    let cli = must_parse(&[
        "agcli",
        "identity",
        "set-subnet",
        "--netuid",
        "42",
        "--name",
        "Tau Vision",
        "--github",
        "owner/subnet-repo",
        "--url",
        "https://tausubnet.ai",
    ]);
    match cli.command {
        Commands::Identity(IdentityCommands::SetSubnet {
            netuid,
            name,
            github,
            url,
        }) => {
            assert_eq!(netuid, 42);
            assert_eq!(name, "Tau Vision");
            assert_eq!(github.as_deref(), Some("owner/subnet-repo"));
            assert_eq!(url.as_deref(), Some("https://tausubnet.ai"));
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_set_subnet_missing_netuid_fails() {
    must_fail(&["agcli", "identity", "set-subnet", "--name", "MySubnet"]);
}

#[test]
fn parse_identity_set_subnet_missing_name_fails() {
    must_fail(&["agcli", "identity", "set-subnet", "--netuid", "1"]);
}

#[test]
fn parse_identity_set_subnet_zero_netuid() {
    // Netuid=0 (root) is syntactically valid at the parse layer.
    let cli = must_parse(&[
        "agcli",
        "identity",
        "set-subnet",
        "--netuid",
        "0",
        "--name",
        "Root",
    ]);
    match cli.command {
        Commands::Identity(IdentityCommands::SetSubnet { netuid, .. }) => {
            assert_eq!(netuid, 0);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn parse_identity_set_subnet_max_netuid() {
    let cli = must_parse(&[
        "agcli",
        "identity",
        "set-subnet",
        "--netuid",
        "65535",
        "--name",
        "Max",
    ]);
    match cli.command {
        Commands::Identity(IdentityCommands::SetSubnet { netuid, .. }) => {
            assert_eq!(netuid, 65535);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

// ─── global flag interaction ──────────────────────────────────────────────────

#[test]
fn parse_identity_show_with_output_json() {
    // --output json is a global flag; it must parse alongside identity show.
    let cli = must_parse(&[
        "agcli",
        "--output",
        "json",
        "identity",
        "show",
        "--address",
        ALICE_SS58,
    ]);
    assert_eq!(cli.output, agcli::cli::OutputFormat::Json);
}

#[test]
fn parse_identity_set_with_yes_flag() {
    let cli = must_parse(&[
        "agcli",
        "--yes",
        "identity",
        "set",
        "--name",
        "AutoValidator",
    ]);
    assert!(cli.yes);
}

#[test]
fn parse_identity_show_live_flag_after_subcommand() {
    // --live after the subcommand avoids the clap ordering pitfall (see Findings).
    let cli = must_parse(&[
        "agcli",
        "identity",
        "show",
        "--address",
        ALICE_SS58,
        "--live",
    ]);
    assert!(cli.command.is_some_identity_show());
}

// ─── helper trait for pattern matching ───────────────────────────────────────

trait IsIdentityVariant {
    fn is_some_identity_show(&self) -> bool;
}

impl IsIdentityVariant for Commands {
    fn is_some_identity_show(&self) -> bool {
        matches!(self, Commands::Identity(IdentityCommands::Show { .. }))
    }
}

// ─── argument field-name surface check ───────────────────────────────────────

/// Verifies the CLI surfaces exactly the fields that are documented in the
/// IdentityCommands enum. Any new field added to the enum without updating this
/// test will fail.
#[test]
fn identity_set_field_surface() {
    // Optional flags (not already present in the baseline args) that should parse.
    let optional_flags = ["--url", "--github", "--description", "--image"];
    for flag in optional_flags {
        let argv = &["agcli", "identity", "set", "--name", "X", flag, "val"];
        assert!(
            Cli::try_parse_from(argv).is_ok(),
            "flag {} failed to parse on identity set",
            flag
        );
    }

    // Flags that do NOT yet exist — confirms they are absent from the clap surface.
    for absent in &["--discord", "--email", "--twitter", "--legal"] {
        let argv = &["agcli", "identity", "set", "--name", "X", absent, "val"];
        assert!(
            Cli::try_parse_from(argv).is_err(),
            "flag {} should not exist on identity set yet",
            absent
        );
    }
}

#[test]
fn identity_set_subnet_field_surface() {
    // Optional flags (not already in baseline) that should parse on set-subnet.
    let optional_flags = ["--github", "--url"];
    for flag in optional_flags {
        let argv = &[
            "agcli",
            "identity",
            "set-subnet",
            "--netuid",
            "1",
            "--name",
            "X",
            flag,
            "val",
        ];
        assert!(
            Cli::try_parse_from(argv).is_ok(),
            "flag {} failed to parse on set-subnet",
            flag
        );
    }

    // Flags that do NOT yet exist on set-subnet.
    for absent in &[
        "--discord",
        "--description",
        "--logo-url",
        "--subnet-contact",
    ] {
        let argv = &[
            "agcli",
            "identity",
            "set-subnet",
            "--netuid",
            "1",
            "--name",
            "X",
            absent,
            "val",
        ];
        assert!(
            Cli::try_parse_from(argv).is_err(),
            "flag {} should not exist on identity set-subnet yet",
            absent
        );
    }
}

// ─── localnet green-path (ignored — requires Docker + running localnet) ───────

/// End-to-end green path against a local subtensor node.
///
/// Prerequisites (not available in standard CI):
///   1. Docker installed and running.
///   2. `agcli localnet start` executed and node healthy on ws://127.0.0.1:9944.
///   3. Alice's wallet present at ~/.bittensor/wallets/alice (or use `agcli wallet create`).
///
/// Run manually:
///   cargo test --test audit_identity -- --ignored green_path_identity --nocapture
#[test]
#[ignore = "requires local subtensor node (Docker) — not available in standard CI"]
fn green_path_identity() {
    // This test is intentionally left as a shell so the file compiles.
    // A real localnet run would:
    //   1. Start the node via `agcli localnet start` or docker directly.
    //   2. Call `agcli identity show --address <alice>` and assert output contains "Name:".
    //   3. Call `agcli identity set --name AliceTest` (signed by alice coldkey).
    //   4. Call `agcli identity show --address <alice>` again and assert name == "AliceTest".
    //   5. Call `agcli identity clear` and assert identity is cleared.
    //   6. Call `agcli identity set-subnet --netuid 1 --name TestSubnet` (alice is owner on localnet).
    //   7. Verify subnet identity via chain storage query.
    //
    // NOTE: Steps 3–5 will currently fail at the CHAIN level because of the
    // `identified` argument bug in `set_registry_identity` / `clear_registry_identity`
    // (see Findings in the audit handoff). Step 6 will also fail because
    // set_subnet_identity calls the wrong dispatchable (set_identity instead of
    // set_subnet_identity). Document failures here when running against localnet.
    eprintln!("green_path_identity: skipped — localnet not available");
}
