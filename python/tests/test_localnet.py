import pytest

from agcli import localnet


def test_dev_accounts_returns_alice_bob() -> None:
    accounts = localnet.dev_accounts()
    names = {a.name for a in accounts}
    assert "Alice" in names
    assert "Bob" in names
    alice = next(a for a in accounts if a.name == "Alice")
    assert alice.uri == "//Alice"
    assert alice.ss58.startswith("5")
    assert "TAO" in alice.balance


def test_localnet_config_defaults_and_overrides() -> None:
    default = localnet.LocalnetConfig()
    assert default.container_name == localnet.DEFAULT_CONTAINER
    assert default.image == localnet.DEFAULT_IMAGE
    assert default.port == 9944
    assert default.wait is True

    custom = localnet.LocalnetConfig(
        container_name="my-localnet",
        port=19944,
        wait=False,
        wait_timeout=60,
    )
    assert custom.container_name == "my-localnet"
    assert custom.port == 19944
    assert custom.wait is False
    assert custom.wait_timeout == 60
    payload = custom.to_dict()
    assert payload["container_name"] == "my-localnet"
    assert payload["port"] == 19944


def test_dev_account_to_dict_shape() -> None:
    alice = next(a for a in localnet.dev_accounts() if a.name == "Alice")
    payload = alice.to_dict()
    assert payload["name"] == "Alice"
    assert payload["uri"] == "//Alice"
    assert payload["ss58"].startswith("5")


@pytest.mark.asyncio
async def test_status_against_missing_container_returns_inactive() -> None:
    status = await localnet.status("agcli_pytest_nonexistent_xyz", port=29944)
    assert status.running is False
    assert status.endpoint is None
    assert status.container_id is None


@pytest.mark.localnet
@pytest.mark.asyncio
async def test_status_against_real_container() -> None:
    import os

    container = os.environ.get("AGCLI_LOCALNET_CONTAINER")
    port = int(os.environ.get("AGCLI_LOCALNET_PORT", "9944"))
    if not container:
        pytest.skip("AGCLI_LOCALNET_CONTAINER not set")
    status = await localnet.status(container, port=port)
    assert status.container_name == container
