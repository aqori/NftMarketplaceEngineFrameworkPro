// src/main.rs
/*
 * Main executable for NftMarketplaceEngineFrameworkPro
 */

use clap::Parser;
use nftmarketplaceengineframeworkpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineFrameworkPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
