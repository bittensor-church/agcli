import hashlib
import os

import pytest

from agcli import AsyncClient, ValidationError, Wallet

pytestmark = [pytest.mark.localnet, pytest.mark.asyncio]

LOCALNET_PASSWORD = "localnet-pass"


def _salt_to_u16_words(value: str) -> list[int]:
    encoded = value.encode()
    words: list[int] = []
    for idx in range(0, len(encoded), 2):
        first = encoded[idx]
        second = encoded[idx + 1] if idx + 1 < len(encoded) else 0
        words.append((second << 8) | first)
    return words


def _weight_commit_hash(uids: list[int], values: list[int], salt: str) -> bytes:
    hasher = hashlib.blake2b(digest_size=32)
    for uid in uids:
        hasher.update(uid.to_bytes(2, byteorder="little", signed=False))
    for value in values:
        hasher.update(value.to_bytes(2, byteorder="little", signed=False))
    hasher.update(salt.encode())
    return hasher.digest()


@pytest.fixture(scope="module")
def localnet_ws() -> str:
    endpoint = os.getenv("AGCLI_LOCALNET_WS")
    if not endpoint:
        pytest.skip("AGCLI_LOCALNET_WS is not set")
    return endpoint


@pytest.fixture(scope="module")
async def localnet_client(localnet_ws: str) -> AsyncClient:
    try:
        client = await AsyncClient.connect(localnet_ws)
        if not await client.is_alive():
            pytest.skip(f"Localnet endpoint is unreachable: {localnet_ws}")
        return client
    except Exception as exc:  # pragma: no cover - defensive skip path
        pytest.skip(f"Failed to connect to localnet endpoint {localnet_ws}: {exc}")


@pytest.fixture(scope="module")
def localnet_wallets(tmp_path_factory: pytest.TempPathFactory) -> tuple[Wallet, Wallet]:
    wallet_dir = tmp_path_factory.mktemp("localnet-wallets")
    alice = Wallet.create_from_uri(str(wallet_dir), "//Alice", LOCALNET_PASSWORD)
    bob = Wallet.create_from_uri(str(wallet_dir), "//Bob", LOCALNET_PASSWORD)
    return alice, bob


async def test_transfer_alice_to_bob(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, bob = localnet_wallets
    assert bob.coldkey_ss58 is not None
    before = await localnet_client.get_balance(bob.coldkey_ss58)
    with alice.session(password=LOCALNET_PASSWORD):
        tx_hash = await localnet_client.transfer(alice, bob.coldkey_ss58, 1_000_000_000)
    after = await localnet_client.get_balance(bob.coldkey_ss58)
    assert tx_hash
    assert after.rao >= before.rao + 1_000_000_000


async def test_add_and_remove_stake(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, _ = localnet_wallets
    with alice.session(password=LOCALNET_PASSWORD):
        add_hash = await localnet_client.add_stake(alice, 0, 100_000_000)
        remove_hash = await localnet_client.remove_stake(alice, 0, 50_000_000)
    assert add_hash
    assert remove_hash


async def test_weights_set_commit_reveal(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, _ = localnet_wallets
    uids = [0]
    values = [1]
    salt_str = "phase2-localnet"
    salt_words = _salt_to_u16_words(salt_str)
    commit_hash = _weight_commit_hash(uids, values, salt_str)

    with alice.session(password=LOCALNET_PASSWORD):
        set_hash = await localnet_client.set_weights(alice, 0, uids, values, 0, dry_run=True)
        commit_tx = await localnet_client.commit_weights(
            alice,
            0,
            commit_hash,
            dry_run=True,
        )
        reveal_tx = await localnet_client.reveal_weights(
            alice,
            0,
            uids,
            values,
            salt_words,
            0,
            dry_run=True,
        )
    assert set_hash
    assert commit_tx
    assert reveal_tx


async def test_register_and_dissolve_network(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, _ = localnet_wallets
    with alice.session(password=LOCALNET_PASSWORD):
        register_hash = await localnet_client.register_network(alice, dry_run=True)
        dissolve_hash = await localnet_client.dissolve_network(alice, 0, dry_run=True)
    assert register_hash
    assert dissolve_hash


async def test_proxy_add_and_remove(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, bob = localnet_wallets
    assert bob.coldkey_ss58 is not None
    with alice.session(password=LOCALNET_PASSWORD):
        add_hash = await localnet_client.add_proxy(
            alice,
            bob.coldkey_ss58,
            "Any",
            dry_run=True,
        )
        remove_hash = await localnet_client.remove_proxy(
            alice,
            bob.coldkey_ss58,
            "Any",
            dry_run=True,
        )
    assert add_hash
    assert remove_hash


async def test_binding_validation_errors_before_submit(
    localnet_client: AsyncClient,
    localnet_wallets: tuple[Wallet, Wallet],
) -> None:
    alice, bob = localnet_wallets
    assert bob.coldkey_ss58 is not None
    with alice.session(password=LOCALNET_PASSWORD):
        with pytest.raises(ValidationError, match="cannot be negative"):
            await localnet_client.transfer(alice, bob.coldkey_ss58, -1, dry_run=True)
        with pytest.raises(ValidationError, match="invalid destination SS58"):
            await localnet_client.transfer(alice, "invalid-ss58", 1, dry_run=True)
        with pytest.raises(ValidationError, match="length mismatch"):
            await localnet_client.set_weights(alice, 0, [0, 1], [1], 0, dry_run=True)
        with pytest.raises(ValidationError, match="exceeds maximum value"):
            await localnet_client.add_stake(alice, 70_000, 1, dry_run=True)
