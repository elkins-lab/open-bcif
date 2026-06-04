# Architecture

`open-bcif` is built on several key layers:

1. **Streaming MessagePack Parser**: Incremental reading of the BCIF hierarchy.
2. **Encoding Chain**: A modular system for applying and reversing transformations like Delta, RunLength, and IntegerPacking.
3. **Command Layer**: CLI subcommands that orchestrate the streaming engine.

## Memory Management

By using a streaming approach, `open-bcif` maintains a constant memory overhead regardless of the input file size.

## Key Design Principles

- **Hybrid Parsing**: Uses a custom streaming MessagePack parser to traverse the BCIF hierarchy (`DataBlock` -> `Category` -> `Column`) incrementally.
- **Full Encoding Support**: Supports all standard BCIF encodings, including `ByteArray`, `Delta`, `RunLength`, `FixedPoint`, `IntegerPacking`, `IntervalQuantization`, and `StringArray`.
- **Validation Engine**: Parallelized validation using `rayon` for high-throughput integrity checks.
- **Robust Testing**: Comprehensive test suite including unit tests with parameterized matrices (via `rstest`) and end-to-end integration tests.
