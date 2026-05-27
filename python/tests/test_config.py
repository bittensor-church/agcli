from agcli import Config


def test_config_roundtrip(tmp_path) -> None:
    path = tmp_path / "config.toml"
    config = Config(
        network="finney",
        endpoint="wss://example.org:443",
        wallet_dir="/tmp/wallets",
        wallet="main",
        hotkey="default",
        output="json",
        proxy="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        live_interval=15,
        batch=True,
        spending_limits={"1": 1.5, "18": 10.0},
        finalization_timeout=45,
        mortality_blocks=64,
    )

    config.save_to(str(path))
    loaded = Config.load_from(str(path))

    assert loaded.network == "finney"
    assert loaded.endpoint == "wss://example.org:443"
    assert loaded.wallet_dir == "/tmp/wallets"
    assert loaded.wallet == "main"
    assert loaded.hotkey == "default"
    assert loaded.output == "json"
    assert loaded.proxy
    assert loaded.live_interval == 15
    assert loaded.batch is True
    assert loaded.spending_limits == {"1": 1.5, "18": 10.0}
    assert loaded.finalization_timeout == 45
    assert loaded.mortality_blocks == 64
