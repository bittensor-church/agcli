import asyncio
import os

from agcli import AsyncClient
from agcli.events import EventFilter


async def main() -> None:
    endpoint = os.environ.get("AGCLI_WS", "ws://127.0.0.1:9944")
    limit = int(os.environ.get("AGCLI_EVENT_LIMIT", "5"))
    category = os.environ.get("AGCLI_EVENT_CATEGORY", "all")

    client = await AsyncClient.connect(endpoint)
    stream = client.subscribe_events(EventFilter(category=category))
    try:
        for _ in range(limit):
            event = await stream.__anext__()
            print(
                f"#{event['block_number']} {event['pallet']}.{event['variant']} "
                f"extrinsic_index={event['extrinsic_index']}"
            )
    finally:
        await stream.close()


if __name__ == "__main__":
    asyncio.run(main())
