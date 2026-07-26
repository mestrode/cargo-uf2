use anyhow::Error;
use clap::Parser;
use std::path::PathBuf;
use uftwo::reader;

#[derive(Parser)]
pub struct Cmd {
    /// Input UF2 file to verify
    #[arg(value_name = "INPUT")]
    input_path: PathBuf,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        let bytes = std::fs::read(&self.input_path)?;
        let uf2_file = reader::from_bytes(&bytes)?;

        match reader::verify(&uf2_file) {
            Ok(()) => {
                println!("✓ UF2 file is valid.");
                println!("  Blocks: {}", uf2_file.len());
                Ok(())
            }
            Err(e) => {
                println!("✗ UF2 file is invalid: {}", e);
                Err(Error::msg(format!("Verification failed: {}", e)))
            }
        }
    }
}
