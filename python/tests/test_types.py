import pytest

from agcli.types import AlphaBalance, ChainIdentity, SubnetInfo


def test_subnet_info_roundtrip() -> None:
    value = {
        "netuid": 1,
        "name": "alpha",
        "symbol": "A",
        "n": 128,
        "max_n": 4096,
        "tempo": 360,
        "emission_value": 10,
        "burn": {"rao": 1_000_000_000},
        "difficulty": 1_000,
        "immunity_period": 100,
        "owner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "registration_allowed": True,
    }
    typed = SubnetInfo(value)
    assert typed.netuid == 1
    assert typed.name == "alpha"
    assert typed.to_dict()["symbol"] == "A"


def test_alpha_balance_roundtrip() -> None:
    alpha = AlphaBalance(2_500_000_000)
    assert alpha.raw == 2_500_000_000
    assert alpha.rao == 2_500_000_000
    assert alpha.tao == pytest.approx(2.5)
    assert alpha.to_dict()["raw"] == 2_500_000_000


def test_chain_identity_roundtrip() -> None:
    value = {
        "name": "alice",
        "url": "https://example.org",
        "github": "alice/repo",
        "image": "https://example.org/image.png",
        "discord": "alice#1234",
        "description": "identity",
        "additional": "none",
    }
    typed = ChainIdentity(value)
    assert typed.name == "alice"
    assert typed.to_dict()["url"] == "https://example.org"
