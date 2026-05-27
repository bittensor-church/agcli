"""Typed views of native binding objects."""

from __future__ import annotations

from agcli import _agcli

Balance = _agcli.Balance
NetUid = _agcli.NetUid
Network = _agcli.Network

__all__ = ["Balance", "NetUid", "Network"]
