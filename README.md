# 🦀 open-bcif: High-Performance BinaryCIF Toolkit

[![Crates.io](https://img.shields.io/crates/v/open-bcif.svg)](https://crates.io/crates/open-bcif)
[![Docs.rs](https://docs.rs/open-bcif/badge.svg)](https://docs.rs/open-bcif)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Powered_by-Rust-orange.svg)](https://www.rust-lang.org/)

`open-bcif` is a high-performance, streaming-capable toolkit for manipulating and validating BinaryCIF (BCIF) files. Designed for structural biology data at scale.

---

### 🧪 For Structural Biologists
*   **Modern Data Standards:** As the PDB moves to BinaryCIF, `open-bcif` ensures you can handle GB-scale structural data without the memory overhead of legacy formats.
*   **Scientific Validation:** Includes strict validation of column data types and dictionary compliance.

### 🤖 For Systems Engineers
*   **Blazing Fast:** Built in Rust for maximum throughput, utilizing zero-copy parsing and MessagePack optimization.
*   **Memory Efficient:** Uses a streaming architecture that can process massive structural biological data on commodity hardware.

---

## 🚀 Key Features

*   **Zero-Copy Parsing:** Extremely low CPU and memory overhead during deserialization.
*   **MessagePack Core:** Native support for the foundational BinaryCIF encoding.
*   **CLI & Library:** Comprehensive tool for inspecting and repairing corrupted BCIF files.

## 📦 Installation (Rust)

```toml
[dependencies]
open-bcif = "0.1.0"
```

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.
