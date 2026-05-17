use std::fs::File;
use std::io::BufReader;
use crate::streaming::parser::StreamingParser;

pub fn validate(input_path: &str) -> anyhow::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut parser = StreamingParser::new(reader);

    println!("Parsing BinaryCIF file: {}", input_path);
    let (version, encoder, block_count) = parser.parse_file_metadata()?;

    println!("File version: {}", version);
    println!("Encoder: {}", encoder);
    println!("Data blocks: {}", block_count);

    for i in 0..block_count {
        let block = parser.next_data_block()?;
        println!("  Block {}: {}", i, block.header);
        for category in block.categories {
            println!("    Category: {} ({} rows)", category.name, category.row_count);
            for column in category.columns {
                println!("      Column: {}", column.name);
                // Simple validation: try to decode the byte array if it's the first encoding
                use crate::encoding::Encoding;
                if let Some(Encoding::ByteArray { data_type }) = column.data.encoding.first() {
                    let _ = crate::encoding::decoders::decode_byte_array(&column.data.data, *data_type)?;
                }
            }
        }
    }

    println!("Validation successful.");
    Ok(())
}
