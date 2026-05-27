"""Wallet facade."""

from __future__ import annotations

from dataclasses import dataclass

from agcli import _agcli


@dataclass(frozen=True)
class WalletCreateResult:
    wallet: Wallet
    coldkey_mnemonic: str
    hotkey_mnemonic: str


class Wallet:
    """Wrapper around the native ``agcli._agcli.Wallet``."""

    def __init__(self, inner: _agcli.Wallet) -> None:
        self._inner = inner

    @classmethod
    def open(cls, path: str) -> Wallet:
        return cls(_agcli.Wallet.open(path))

    @classmethod
    def create(
        cls,
        wallet_dir: str,
        name: str,
        password: str,
        *,
        hotkey_name: str = "default",
    ) -> WalletCreateResult:
        wallet, coldkey_mnemonic, hotkey_mnemonic = _agcli.Wallet.create(
            wallet_dir,
            name,
            password,
            hotkey_name,
        )
        return WalletCreateResult(
            wallet=cls(wallet),
            coldkey_mnemonic=coldkey_mnemonic,
            hotkey_mnemonic=hotkey_mnemonic,
        )

    @classmethod
    def import_from_mnemonic(
        cls,
        wallet_dir: str,
        name: str,
        mnemonic: str,
        password: str,
    ) -> Wallet:
        return cls(
            _agcli.Wallet.import_from_mnemonic(wallet_dir, name, mnemonic, password)
        )

    @classmethod
    def list_wallets(cls, wallet_dir: str) -> list[str]:
        return _agcli.Wallet.list_wallets(wallet_dir)

    def unlock_coldkey(self, password: str) -> None:
        self._inner.unlock_coldkey(password)

    def load_hotkey(self, hotkey_name: str) -> None:
        self._inner.load_hotkey(hotkey_name)

    @property
    def name(self) -> str:
        return self._inner.name

    @property
    def path(self) -> str:
        return self._inner.path

    @property
    def coldkey_ss58(self) -> str | None:
        return self._inner.coldkey_ss58

    @property
    def hotkey_ss58(self) -> str | None:
        return self._inner.hotkey_ss58

    def list_hotkeys(self) -> list[str]:
        return self._inner.list_hotkeys()

    def __repr__(self) -> str:
        return repr(self._inner)
