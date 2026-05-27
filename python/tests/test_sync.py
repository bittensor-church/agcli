import pytest

from agcli import Network, SyncClient


@pytest.mark.network
def test_sync_client_core_reads() -> None:
    client = SyncClient.connect_network(Network.finney())
    assert client.endpoint.startswith("ws")

    issuance = client.get_total_issuance()
    assert issuance.rao >= 0

    metagraph = client.get_metagraph(1)
    assert isinstance(metagraph, dict)
    assert metagraph["netuid"] == 1

    balance = client.get_balance("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
    assert balance.rao >= 0
