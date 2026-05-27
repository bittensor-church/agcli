import pytest

from agcli import admin


EXPECTED_FUNCTION_NAMES = {
    "set_tempo",
    "set_max_allowed_validators",
    "set_max_allowed_uids",
    "set_immunity_period",
    "set_min_allowed_weights",
    "set_max_weight_limit",
    "set_weights_set_rate_limit",
    "set_commit_reveal_weights_enabled",
    "set_difficulty",
    "set_bonds_moving_average",
    "set_target_registrations_per_interval",
    "set_activity_cutoff",
    "set_serving_rate_limit",
    "set_default_take",
    "set_tx_rate_limit",
    "set_min_difficulty",
    "set_max_difficulty",
    "set_adjustment_interval",
    "set_adjustment_alpha",
    "set_kappa",
    "set_rho",
    "set_min_burn",
    "set_max_burn",
    "set_liquid_alpha_enabled",
    "set_alpha_values",
    "set_yuma3_enabled",
    "set_bonds_penalty",
    "set_subnet_moving_alpha",
    "set_mechanism_count",
    "set_mechanism_emission_split",
    "set_stake_threshold",
    "set_nominator_min_required_stake",
    "set_network_registration_allowed",
    "set_network_pow_registration_allowed",
    "raw_admin_call",
}


def test_admin_exports_every_set_fn() -> None:
    for name in EXPECTED_FUNCTION_NAMES:
        assert hasattr(admin, name), f"admin.{name} is missing"
        fn = getattr(admin, name)
        assert callable(fn)


def test_known_params_non_empty_and_well_formed() -> None:
    params = admin.known_params()
    assert isinstance(params, list)
    assert len(params) > 0
    names = set()
    for entry in params:
        assert len(entry) == 3
        name, description, args = entry
        assert isinstance(name, str)
        assert isinstance(description, str)
        assert isinstance(args, list)
        assert name.startswith("sudo_set_")
        assert description
        assert args
        assert name not in names
        names.add(name)


def test_known_params_covers_set_tempo() -> None:
    params = {name: (desc, args) for name, desc, args in admin.known_params()}
    assert "sudo_set_tempo" in params
    desc, args = params["sudo_set_tempo"]
    assert "epoch" in desc.lower() or "tempo" in desc.lower()
    assert args[0] == "netuid: u16"


@pytest.mark.network
def test_module_import_is_clean() -> None:
    import importlib

    importlib.import_module("agcli.admin")
