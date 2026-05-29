//! Shared utilities.

pub mod explain;
pub mod format;
pub mod pow;

pub use format::*;

/// Validate subnet lease emissions share (0–100 percent).
pub fn validate_emissions_share(value: u8) -> anyhow::Result<()> {
    if value > 100 {
        anyhow::bail!("emissions_share must be 0–100, got {value}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // Verify that re-exported format module functions are accessible
    // through the utils module path (via `pub use format::*`).

    #[test]
    fn re_export_short_ss58_accessible() {
        // short_ss58 is defined in format.rs and re-exported by this module
        let addr = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let short = super::short_ss58(addr);
        assert_eq!(short, "5Grw...utQY");
    }

    #[test]
    fn re_export_truncate_accessible() {
        // truncate is defined in format.rs and re-exported by this module
        let result = super::truncate("hello world", 5);
        assert_eq!(result, "hell\u{2026}"); // 4 chars + ellipsis
    }

    #[test]
    fn re_export_u16_to_float_accessible() {
        let result = super::u16_to_float(0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn validate_emissions_share_accepts_range() {
        assert!(super::validate_emissions_share(0).is_ok());
        assert!(super::validate_emissions_share(100).is_ok());
    }

    #[test]
    fn validate_emissions_share_rejects_over_100() {
        assert!(super::validate_emissions_share(101).is_err());
    }

    #[test]
    fn re_export_float_to_u16_accessible() {
        let result = super::float_to_u16(1.0);
        assert_eq!(result, 65535);
    }

    #[test]
    fn re_export_format_tao_accessible() {
        // format_tao is defined in format.rs and re-exported by this module
        let b = crate::types::Balance::from_tao(1.0);
        let s = super::format_tao(b);
        assert!(s.contains("1."));
    }

    #[test]
    fn explain_module_accessible() {
        // The explain submodule should be publicly accessible
        assert!(super::explain::explain("tempo").is_some());
    }
}
