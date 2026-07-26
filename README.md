# cargo-uf2

A cargo subcommand for [UF2 file](https://github.com/microsoft/uf2) operations. UF2 (USB Flashing Format) is a file format used for flashing microcontrollers over USB, popularized by devices like the Raspberry Pi Pico.

This crate utilizes the [uftwo crate](https://crates.io/crates/uftwo).

## Features
* Convert Bin to UF2
* Convert UF2 to Bin

## Installation

Install from crates.io:
```bash
cargo install cargo-uf2
```

## Usage

```bash
cargo uf2 <subcommand> [options]
```

## Subcommands

### Convert

Convert between binary and UF2 formats.

```bash
# Binary to UF2
cargo uf2 convert input.bin output.uf2

# UF2 to binary
cargo uf2 convert input.uf2 output.bin

# With options
cargo uf2 convert input.bin output.uf2 \
    -b 0x08000000 \  # Base address
    -f 0xADA52840 \  # Family ID
    -p 256 \         # Page size
    -s "V1.0.0" \    # Semantic version
    -d "My firmware" # Description
```

**Options:**
- `-b, --base-addr <ADDR>`: Target base address in flash memory (default: 0x00)
- `-f, --family-id <ID>`: Family ID for the target device
- `-p, --page-size <SIZE>`: Page size for target device (default: auto, fallback: 256)
- `-s, --semver <VERSION>`: Semantic version string
- `-d, --description <TEXT>`: Description string for firmware

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/mestrode/cargo-uf2).
