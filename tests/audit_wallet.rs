//! Audit tests for the `wallet` command group.
//!
//! Coverage:
//!   (a) Parse-surface tests: every `WalletCommands` variant via `Cli::try_parse_from`.
//!   (b) Handler-level unit tests: green-path invocations of `handle_wallet` without a chain.
//!   (c) One `#[ignore]` integration test gated on a local chain.
//!
//! Run: `cargo test --test audit_wallet`
//! Run ignored: `cargo test --test audit_wallet -- --ignored`

use agcli::cli::{Cli, OutputFormat, WalletCommands};
use clap::Parser as _;

// ─────────────────────────────────────────────────────────────────────────────
// (a) Parse-surface tests
// ─────────────────────────────────────────────────────────────────────────────

/// Every subcommand must parse without error given minimal realistic flags.
/// These tests catch regressions where a required arg is added to the clap
/// definition without a corresponding default or `Option` wrapper.

#[test]
fn parse_wallet_create_minimal() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "create"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Create {
            name,
            hotkey_name,
            password,
            no_mnemonic,
        })         => {
            assert_eq!(name, "default");
            // Audit finding: WalletCommands::Create::hotkey_name conflicts with the global
            // Cli::hotkey_name (flag --hotkey-name, default "default"). Clap resolves this by
            // populating the subcommand field with the global default, so the value is always
            // Some("default") rather than None when not explicitly set. The handler uses
            // `.as_deref().unwrap_or("default")` which works correctly either way, but the
            // None vs Some distinction is lost.
            assert_eq!(hotkey_name.as_deref().unwrap_or("default"), "default");
            assert!(password.is_none());
            assert!(!no_mnemonic);
        }
        other => panic!("unexpected parse: {:?}", other),
    }
}

#[test]
fn parse_wallet_create_full() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "create",
        "--name",
        "mywallet",
        "--hotkey-name",
        "miner",
        "--password",
        "secret",
        "--no-mnemonic",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Create {
            name,
            hotkey_name,
            password,
            no_mnemonic,
        }) => {
            assert_eq!(name, "mywallet");
            assert_eq!(hotkey_name.as_deref(), Some("miner"));
            assert_eq!(password.as_deref(), Some("secret"));
            assert!(no_mnemonic);
        }
        other => panic!("unexpected parse: {:?}", other),
    }
}

#[test]
fn parse_wallet_list() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "list"]).unwrap();
    assert!(matches!(
        cli.command,
        agcli::cli::Commands::Wallet(WalletCommands::List)
    ));
}

#[test]
fn parse_wallet_show_default() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "show"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Show { all }) => assert!(!all),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_show_all() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "show", "--all"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Show { all }) => assert!(all),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_import_with_mnemonic() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "import",
        "--name",
        "imported",
        "--mnemonic",
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        "--password",
        "pw",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Import { name, mnemonic, password }) => {
            assert_eq!(name, "imported");
            assert!(mnemonic.is_some());
            assert_eq!(password.as_deref(), Some("pw"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_import_minimal() {
    // Mnemonic may be omitted (will be prompted interactively)
    let cli = Cli::try_parse_from(["agcli", "wallet", "import"]).unwrap();
    assert!(matches!(
        cli.command,
        agcli::cli::Commands::Wallet(WalletCommands::Import { .. })
    ));
}

#[test]
fn parse_wallet_regen_coldkey_minimal() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "regen-coldkey"]).unwrap();
    assert!(matches!(
        cli.command,
        agcli::cli::Commands::Wallet(WalletCommands::RegenColdkey { .. })
    ));
}

