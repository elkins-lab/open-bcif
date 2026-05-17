# open-bcif: The Infrastructure Bridge

`open-bcif` is a high-performance, streaming-capable toolkit for manipulating and validating BinaryCIF (BCIF) files. Designed for structural biology data at scale, it provides a "Swiss Army Knife" CLI that can handle GB-scale files efficiently without loading them entirely into memory.

## Features

- **Validate**: Check the structural integrity and encoding of BCIF files.
- **Split**: Break large BCIF files into smaller chunks by DataBlock or Category.
- **Merge**: Combine multiple BCIF files into a single optimized stream.
- **Convert**: High-speed conversion between BCIF and other formats (e.g., text CIF).
- **Streaming Engine**: Built on a reactive streaming architecture to minimize memory footprint.

## Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed:

```bash
git clone https://github.com/your-repo/open-bcif.git
cd open-bcif
cargo install --path .
```

## Usage

### Validate a File
```bash
open-bcif validate structure.bcif
```

### Split a File
```bash
open-bcif split large_entry.bcif --output-dir ./parts
```

### Merge Files
```bash
open-bcif merge part1.bcif part2.bcif --output merged.bcif
```

## Architecture

`open-bcif` is written in Rust for maximum performance and memory safety. It utilizes a custom streaming MessagePack parser to traverse the BCIF hierarchy (`DataBlock` -> `Category` -> `Column`) incrementally.

For more details, see the [Documentation Site](docs/README.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
