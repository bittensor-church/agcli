import asyncio
import os

from agcli import AsyncClient, Config, Network, NetworkError, SyncClient, ValidationError


DEFAULT_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"


async def main() -> None:
    config = Config.load()
    address = os.environ.get("AGCLI_ADDRESS", DEFAULT_ADDRESS)
    network = Network.parse(os.environ.get("AGCLI_NETWORK", config.network or "finney"))
    endpoint = os.environ.get("AGCLI_WS", config.endpoint or "")

    try:
        client = await (
            AsyncClient.connect(endpoint)
            if endpoint
            else AsyncClient.connect_network(network)
        )
        balance = await client.get_balance(address)
        metagraph = await client.get_metagraph(1)
        portfolio = await client.fetch_portfolio(address)
        print(f"async endpoint={client.endpoint}")
        print(f"async balance[{address}]={balance.tao:.6f} TAO")
        print(f"metagraph netuid={metagraph['netuid']} n={metagraph['n']}")
        print(f"portfolio keys={sorted(portfolio.keys())}")
    except (NetworkError, ValidationError) as err:
        print(f"{err.__class__.__name__}: {err}")
        return

    sync_client = SyncClient.connect_network(network)
    sync_balance = sync_client.get_balance(address)
    print(f"sync endpoint={sync_client.endpoint}")
    print(f"sync balance[{address}]={sync_balance.tao:.6f} TAO")


if __name__ == "__main__":
    asyncio.run(main())
