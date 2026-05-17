use std::fs::File;
use std::io::BufReader;
use crate::streaming::parser::StreamingParser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

pub fn validate(input_path: &str) -> anyhow::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut parser = StreamingParser::new(reader);

    println!("Parsing BinaryCIF file: {}", input_path);
    let (version, encoder, block_count) = parser.parse_file_metadata()?;

    println!("File version: {}", version);
    println!("Encoder: {}", encoder);
    println!("Data blocks: {}", block_count);

    let pb = ProgressBar::new(block_count as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
        .progress_chars("#>-"));

    for i in 0..block_count {
        pb.set_message(format!("Block {}", i));
        let block = parser.next_data_block()?;
        
        // Parallelize validation of categories within a block
        block.categories.par_iter().try_for_each(|category| -> anyhow::Result<()> {
            category.columns.par_iter().try_for_each(|column| -> anyhow::Result<()> {
                use crate::encoding::Encoding;
                if let Some(Encoding::ByteArray { data_type }) = column.data.encoding.first() {
                    let _ = crate::encoding::decoders::decode_byte_array(&column.data.data, *data_type)?;
                }
                Ok(())
            })
        })?;
        
        pb.inc(1);
    }

    pb.finish_with_message("Validation complete");
    println!("Validation successful.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_sample_bcif;
    use std::fs;

    #[test]
    fn test_validate_functional() {
        let path = "test_validate.bcif";
        create_sample_bcif(path).unwrap();
        
        let res = validate(path);
        assert!(res.is_ok());
        
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_validate_large() {
        let path = "test_validate_large.bcif";
        crate::test_utils::create_large_bcif(path, 100).unwrap();
        
        let res = validate(path);
        assert!(res.is_ok());
        
        fs::remove_file(path).unwrap();
    }
}

