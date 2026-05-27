import pytest

from agcli import AgcliError, AsyncClient, Balance, NetUid, Network, Wallet


def test_import_public_api() -> None:
    assert AsyncClient.__name__ == "AsyncClient"
    assert Wallet.__name__ == "Wallet"
    assert Balance.from_tao(1.0).tao == pytest.approx(1.0)


def test_balance_roundtrip() -> None:
    balance = Balance.from_rao(1_000_000_000)
    assert balance.rao == 1_000_000_000
    assert balance.tao == pytest.approx(1.0)


def test_netuid_value() -> None:
    uid = NetUid(97)
    assert uid.value == 97
    assert int(uid) == 97


def test_network_parse() -> None:
    network = Network.parse("finney")
    assert "finney" in network.ws_url or "opentensor" in network.ws_url


def test_wallet_create_roundtrip(tmp_path) -> None:
    wallet_dir = tmp_path / "wallets"
    result = Wallet.create(
        str(wallet_dir),
        "pytest_wallet",
        "test-password-123",
    )
    assert result.wallet.name == "pytest_wallet"
    assert result.coldkey_mnemonic
    assert result.hotkey_mnemonic

    reopened = Wallet.open(str(wallet_dir / "pytest_wallet"))
    assert reopened.coldkey_ss58 == result.wallet.coldkey_ss58
    reopened.unlock_coldkey("test-password-123")
    assert reopened.coldkey_ss58


def test_invalid_wallet_name_raises(tmp_path) -> None:
    with pytest.raises(AgcliError) as exc:
        Wallet.create(str(tmp_path), "../bad", "test-password-123")
    assert "Invalid wallet name" in str(exc.value)
    assert exc.value.code != 0


def test_wallet_sign_and_verify(tmp_path) -> None:
    wallet = Wallet.create_from_uri(str(tmp_path), "//Alice", "password123")
    message = b"agcli python bindings"
    signature = wallet.sign_message("coldkey", message)
    assert wallet.coldkey_ss58
    assert Wallet.verify_message(wallet.coldkey_ss58, message, signature)
    assert wallet.coldkey_public_ss58 == wallet.coldkey_ss58


def test_pickle_is_blocked_for_wallet_and_async_client(tmp_path) -> None:
    import pickle

    wallet = Wallet.create_from_uri(str(tmp_path), "//Alice", "password123")
    with pytest.raises(TypeError, match="cannot be pickled"):
        pickle.dumps(wallet)

    client = AsyncClient.__new__(AsyncClient)
    with pytest.raises(TypeError, match="cannot be pickled"):
        pickle.dumps(client)


@pytest.mark.network
@pytest.mark.asyncio
async def test_connect_and_list_subnets() -> None:
    client = await AsyncClient.connect_network(Network.finney())
    subnets = await client.get_all_subnets()
    assert isinstance(subnets, list)
    assert len(subnets) > 0
