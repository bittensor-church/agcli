import asyncio

from agcli import AsyncClient, localnet


async def main() -> None:
    info = await localnet.start()
    try:
        print(f"localnet endpoint={info.endpoint}")
        accounts = localnet.dev_accounts()
        for account in accounts:
            print(f"{account.name}: {account.ss58}")

        client = await AsyncClient.connect(info.endpoint)
        alice = accounts[0]
        balance = await client.get_balance(alice.ss58)
        print(f"alice balance={balance.tao:.6f} TAO")
    finally:
        localnet.stop(info.container_name)


if __name__ == "__main__":
    asyncio.run(main())
