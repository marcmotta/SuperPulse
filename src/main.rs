// src/main.rs
/*
 * Main executable for SuperPulse
 */

use clap::Parser;
use superpulse::{Result, run};

#[derive(Parser)]
#[command(version, about = "SuperPulse - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
