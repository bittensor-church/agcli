import asyncio
import os
import tempfile
from pathlib import Path

from agcli import AsyncClient, Wallet


DEFAULT_DEST = "5FHneW46xGXgs5mUiveU4sbTyGBzmst7hQfYQfR6N2P2Vf5Q"


async def main() -> None:
    endpoint = os.environ.get("AGCLI_WS", "ws://127.0.0.1:9944")
    password = os.environ.get("AGCLI_PASSWORD", "password123")
    dest_ss58 = os.environ.get("AGCLI_DEST", DEFAULT_DEST)

    with tempfile.TemporaryDirectory(prefix="agcli-py-") as wallet_dir:
        wallet = Wallet.create_from_uri(str(Path(wallet_dir)), "//Alice", password)
        wallet.load_hotkey("default")
        wallet.unlock_coldkey(password)

        client = await AsyncClient.connect(endpoint)
        transfer_hash = await client.transfer(
            wallet,
            dest_ss58,
            10_000_000,
            dry_run=True,
            mev=False,
        )
        stake_hash = await client.add_stake(
            wallet,
            1,
            10_000_000,
            dry_run=True,
            mev=False,
        )
        weights_hash = await client.set_weights(
            wallet,
            1,
            [0, 1],
            [32_767, 32_768],
            0,
            dry_run=True,
            mev=False,
        )
        print(f"transfer={transfer_hash}")
        print(f"add_stake={stake_hash}")
        print(f"set_weights={weights_hash}")


if __name__ == "__main__":
    asyncio.run(main())
