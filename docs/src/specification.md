# BinaryCIF Specification

BinaryCIF is a binary serialization of CIF data using MessagePack.

## Data Hierarchy

- **File**: Root object.
- **DataBlock**: A logical grouping (e.g., a single structure).
- **Category**: A table of data (e.g., `_atom_site`).
- **Column**: A single attribute with its associated data and encoding.

## Supported Encodings

- `ByteArray`
- `Delta`
- `RunLength`
- `FixedPoint`
- `IntegerPacking`
- `IntervalQuantization`
- `StringArray`
