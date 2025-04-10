// ─────────────────────────────────────────────────────────────────────────────
// CLI definitions
// ─────────────────────────────────────────────────────────────────────────────

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Classify proxy type
    Detect { proxy: String },
    /// Current implementation address
    Impl { proxy: String },
    /// Compare stored impl with current
    Check { proxy: String, last_impl: String },
}
