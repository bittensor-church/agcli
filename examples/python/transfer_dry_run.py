import asyncio
import os
import tempfile
from pathlib import Path

from agcli import AsyncClient, Wallet


DEFAULT_DEST = "5FHneW46xGXgs5mUiveU4sbTyGBzmst7hQfYQfR6N2P2Vf5Q"


async def main() -> None:
    endpoint = os.environ.get("AGCLI_WS", "ws://127.0.0.1:9944")
    dest_ss58 = os.environ.get("AGCLI_DEST", DEFAULT_DEST)
    uri = os.environ.get("AGCLI_URI", "//Alice")
    password = os.environ.get("AGCLI_PASSWORD", "password123")

    with tempfile.TemporaryDirectory(prefix="agcli-py-") as wallet_dir:
        wallet = Wallet.create_from_uri(str(Path(wallet_dir)), uri, password)
        wallet.load_hotkey("default")
        wallet.unlock_coldkey(password)

        client = await AsyncClient.connect(endpoint)
        extrinsic_hash = await client.transfer(
            wallet,
            dest_ss58,
            10_000_000,
            dry_run=True,
            mev=False,
        )
        print(f"dry-run transfer hash={extrinsic_hash}")


if __name__ == "__main__":
    asyncio.run(main())
