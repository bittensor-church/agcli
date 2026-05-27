"""Config facade."""

from __future__ import annotations

from agcli import _agcli


class Config:
    """Wrapper around the native ``agcli._agcli.Config``."""

    def __init__(self, inner: _agcli.Config | None = None, **kwargs) -> None:
        object.__setattr__(self, "_inner", inner or _agcli.Config())
        for key, value in kwargs.items():
            setattr(self._inner, key, value)

    @classmethod
    def load(cls) -> Config:
        return cls(_agcli.Config.load())

    @classmethod
    def load_from(cls, path: str) -> Config:
        return cls(_agcli.Config.load_from(path))

    @staticmethod
    def default_path() -> str:
        return _agcli.Config.default_path()

    def save(self) -> None:
        self._inner.save()

    def save_to(self, path: str) -> None:
        self._inner.save_to(path)

    def __getattr__(self, name: str):
        return getattr(self._inner, name)

    def __setattr__(self, name: str, value) -> None:
        if name == "_inner":
            object.__setattr__(self, name, value)
            return
        setattr(self._inner, name, value)

    def __repr__(self) -> str:
        return f"Config(network={self.network!r}, endpoint={self.endpoint!r})"
