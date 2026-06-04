# open-bcif: The Infrastructure Bridge

[![CI](https://github.com/elkins/open-bcif/actions/workflows/rust.yml/badge.svg)](https://github.com/elkins/open-bcif/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Powered_by-Rust-orange.svg)](https://www.rust-lang.org/)

`open-bcif` is a high-performance, streaming-capable toolkit for manipulating and validating BinaryCIF (BCIF) files. Designed for structural biology data at scale, it provides a "Swiss Army Knife" CLI that can handle GB-scale files efficiently without loading them entirely into memory.

---

### 🧪 For Structural Biologists
*   **Modern Data Standards:** As the PDB moves to BinaryCIF, `open-bcif` ensures you can handle GB-scale structural data without the memory overhead of legacy formats.
*   **Scientific Validation:** Includes strict validation of column data types and dictionary compliance.

### ⚙️ For Systems Engineers
*   **Blazing Fast:** Built in Rust for maximum throughput, utilizing zero-copy parsing and MessagePack optimization.
*   **Memory Efficient:** Uses a streaming architecture that can process massive structural biological data on commodity hardware.

---

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
git clone https://github.com/elkins/open-bcif.git
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

For more details, see the [Documentation](https://elkins.github.io/open-bcif/).

## Citation

If you use `open-bcif` in your research, please cite it as:

```bibtex
@software{elkins2026openbcif,
  author = {Elkins, George},
  title = {open-bcif: High-performance streaming toolkit for BinaryCIF},
  year = {2026},
  url = {https://github.com/elkins/open-bcif},
  version = {0.1.0}
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
