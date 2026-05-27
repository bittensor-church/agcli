import pytest

from agcli import live


def test_live_module_exports() -> None:
    assert callable(live.live_dynamic)
    assert callable(live.live_metagraph)
    assert callable(live.live_portfolio)


@pytest.mark.localnet
@pytest.mark.asyncio
async def test_live_module_smoke() -> None:
    import asyncio
    import os

    from agcli import AsyncClient

    ws = os.environ.get("AGCLI_LOCALNET_WS")
    if not ws:
        pytest.skip("AGCLI_LOCALNET_WS not set")
    client = await AsyncClient.connect(ws)
    task = asyncio.create_task(live.live_dynamic(client, interval_secs=5))
    await asyncio.sleep(0.1)
    task.cancel()
    with pytest.raises(asyncio.CancelledError):
        await task
