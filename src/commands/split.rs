use crate::streaming::parser::StreamingParser;
use anyhow::Context;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn split(input_path: &str, output_dir: &str) -> anyhow::Result<()> {
    let file = File::open(input_path).context("Failed to open input file")?;
    let reader = BufReader::new(file);
    let mut parser = StreamingParser::new(reader);

    println!("Splitting BinaryCIF file: {}", input_path);
    let (version, encoder, block_count) = parser.parse_file_metadata()?;

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    for i in 0..block_count {
        let block = parser.next_data_block()?;
        let output_filename = format!("block_{}_{}.bcif", i, sanitize_filename(&block.header));
        let output_path = Path::new(output_dir).join(output_filename);

        println!(
            "  Writing Block {}: {} -> {:?}",
            i, block.header, output_path
        );

        let out_file = File::create(&output_path)?;
        let mut writer = BufWriter::new(out_file);

        // Construct a new single-block BCIF structure
        let single_block_file = crate::streaming::File {
            version: version.clone(),
            encoder: format!("{} (split)", encoder),
            data_blocks: vec![block],
        };

        rmp_serde::encode::write(&mut writer, &single_block_file)?;
    }

    println!(
        "Split complete. Created {} files in {}",
        block_count, output_dir
    );
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_sample_bcif;
    use std::fs;

    #[test]
    fn test_split_functional() {
        let input = "test_split_input.bcif";
        let out_dir = "test_split_dir";
        create_sample_bcif(input).unwrap();

        split(input, out_dir).unwrap();

        let paths: Vec<_> = fs::read_dir(out_dir)
            .unwrap()
            .map(|r| r.unwrap().path())
            .collect();
        assert_eq!(paths.len(), 2);

        // Clean up
        fs::remove_file(input).unwrap();
        fs::remove_dir_all(out_dir).unwrap();
    }
}
