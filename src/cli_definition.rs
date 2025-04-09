// ─────────────────────────────────────────────────────────────────────────────
// CLI definitions
// ─────────────────────────────────────────────────────────────────────────────

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Classify proxy type
    Detect { proxy: String },
    /// Current implementation address
    Impl { proxy: String },
    /// Compare stored impl with current
    Check { proxy: String, last_impl: String },
}
