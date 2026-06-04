# Quick Start

Get up and running with `open-bcif` in 5 minutes.

## 1. Installation

The easiest way to install is via `cargo`:

```bash
cargo install open-bcif
```

Alternatively, you can download pre-built binaries from the [Releases](https://github.com/elkins/open-bcif/releases) page.

## 2. Basic Validation

Check if a BinaryCIF file is valid:

```bash
open-bcif validate structure.bcif
```

## 3. Extracting a Single Block

If you have a large file with multiple structures and only want the first one:

```bash
open-bcif split all_entries.bcif --output-dir ./parts
# This will create block_0_....bcif in the ./parts directory
```

## 4. Converting to Text CIF

Need to see the coordinates in a text editor?

```bash
open-bcif convert structure.bcif --output structure.cif --format cif
```
