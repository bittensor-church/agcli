//! On-chain extrinsic argument units — single source of truth for CLI encoding, batch limits, and docs.
//!
//! τ (TAO) and α (alpha) share 9 decimal places and encode as u64 on-wire; semantic unit is
//! tracked here, not in subxt metadata.

/// Semantic unit for a dispatchable argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgUnit {
    /// Free TAO from coldkey (`TaoBalance` / RAO).
    TaoRao,
    /// Subnet alpha stake (`AlphaBalance` raw u64).
    AlphaRaw,
    /// Limit order price: TAO per alpha, on-chain RAO/α (`limit_price`).
    TaoPerAlphaRao,
    /// u16 normalized to [0,1] via ÷65535 (kappa, bonds_penalty, childkey/delegate take input).
    NormalizedU16,
    /// u16 sigmoid scale, **not** ÷65535 (rho).
    RhoScaleU16,
    /// Child proportion u64; runtime uses value ÷ u64::MAX.
    U64OverU64Max,
    /// Childkey take u16; runtime uses value ÷65535 (18% = 11796).
    TakeU16_65535,
    /// Per-UID weight u16 (0–65535, vector normalized on-chain).
    WeightU16,
    /// Plain integer / enum / account — no amount conversion.
    Other,
}

impl ArgUnit {
    pub fn label(self) -> &'static str {
        match self {
            Self::TaoRao => "τ (TAO, RAO)",
            Self::AlphaRaw => "α (alpha, raw u64)",
            Self::TaoPerAlphaRao => "τ/α (RAO per alpha)",
            Self::NormalizedU16 => "u16÷65535 or decimal 0–1",
            Self::RhoScaleU16 => "u16 sigmoid scale (not ÷65535)",
            Self::U64OverU64Max => "u64÷u64::MAX or decimal 0–1",
            Self::TakeU16_65535 => "take u16 (÷65535, max 18% = 11796)",
            Self::WeightU16 => "weight u16 (0–65535)",
            Self::Other => "—",
        }
    }

    pub fn is_tao_spending(self) -> bool {
        matches!(self, Self::TaoRao)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExtrinsicArgSpec {
    pub name: &'static str,
    pub unit: ArgUnit,
}

#[derive(Debug, Clone, Copy)]
pub struct ExtrinsicSpec {
    pub pallet: &'static str,
    pub call: &'static str,
    pub args: &'static [ExtrinsicArgSpec],
}

impl ExtrinsicSpec {
    /// Index of `(netuid, tao_amount)` for `check_spending_limit`, if this call spends free TAO.
    pub fn tao_spending_indices(&self) -> Option<(usize, usize)> {
        let mut netuid_idx = None;
        let mut amount_idx = None;
        for (i, arg) in self.args.iter().enumerate() {
            if arg.name == "netuid" || arg.name == "origin_netuid" {
                netuid_idx = Some(i);
            }
            if arg.unit.is_tao_spending() {
                amount_idx = Some(i);
            }
        }
        match (netuid_idx, amount_idx) {
            (Some(n), Some(a)) => Some((n, a)),
            _ => None,
        }
    }

    pub fn find(pallet: &str, call: &str) -> Option<&'static ExtrinsicSpec> {
        ALL_SPECS
            .iter()
            .find(|s| s.pallet == pallet && s.call == call)
    }
}

