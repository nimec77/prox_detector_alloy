//! proxy_detector_alloy.rs — upgraded to **Alloy 0.14**
//! ---------------------------------------------------------------------------
//! Same functionality (detect proxy, get implementation, check upgrades) but
//! all Alloy crates are bumped to the current *0.14* line.  No external
//! dependencies besides those shown below.
//!
//! Add this to **Cargo.toml**:
//! ```toml
//! [dependencies]
//! alloy-primitives      = "0.14"
//! alloy-provider        = { version = "0.14", features = ["reqwest"] }
//! alloy-transport-http  = "0.14"
//! alloy-rpc-types       = "0.14"
//! clap                  = { version = "4", features = ["derive"] }
//! tokio                 = { version = "1", features = ["rt-multi-thread", "macros"] }
//! anyhow                = "1"
//! hex                   = "0.4"
//! serde_json            = "1"
//! ```
//!
//! **Build / run**
//! ```bash
//! export RPC_URL=https://mainnet.infura.io/v3/<key>
//! cargo run --release -- --impl 0xdAC17F958D2ee523a2206206994597C13D831ec7
//! ```
//!
use std::str::FromStr;

use alloy_primitives::Address;
use clap::Parser;
use cli_definition::{Cli, Command};
use di::build_provider;
use domain::helpers::{detect_proxy, get_implementation_address, has_implementation_changed};

pub mod app;
pub mod cli_definition;
pub mod di;
pub mod domain;

// ─────────────────────────────────────────────────────────────────────────────
// Main entry
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let provider = build_provider()?;

    match cli.cmd {
        Command::Detect { proxy } => {
            let proxy = Address::from_str(&proxy)?;
            let kind = detect_proxy(proxy, &provider).await?;
            println!("{proxy:?} → {kind}");
        }
        Command::Impl { proxy } => {
            let proxy = Address::from_str(&proxy)?;
            match get_implementation_address(proxy, &provider).await? {
                Some(addr) => println!("{addr:?}"),
                None => println!("<no implementation>"),
            }
        }
        Command::Check { proxy, last_impl } => {
            let proxy = Address::from_str(&proxy)?;
            let last_impl = Address::from_str(&last_impl)?;
            let res = has_implementation_changed(proxy, &provider, last_impl).await?;
            match res.current {
                None => println!("<no implementation>"),
                Some(curr) if res.changed => println!("CHANGED  {last_impl:?} → {curr:?}"),
                Some(curr) => println!("UNCHANGED  {curr:?}"),
            }
        }
    }

    Ok(())
}
