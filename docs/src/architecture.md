# Architecture

`open-bcif` is built on several key layers:

1. **Streaming MessagePack Parser**: Incremental reading of the BCIF hierarchy.
2. **Encoding Chain**: A modular system for applying and reversing transformations like Delta, RunLength, and IntegerPacking.
3. **Command Layer**: CLI subcommands that orchestrate the streaming engine.

## Memory Management

By using a streaming approach, `open-bcif` maintains a constant memory overhead regardless of the input file size.
