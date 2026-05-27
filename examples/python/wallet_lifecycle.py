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
