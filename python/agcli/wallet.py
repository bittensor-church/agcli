"""Wallet facade."""

from __future__ import annotations

from contextlib import AbstractContextManager
from dataclasses import dataclass

from agcli import _agcli


@dataclass(frozen=True)
class WalletCreateResult:
    wallet: Wallet
    coldkey_mnemonic: str
    hotkey_mnemonic: str


class WalletSession(AbstractContextManager["Wallet"]):
    def __init__(self, wallet: Wallet, password: str) -> None:
        self._wallet = wallet
        self._password = password

    def __enter__(self) -> Wallet:
        self._wallet.unlock_coldkey_with_password(self._password)
        if not self._wallet.is_hotkey_loaded:
            self._wallet.load_hotkey("default")
        return self._wallet

    def __exit__(self, exc_type, exc, tb) -> bool:
        self._wallet.lock()
        return False


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
    def create_from_uri(cls, wallet_dir: str, uri: str, password: str) -> Wallet:
        return cls(_agcli.Wallet.create_from_uri(wallet_dir, uri, password))

    @classmethod
    def list_wallets(cls, wallet_dir: str) -> list[str]:
        return _agcli.Wallet.list_wallets(wallet_dir)

    def unlock_coldkey(self, password: str) -> None:
        self._inner.unlock_coldkey(password)

    def unlock_coldkey_with_password(self, password: str) -> None:
        self._inner.unlock_coldkey_with_password(password)

    def load_hotkey(self, hotkey_name: str) -> None:
        self._inner.load_hotkey(hotkey_name)

    def lock(self) -> None:
        self._inner.lock()

    @property
    def is_coldkey_unlocked(self) -> bool:
        return self._inner.is_coldkey_unlocked

    @property
    def is_hotkey_loaded(self) -> bool:
        return self._inner.is_hotkey_loaded

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
    def coldkey_public_ss58(self) -> str | None:
        return self._inner.coldkey_public_ss58

    @property
    def hotkey_ss58(self) -> str | None:
        return self._inner.hotkey_ss58

    def list_hotkeys(self) -> list[str]:
        return self._inner.list_hotkeys()

    def sign_message(self, role: str, message: bytes) -> bytes:
        return self._inner.sign_message(role, message)

    def session(self, *, password: str) -> WalletSession:
        return WalletSession(self, password)

    @staticmethod
    def verify_message(ss58: str, message: bytes, signature: bytes) -> bool:
        return _agcli.Wallet.verify_message(ss58, message, signature)

    def __repr__(self) -> str:
        return repr(self._inner)
