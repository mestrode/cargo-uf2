//! Cargo-UF2 CLI tool.

mod convert;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "uf2", about = "UF2 file utility (cargo-uf2)", version)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Convert between binary and UF2 formats.
    Convert(convert::Cmd),
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommand::Convert(cmd) => cmd.run(),
    }
}
