//! # agcli — Rust SDK + CLI for the Bittensor Network
//!
//! `agcli` provides a complete toolkit for interacting with the Bittensor
//! blockchain (subtensor). It covers wallet management, staking, transfers,
//! subnet operations, weight setting, registration, and chain queries.
//!
//! ## Quick Start (SDK)
//!
//! ```rust,no_run
//! use agcli::{Client, Wallet};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::connect("wss://entrypoint-finney.opentensor.ai:443").await?;
//!     let wallet = Wallet::open("~/.bittensor/wallets/default")?;
//!     let coldkey = wallet.coldkey_public()?;
//!     let balance = client.get_balance(&coldkey).await?;
//!     println!("Balance: {} TAO", balance.tao());
//!     Ok(())
//! }
//! ```

pub mod admin;
pub mod chain;
pub mod config;
pub mod error;
pub mod events;
pub mod extrinsics;
pub mod live;
pub mod localnet;
pub mod queries;
pub mod scaffold;
pub mod types;
pub mod utils;
pub mod wallet;

#[cfg(feature = "cli")]
pub mod cli;

/// Generated chain API from subtensor runtime metadata (build.rs).
#[allow(dead_code, unused_imports, non_camel_case_types, clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/metadata.rs"));
}
pub use generated::api;

pub use subxt::config::SubstrateConfig as SubtensorConfig;

pub type AccountId = <SubtensorConfig as subxt::Config>::AccountId;
pub type Hash = <SubtensorConfig as subxt::Config>::Hash;

// Re-exports for ergonomic SDK use
pub use chain::Client;
pub use config::Config;
pub use types::balance::{AlphaBalance, Balance, LimitPriceRao};
pub use wallet::Wallet;

pub mod sdk {
    pub use crate::{
        error, types::chain_data, types::network::NetUid, types::Network, Balance, Client, Config,
        Wallet,
    };
}

/// Embedded runtime metadata for extrinsic encoding regression tests.
///
/// Requires the `test-utils` feature (`cargo test --features test-utils`).
#[cfg(feature = "test-utils")]
#[doc(hidden)]
pub fn test_metadata() -> &'static subxt::metadata::Metadata {
    use parity_scale_codec::Decode;
    use std::sync::OnceLock;

    static METADATA: OnceLock<subxt::metadata::Metadata> = OnceLock::new();
    METADATA.get_or_init(|| {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/metadata.scale"));
        let mut slice: &[u8] = bytes;
        subxt_metadata::Metadata::decode(&mut slice)
            .expect("metadata.scale should decode")
            .into()
    })
}
