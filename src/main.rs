//! proxy_detector_alloy.rs — using Alloy (2025‑04‑09)
//! -------------------------------------------------------------
//! Detect EVM proxy patterns, fetch current implementation, and
//! check if it changed — implemented with the *Alloy* ecosystem
//! instead of ethers‑rs.
//!
//! Build
//!   cargo run --release -- <flags>
//!
//! Flags (mutually exclusive):
//!   <PROXY>                           classify proxy type
//!   --impl  <PROXY>                   print current implementation
//!   --check <PROXY> <LAST_IMPL>       CHANGED / UNCHANGED + impl
//!
//! Env
//!   RPC_URL           JSON‑RPC endpoint (https or ipc path)
//!   INFURA_API_KEY    fallback if RPC_URL missing
//!
//! Crate features (Cargo.toml):
//!   alloy-primitives = "0.5"
//!   alloy-provider   = { version = "0.5", features = ["reqwest"] }
//!   alloy-transport-http = "0.5"
//!   alloy-sol-types = "0.5"
//!   clap = { version = "4", features = ["derive"] }
//!   tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
//!   anyhow = "1"
//!   hex = "0.4"

pub mod app;
pub mod cli_definition;
pub mod domain;

#[tokio::main]
async fn main() -> anyhow::Result<()>   {
    Ok(())
}