/// All registered extrinsic argument specs (staking + phase-2 extensions).
pub static ALL_SPECS: &[ExtrinsicSpec] = &[
    // ── Staking (SubtensorModule) ──
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "add_stake",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount_staked",
                unit: ArgUnit::TaoRao,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "remove_stake",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount_unstaked",
                unit: ArgUnit::AlphaRaw,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "move_stake",
        args: &[
            ExtrinsicArgSpec {
                name: "origin_hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "destination_hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "origin_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "destination_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "alpha_amount",
                unit: ArgUnit::AlphaRaw,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "transfer_stake",
        args: &[
            ExtrinsicArgSpec {
                name: "destination_coldkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "origin_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "destination_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "alpha_amount",
                unit: ArgUnit::AlphaRaw,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "swap_stake",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "origin_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "destination_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "alpha_amount",
                unit: ArgUnit::AlphaRaw,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "add_stake_limit",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount_staked",
                unit: ArgUnit::TaoRao,
            },
            ExtrinsicArgSpec {
                name: "limit_price",
                unit: ArgUnit::TaoPerAlphaRao,
            },
            ExtrinsicArgSpec {
                name: "allow_partial",
                unit: ArgUnit::Other,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "remove_stake_limit",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount_unstaked",
                unit: ArgUnit::AlphaRaw,
            },
            ExtrinsicArgSpec {
                name: "limit_price",
                unit: ArgUnit::TaoPerAlphaRao,
            },
            ExtrinsicArgSpec {
                name: "allow_partial",
                unit: ArgUnit::Other,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "swap_stake_limit",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "origin_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "destination_netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "alpha_amount",
                unit: ArgUnit::AlphaRaw,
            },
            ExtrinsicArgSpec {
                name: "limit_price",
                unit: ArgUnit::TaoPerAlphaRao,
            },
            ExtrinsicArgSpec {
                name: "allow_partial",
                unit: ArgUnit::Other,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "remove_stake_full_limit",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "limit_price",
                unit: ArgUnit::TaoPerAlphaRao,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "recycle_alpha",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount",
                unit: ArgUnit::AlphaRaw,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "burn_alpha",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount",
                unit: ArgUnit::AlphaRaw,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "set_childkey_take",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "take",
                unit: ArgUnit::TakeU16_65535,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "set_children",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "children",
                unit: ArgUnit::U64OverU64Max,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "unstake_all",
        args: &[ExtrinsicArgSpec {
            name: "hotkey",
            unit: ArgUnit::Other,
        }],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "unstake_all_alpha",
        args: &[ExtrinsicArgSpec {
            name: "hotkey",
            unit: ArgUnit::Other,
        }],
    },
    // ── Weights ──
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "set_weights",
        args: &[
            ExtrinsicArgSpec {
                name: "netuid",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "dests",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "weights",
                unit: ArgUnit::WeightU16,
            },
            ExtrinsicArgSpec {
                name: "version_key",
                unit: ArgUnit::Other,
            },
        ],
    },
    // ── Delegate take ──
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "increase_take",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "take",
                unit: ArgUnit::TakeU16_65535,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "SubtensorModule",
        call: "decrease_take",
        args: &[
            ExtrinsicArgSpec {
                name: "hotkey",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "take",
                unit: ArgUnit::TakeU16_65535,
            },
        ],
    },
    // ── Transfers / crowdloan ──
    ExtrinsicSpec {
        pallet: "Balances",
        call: "transfer_allow_death",
        args: &[
            ExtrinsicArgSpec {
                name: "dest",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "value",
                unit: ArgUnit::TaoRao,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "Balances",
        call: "transfer_keep_alive",
        args: &[
            ExtrinsicArgSpec {
                name: "dest",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "value",
                unit: ArgUnit::TaoRao,
            },
        ],
    },
    ExtrinsicSpec {
        pallet: "Crowdloan",
        call: "contribute",
        args: &[
            ExtrinsicArgSpec {
                name: "crowdloan_id",
                unit: ArgUnit::Other,
            },
            ExtrinsicArgSpec {
                name: "amount",
                unit: ArgUnit::TaoRao,
            },
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_specs_have_unique_pallet_call() {
        let mut seen = std::collections::HashSet::new();
        for spec in ALL_SPECS {
            let key = (spec.pallet, spec.call);
            assert!(
                seen.insert(key),
                "duplicate spec: {}::{}",
                spec.pallet,
                spec.call
            );
        }
    }

    #[test]
    fn add_stake_tao_spending_indices() {
        let spec = ExtrinsicSpec::find("SubtensorModule", "add_stake").unwrap();
        assert_eq!(spec.tao_spending_indices(), Some((1, 2)));
    }

    #[test]
    fn transfer_stake_has_no_tao_spending() {
        let spec = ExtrinsicSpec::find("SubtensorModule", "transfer_stake").unwrap();
        assert_eq!(spec.tao_spending_indices(), None);
        assert_eq!(spec.args[4].unit, ArgUnit::AlphaRaw);
    }

    #[test]
    fn phase2_specs_tao_and_special_units() {
        let weights = ExtrinsicSpec::find("SubtensorModule", "set_weights").unwrap();
        assert!(weights.args.iter().any(|a| a.unit == ArgUnit::WeightU16));

        for call in ["increase_take", "decrease_take"] {
            let spec = ExtrinsicSpec::find("SubtensorModule", call).unwrap();
            assert_eq!(spec.args[1].unit, ArgUnit::TakeU16_65535);
        }

        let contribute = ExtrinsicSpec::find("Crowdloan", "contribute").unwrap();
        assert_eq!(contribute.args[1].unit, ArgUnit::TaoRao);
        assert_eq!(contribute.tao_spending_indices(), None);

        for call in ["transfer_allow_death", "transfer_keep_alive"] {
            let spec = ExtrinsicSpec::find("Balances", call).unwrap();
            assert_eq!(spec.args[1].unit, ArgUnit::TaoRao);
        }
    }
}