#[test]
fn parse_wallet_regen_coldkey_with_mnemonic() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "regen-coldkey",
        "--mnemonic",
        "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
        "--password",
        "pw",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::RegenColdkey { mnemonic, password }) => {
            assert!(mnemonic.is_some());
            assert_eq!(password.as_deref(), Some("pw"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_regen_hotkey() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "regen-hotkey", "--name", "miner1"])
        .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::RegenHotkey { name, .. }) => {
            assert_eq!(name, "miner1");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_regen_hotkey_default_name() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "regen-hotkey"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::RegenHotkey { name, .. }) => {
            assert_eq!(name, "default");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_new_hotkey() {
    let cli =
        Cli::try_parse_from(["agcli", "wallet", "new-hotkey", "--name", "validator"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::NewHotkey { name }) => {
            assert_eq!(name, "validator");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

/// `--name` is required (no default) for new-hotkey — missing it must fail to parse.
#[test]
fn parse_wallet_new_hotkey_requires_name() {
    let result = Cli::try_parse_from(["agcli", "wallet", "new-hotkey"]);
    assert!(
        result.is_err(),
        "new-hotkey without --name should fail to parse"
    );
}

#[test]
fn parse_wallet_sign() {
    let cli =
        Cli::try_parse_from(["agcli", "wallet", "sign", "--message", "hello bittensor"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Sign { message }) => {
            assert_eq!(message, "hello bittensor");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_sign_hex_message() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "sign", "--message", "0xdeadbeef"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Sign { message }) => {
            assert_eq!(message, "0xdeadbeef");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_verify() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "verify",
        "--message",
        "hello",
        "--signature",
        "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Verify {
            message,
            signature,
            signer,
        }) => {
            assert_eq!(message, "hello");
            assert!(signature.starts_with("0x"));
            assert!(signer.is_none());
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_verify_with_signer() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "verify",
        "--message",
        "test",
        "--signature",
        "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab",
        "--signer",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Verify { signer, .. }) => {
            assert_eq!(
                signer.as_deref(),
                Some("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_derive_pubkey() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "derive",
        "--input",
        "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::Derive { input }) => {
            assert!(input.starts_with("0x"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_derive_mnemonic() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "derive",
        "--input",
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    ])
    .unwrap();
    assert!(matches!(
        cli.command,
        agcli::cli::Commands::Wallet(WalletCommands::Derive { .. })
    ));
}

#[test]
fn parse_wallet_dev_key_default() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "dev-key"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::DevKey { uri, .. }) => {
            assert_eq!(uri, "Alice");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_dev_alias() {
    // `dev` is an alias for `dev-key`
    let cli = Cli::try_parse_from(["agcli", "wallet", "dev", "--uri", "Bob"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::DevKey { uri, .. }) => {
            assert_eq!(uri, "Bob");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_associate_hotkey_no_address() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "associate-hotkey"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::AssociateHotkey { hotkey }) => {
            assert!(hotkey.is_none());
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_associate_hotkey_with_address() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "associate-hotkey",
        "--hotkey-address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::AssociateHotkey { hotkey }) => {
            assert_eq!(
                hotkey.as_deref(),
                Some("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_check_swap_no_address() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "check-swap"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::CheckSwap { address }) => {
            assert!(address.is_none());
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_check_swap_with_address() {
    let cli = Cli::try_parse_from([
        "agcli",
        "wallet",
        "check-swap",
        "--address",
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    ])
    .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::CheckSwap { address }) => {
            assert_eq!(
                address.as_deref(),
                Some("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_show_mnemonic_minimal() {
    let cli = Cli::try_parse_from(["agcli", "wallet", "show-mnemonic"]).unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::ShowMnemonic { password }) => {
            assert!(password.is_none());
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn parse_wallet_show_mnemonic_with_password() {
    let cli =
        Cli::try_parse_from(["agcli", "wallet", "show-mnemonic", "--password", "hunter2"])
            .unwrap();
    match cli.command {
        agcli::cli::Commands::Wallet(WalletCommands::ShowMnemonic { password }) => {
            assert_eq!(password.as_deref(), Some("hunter2"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// (b) Handler-level green-path tests (no chain required)
// ─────────────────────────────────────────────────────────────────────────────

use agcli::cli::wallet_cmds::handle_wallet;
use sp_core::Pair as _;

#[tokio::test]
async fn green_path_wallet_create() {
    let dir = tempfile::tempdir().unwrap();
    let result = handle_wallet(
        WalletCommands::Create {
            name: "audit_test".to_string(),
            hotkey_name: Some("default".to_string()),
            password: Some("Audit1234!".to_string()),
            no_mnemonic: true,
        },
        dir.path().to_str().unwrap(),
        "audit_test",
        Some("Audit1234!"),
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "wallet create failed: {:?}", result.err());
    assert!(dir.path().join("audit_test").join("coldkey").exists());
    assert!(dir.path().join("audit_test").join("coldkeypub.txt").exists());
    assert!(
        dir.path()
            .join("audit_test")
            .join("hotkeys")
            .join("default")
            .exists()
    );
}

#[tokio::test]
async fn green_path_wallet_list() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "w1", "pw", "default").unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "w2", "pw", "default").unwrap();
    let result = handle_wallet(
        WalletCommands::List,
        dir.path().to_str().unwrap(),
        "default",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "wallet list failed: {:?}", result.err());
}

#[tokio::test]
async fn green_path_wallet_show() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "showme", "pw", "default").unwrap();
    let result = handle_wallet(
        WalletCommands::Show { all: true },
        dir.path().to_str().unwrap(),
        "default",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "wallet show failed: {:?}", result.err());
}

#[tokio::test]
async fn green_path_wallet_import() {
    let dir = tempfile::tempdir().unwrap();
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .to_string();
    // require_mnemonic() refuses the CLI flag unless AGCLI_MNEMONIC env var is set — security guard.
    std::env::set_var("AGCLI_MNEMONIC", &mnemonic);
    let result = handle_wallet(
        WalletCommands::Import {
            name: "imported".to_string(),
            mnemonic: Some(mnemonic.clone()),
            password: Some("testpw123".to_string()),
        },
        dir.path().to_str().unwrap(),
        "imported",
        Some("testpw123"),
        OutputFormat::Json,
    )
    .await;
    std::env::remove_var("AGCLI_MNEMONIC");
    assert!(result.is_ok(), "wallet import failed: {:?}", result.err());
    assert!(dir.path().join("imported").join("coldkey").exists());
}

#[tokio::test]
async fn green_path_wallet_regen_coldkey() {
    let dir = tempfile::tempdir().unwrap();
    let mnemonic = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong".to_string();
    // require_mnemonic() refuses CLI flag unless AGCLI_MNEMONIC env var is set.
    std::env::set_var("AGCLI_MNEMONIC", &mnemonic);
    let result = handle_wallet(
        WalletCommands::RegenColdkey {
            mnemonic: Some(mnemonic),
            password: Some("regen_pw!".to_string()),
        },
        dir.path().to_str().unwrap(),
        "regen_wallet",
        Some("regen_pw!"),
        OutputFormat::Json,
    )
    .await;
    std::env::remove_var("AGCLI_MNEMONIC");
    assert!(
        result.is_ok(),
        "wallet regen-coldkey failed: {:?}",
        result.err()
    );
    assert!(dir.path().join("regen_wallet").join("coldkey").exists());
}

#[tokio::test]
async fn green_path_wallet_regen_hotkey() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "regen_hk", "pw", "default").unwrap();
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .to_string();
    // require_mnemonic() refuses CLI flag unless AGCLI_MNEMONIC env var is set.
    std::env::set_var("AGCLI_MNEMONIC", &mnemonic);
    let result = handle_wallet(
        WalletCommands::RegenHotkey {
            name: "newhotkey".to_string(),
            mnemonic: Some(mnemonic),
        },
        dir.path().to_str().unwrap(),
        "regen_hk",
        None,
        OutputFormat::Json,
    )
    .await;
    std::env::remove_var("AGCLI_MNEMONIC");
    assert!(
        result.is_ok(),
        "wallet regen-hotkey failed: {:?}",
        result.err()
    );
    assert!(
        dir.path()
            .join("regen_hk")
            .join("hotkeys")
            .join("newhotkey")
            .exists()
    );
}

#[tokio::test]
async fn green_path_wallet_new_hotkey() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "newhk_test", "pw", "default").unwrap();
    let result = handle_wallet(
        WalletCommands::NewHotkey {
            name: "miner99".to_string(),
        },
        dir.path().to_str().unwrap(),
        "newhk_test",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(
        result.is_ok(),
        "wallet new-hotkey failed: {:?}",
        result.err()
    );
    assert!(
        dir.path()
            .join("newhk_test")
            .join("hotkeys")
            .join("miner99")
            .exists()
    );
}

#[tokio::test]
async fn green_path_wallet_derive_pubkey() {
    let dir = tempfile::tempdir().unwrap();
    // Alice's known public key in 0x hex (32 bytes)
    let result = handle_wallet(
        WalletCommands::Derive {
            input: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                .to_string(),
        },
        dir.path().to_str().unwrap(),
        "default",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "wallet derive pubkey failed: {:?}", result.err());
}

#[tokio::test]
async fn green_path_wallet_derive_mnemonic() {
    let dir = tempfile::tempdir().unwrap();
    let result = handle_wallet(
        WalletCommands::Derive {
            input:
                "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
                    .to_string(),
        },
        dir.path().to_str().unwrap(),
        "default",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(
        result.is_ok(),
        "wallet derive mnemonic failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn green_path_wallet_dev_key() {
    let dir = tempfile::tempdir().unwrap();
    let result = handle_wallet(
        WalletCommands::DevKey {
            uri: "Alice".to_string(),
            password: Some("devpass".to_string()),
        },
        dir.path().to_str().unwrap(),
        "alice",
        Some("devpass"),
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "wallet dev-key failed: {:?}", result.err());
    // coldkeypub.txt stores the raw hex public key (no SS58, no 0x prefix) —
    // this is an audit finding: docs previously said "SS58 address".
    assert!(dir.path().join("alice").join("coldkeypub.txt").exists());
    let pub_content =
        std::fs::read_to_string(dir.path().join("alice").join("coldkeypub.txt")).unwrap();
    // Alice's known hex public key (32 bytes, no 0x prefix)
    assert!(
        pub_content.trim().contains("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"),
        "Alice's hex pubkey not found in coldkeypub.txt: {}",
        pub_content
    );
}

#[tokio::test]
async fn green_path_wallet_sign_and_verify() {
    let dir = tempfile::tempdir().unwrap();
    let (wallet, _, _) =
        agcli::Wallet::create(dir.path().to_str().unwrap(), "sv_audit", "signpw", "default")
            .unwrap();
    let coldkey_ss58 = wallet.coldkey_ss58().unwrap().to_string();

    // Sign a message
    let sign_result = handle_wallet(
        WalletCommands::Sign {
            message: "hello audit".to_string(),
        },
        dir.path().to_str().unwrap(),
        "sv_audit",
        Some("signpw"),
        OutputFormat::Json,
    )
    .await;
    assert!(sign_result.is_ok(), "wallet sign failed: {:?}", sign_result.err());

    // Verify always uses the signer's public key — just test parse path here
    let verify_result = handle_wallet(
        WalletCommands::Verify {
            message: "hello audit".to_string(),
            // Zero signature (64 bytes = 128 hex chars after 0x prefix) — will fail verification
            signature:
                "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                    .to_string(),
            signer: Some(coldkey_ss58),
        },
        dir.path().to_str().unwrap(),
        "sv_audit",
        Some("signpw"),
        OutputFormat::Json,
    )
    .await;
    // Zero signature will fail verification — that's expected (exit code 1 / anyhow error)
    assert!(
        verify_result.is_err(),
        "zero signature should fail verification"
    );
    let msg = format!("{:#}", verify_result.unwrap_err());
    assert!(
        msg.contains("verification failed") || msg.contains("invalid"),
        "unexpected error: {}",
        msg
    );
}

#[tokio::test]
async fn green_path_wallet_show_mnemonic() {
    let dir = tempfile::tempdir().unwrap();
    let known_mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    agcli::Wallet::import_from_mnemonic(
        dir.path().to_str().unwrap(),
        "mnemo_test",
        known_mnemonic,
        "retrievepw",
    )
    .unwrap();

    let result = handle_wallet(
        WalletCommands::ShowMnemonic {
            password: Some("retrievepw".to_string()),
        },
        dir.path().to_str().unwrap(),
        "mnemo_test",
        Some("retrievepw"),
        OutputFormat::Json,
    )
    .await;
    assert!(
        result.is_ok(),
        "wallet show-mnemonic failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn wallet_new_hotkey_duplicate_name_fails() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "dup_hk", "pw", "default").unwrap();

    // Create hotkey once — should succeed
    handle_wallet(
        WalletCommands::NewHotkey {
            name: "miner1".to_string(),
        },
        dir.path().to_str().unwrap(),
        "dup_hk",
        None,
        OutputFormat::Json,
    )
    .await
    .unwrap();

    // Create same hotkey again — should fail with a useful error
    let result = handle_wallet(
        WalletCommands::NewHotkey {
            name: "miner1".to_string(),
        },
        dir.path().to_str().unwrap(),
        "dup_hk",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_err(), "duplicate new-hotkey should fail");
    let msg = format!("{:#}", result.unwrap_err());
    assert!(
        msg.contains("already exists"),
        "expected 'already exists', got: {}",
        msg
    );
}

#[tokio::test]
async fn wallet_derive_wrong_pubkey_length_fails() {
    let dir = tempfile::tempdir().unwrap();
    let result = handle_wallet(
        WalletCommands::Derive {
            input: "0xdeadbeef".to_string(), // only 4 bytes — should fail
        },
        dir.path().to_str().unwrap(),
        "default",
        None,
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_err(), "short pubkey should fail");
    let msg = format!("{:#}", result.unwrap_err());
    assert!(
        msg.contains("32 bytes") || msg.contains("invalid") || msg.contains("bytes"),
        "expected length error, got: {}",
        msg
    );
}

#[tokio::test]
async fn wallet_sign_hex_message_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    agcli::Wallet::create(dir.path().to_str().unwrap(), "hexsign", "pw", "default").unwrap();
    let result = handle_wallet(
        WalletCommands::Sign {
            message: "0xcafebabe".to_string(),
        },
        dir.path().to_str().unwrap(),
        "hexsign",
        Some("pw"),
        OutputFormat::Json,
    )
    .await;
    assert!(result.is_ok(), "hex message sign failed: {:?}", result.err());
}

// ─────────────────────────────────────────────────────────────────────────────
// (c) Ignored integration test — requires a running localnet
// ─────────────────────────────────────────────────────────────────────────────

/// Green-path integration test: create a wallet then call associate-hotkey against a local chain.
///
/// Prerequisites:
///   - Docker with `ghcr.io/opentensor/subtensor-localnet:devnet-ready` available
///   - Local chain running on ws://127.0.0.1:9944 (start via `agcli localnet start`)
///
/// Run: `cargo test --test audit_wallet -- --ignored green_path_associate_hotkey_localnet`
#[tokio::test]
#[ignore]
async fn green_path_associate_hotkey_localnet() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path().to_str().unwrap();

    // Create Alice wallet (well-funded dev account on localnet)
    let alice_wallet =
        agcli::Wallet::create_from_uri(base, "//Alice", "alicepw").unwrap();
    let coldkey_ss58 = alice_wallet.coldkey_ss58().unwrap().to_string();

    // Create a fresh hotkey
    let (hk_pair, _) = agcli::wallet::keypair::generate_mnemonic_keypair().unwrap();
    let hk_ss58 = agcli::wallet::keypair::to_ss58(&hk_pair.public(), 42);

    // Connect to local chain
    let client = agcli::Client::connect("ws://127.0.0.1:9944").await.expect(
        "localnet not reachable — ensure `agcli localnet start` is running on port 9944",
    );

    // Unlock Alice's coldkey
    let mut wallet = agcli::Wallet::open(format!("{}/alice", base)).unwrap();
    wallet.unlock_coldkey("alicepw").unwrap();
    let pair = wallet.coldkey().unwrap().clone();

    // Submit associate_hotkey
    let result = client.try_associate_hotkey(&pair, &hk_ss58).await;
    assert!(
        result.is_ok(),
        "try_associate_hotkey failed: {:?}",
        result.err()
    );
    let tx_hash = result.unwrap();
    assert!(tx_hash.starts_with("0x"), "expected tx hash, got: {}", tx_hash);

    println!(
        "associate-hotkey green path: coldkey={} hotkey={} tx={}",
        coldkey_ss58, hk_ss58, tx_hash
    );
}
