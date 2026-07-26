use anyhow::Error;
use clap::Parser;
use clap_num::maybe_hex;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use uftwo::{Uf2File, reader};

// Load mappings at compile time
const FAMILY_PAGE_SIZES: &str = include_str!("../data/family_pagesizes.json");

#[derive(Parser)]
pub struct Cmd {
    #[arg(value_name = "INPUT")]
    input_path: PathBuf,
    #[arg(value_name = "OUTPUT")]
    output_path: Option<PathBuf>,

    /// Target base address in flash memory (example: 0x08000000)
    #[clap(short, long, value_parser=maybe_hex::<u32>, default_value = "0x00")]
    base_addr: u32,
    /// Family ID (example: 0xADA52840)
    #[clap(short, long, value_parser=maybe_hex::<u32>)]
    family_id: Option<u32>,
    /// Page size for target device [default: auto, fallback: 256]
    #[clap(short, long)]
    page_size: Option<usize>,

    /// Semantic version string for firmware (example "V1.2.3")
    #[clap(short, long)]
    semver: Option<String>,
    /// Description string for firmware
    #[clap(short, long)]
    description: Option<String>,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        let input_uf2 = match self.input_path.extension() {
            Some(ext) => ext == OsStr::new("uf2") || ext == OsStr::new("UF2"),
            None => false, // Treat as binary if no extension
        };

        let output_path = if let Some(path) = self.output_path {
            path
        } else {
            let mut path = self.input_path.clone();

            if !input_uf2 {
                // add extension
                path.set_extension("uf2");
            } else {
                path.set_extension("bin");
            }

            path
        };

        println!("Converting {:?} to {:?}", self.input_path, output_path);

        // Parse the embedded JSON
        let mappings: HashMap<String, usize> =
            serde_json::from_str(FAMILY_PAGE_SIZES).unwrap_or_default();

        // Determine page_size: user-provided > family_id mapping > default
        let page_size = if let Some(ps) = self.page_size {
            println!("Page Size: {} (user-provided)", ps);
            ps
        } else if let Some(family_id) = self.family_id {
            // Convert family_id to hex string (e.g., "0xADA52840")
            let family_id_str = format!("0x{:X}", family_id);
            if let Some(ps) = mappings.get(&family_id_str) {
                println!("Page Size: {} (for Family ID 0x{:X})", ps, family_id);
                *ps
            } else {
                println!("Page Size: {} (default)", 256); // Default fallback (unknown FamilyId)
                256
            }
        } else {
            println!("Page Size: {} (default)", 256); // Default (no FamilyID)
            256
        };

        if input_uf2 {
            uf2_to_bin(self.input_path, output_path, self.family_id)
        } else {
            bin_to_uf2(
                self.input_path,
                output_path,
                self.base_addr,
                self.family_id,
                page_size,
                self.semver,
            )
        }
    }
}

/// Binary to UF2.
fn bin_to_uf2(
    input: PathBuf,
    output: PathBuf,
    base_addr: u32,
    family_id: Option<u32>,
    page_size: usize,
    semver: Option<String>,
) -> anyhow::Result<()> {
    let binary = std::fs::read(&input)?;

    let mut uf2_file = Uf2File::new();

    // Use add_binary with page_size and optional semver
    uf2_file.add_binary(&binary, base_addr, family_id, page_size, semver.as_deref())?;

    uf2_file.to_file(&output)?;

    println!(
        "Written {} bytes into {} blocks.",
        binary.len(),
        uf2_file.len()
    );

    Ok(())
}

/// UF2 to binary.
fn uf2_to_bin(input: PathBuf, output: PathBuf, family_id: Option<u32>) -> anyhow::Result<()> {
    let uf2_file = reader::from_bytes(&std::fs::read(&input)?)?;

    let payload = uf2_file
        .get_payload(family_id)
        .ok_or_else(|| Error::msg("No payload found"))?;

    std::fs::write(&output, &payload)?;

    println!(
        "Read {} bytes from {} blocks.",
        payload.len(),
        uf2_file.len()
    );

    Ok(())
}
