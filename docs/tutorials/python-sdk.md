# Python SDK tutorial

The Python SDK exposes async and sync clients, wallet lifecycle helpers, typed errors, event subscriptions, localnet helpers, and admin/extrinsic APIs through `agcli`.

## Install

```bash
python -m pip install agcli
```

## Connect + reads (`AsyncClient.connect_network` and `SyncClient`)

Source: `examples/python/connect_and_balance.py`

```python
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
```

## Wallet lifecycle (`Wallet.create`, `Wallet.create_from_uri`, `session(...)`)

Source: `examples/python/wallet_lifecycle.py`

```python
from __future__ import annotations

import tempfile
from pathlib import Path

from agcli import Wallet


def main() -> None:
    password = "password123"
    with tempfile.TemporaryDirectory(prefix="agcli-wallet-") as wallet_dir:
        created = Wallet.create(str(Path(wallet_dir)), "example", password)
        print(f"created wallet={created.wallet.name}")
        print(f"coldkey mnemonic words={len(created.coldkey_mnemonic.split())}")

        uri_wallet = Wallet.create_from_uri(str(Path(wallet_dir)), "//Alice", password)
        with uri_wallet.session(password=password) as session_wallet:
            print(f"session coldkey unlocked={session_wallet.is_coldkey_unlocked}")
            print(f"session hotkey loaded={session_wallet.is_hotkey_loaded}")


if __name__ == "__main__":
    main()
```

## Extrinsics (`transfer`, `add_stake`, `set_weights`, `dry_run`, `mev`)

Source: `examples/python/extrinsics_mvp.py`

```python
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
```

For a minimal transfer-only dry-run script, see `examples/python/transfer_dry_run.py`.

## Events (`subscribe_events`)

Source: `examples/python/subscribe_events.py`

```python
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
```

## Localnet (`start`, `dev_accounts`)

Source: `examples/python/localnet_smoke.py`

```python
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
```

## Typed errors and config

- Typed exceptions map to the Rust SDK exit code classes (`NetworkError`, `ValidationError`, `AuthError`, `ChainError`, `TimeoutError`, `IOError`).
- `Config.load()` / `Config.save()` let you persist endpoint and wallet defaults in the same shape as the CLI config.
- `examples/python/connect_and_balance.py` shows a typed `try/except` around connection/read calls and config-driven endpoint/network selection.
