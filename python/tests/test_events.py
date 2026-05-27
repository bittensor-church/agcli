import pytest

from agcli.events import EVENT_CATEGORIES, EventFilter


def test_event_filter_defaults() -> None:
    f = EventFilter()
    assert f.category == "all"
    assert f.netuid is None
    assert f.account is None


def test_event_filter_roundtrip() -> None:
    f = EventFilter(category="staking", netuid=42, account="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
    payload = f.to_dict()
    assert payload == {
        "category": "staking",
        "netuid": 42,
        "account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    }
    restored = EventFilter(**payload)
    assert restored == f


def test_event_categories_contains_known_values() -> None:
    expected = {
        "all",
        "staking",
        "registration",
        "transfer",
        "weights",
        "subnet",
        "delegation",
        "keys",
        "swap",
        "governance",
        "crowdloan",
    }
    assert expected.issubset(set(EVENT_CATEGORIES))


@pytest.mark.localnet
@pytest.mark.asyncio
async def test_subscribe_blocks_yields_dicts() -> None:
    import os

    from agcli import AsyncClient

    ws = os.environ.get("AGCLI_LOCALNET_WS")
    if not ws:
        pytest.skip("AGCLI_LOCALNET_WS not set")
    client = await AsyncClient.connect(ws)
    stream = client.subscribe_blocks()
    try:
        first = await stream.__anext__()
        assert isinstance(first, dict)
        assert "block_number" in first
        assert "hash" in first
    finally:
        await stream.close()


@pytest.mark.localnet
@pytest.mark.asyncio
async def test_subscribe_events_with_filter() -> None:
    import os

    from agcli import AsyncClient

    ws = os.environ.get("AGCLI_LOCALNET_WS")
    if not ws:
        pytest.skip("AGCLI_LOCALNET_WS not set")
    client = await AsyncClient.connect(ws)
    stream = client.subscribe_events(EventFilter(category="all"))
    try:
        event = await stream.__anext__()
        assert isinstance(event, dict)
        for key in ("block_number", "pallet", "variant", "fields", "extrinsic_index"):
            assert key in event
    finally:
        await stream.close()
