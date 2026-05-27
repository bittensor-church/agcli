import textwrap

from agcli import scaffold


SAMPLE_TOML = textwrap.dedent(
    """\
    [chain]
    image = "ghcr.io/opentensor/subtensor-localnet:devnet-ready"
    container = "agcli_pytest_scaffold"
    port = 19944
    start = false
    timeout = 90

    [[subnet]]
    tempo = 50
    max_allowed_validators = 4
    min_allowed_weights = 1
    weights_rate_limit = 0
    commit_reveal = true

    [[subnet.neuron]]
    name = "validator1"
    fund_tao = 500.0
    register = true

    [[subnet.neuron]]
    name = "miner1"
    fund_tao = 50.0
    register = true
    """
)


def test_load_config_roundtrip(tmp_path) -> None:
    path = tmp_path / "scaffold.toml"
    path.write_text(SAMPLE_TOML)

    cfg = scaffold.load_config(str(path))

    assert cfg.chain.container == "agcli_pytest_scaffold"
    assert cfg.chain.port == 19944
    assert cfg.chain.start is False
    assert cfg.chain.timeout == 90

    subnets = cfg.subnets
    assert len(subnets) == 1
    subnet = subnets[0]
    assert subnet.tempo == 50
    assert subnet.max_allowed_validators == 4
    assert subnet.min_allowed_weights == 1
    assert subnet.weights_rate_limit == 0
    assert subnet.commit_reveal is True

    neurons = subnet.neurons
    assert [n.name for n in neurons] == ["validator1", "miner1"]
    assert neurons[0].fund_tao == 500.0
    assert neurons[0].register is True
    assert neurons[1].fund_tao == 50.0

    payload = cfg.to_dict()
    assert payload["chain"]["container"] == "agcli_pytest_scaffold"
    assert payload["chain"]["port"] == 19944
    assert payload["subnets"][0]["tempo"] == 50
    assert payload["subnets"][0]["neurons"][0]["name"] == "validator1"


def test_scaffold_config_constructor_defaults() -> None:
    cfg = scaffold.ScaffoldConfig()
    assert cfg.chain.port == 9944
    assert cfg.chain.start is True
    assert cfg.subnets == []


def test_neuron_config_constructor() -> None:
    n = scaffold.NeuronConfig("alpha", fund_tao=100.0, register=False)
    payload = n.to_dict()
    assert payload == {"name": "alpha", "fund_tao": 100.0, "register": False}
