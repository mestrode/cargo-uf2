//! Cargo-UF2 CLI tool.

use clap::Parser;

#[derive(Parser)]
#[clap(name = "uf2", about = "UF2 file utility (cargo-uf2)", version)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    // Subcommands will be added in subsequent commits
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if let Some(subcommand) = args.subcommand {
        match subcommand {
            // Match arms will be added with each subcommand
        }
    }
    Ok(())
}
