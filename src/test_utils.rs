use std::fs::File;
use std::io::BufWriter;
use crate::streaming::{File as BcifFile, DataBlock, Category, Column};
use crate::encoding::{EncodedData, Encoding};
use serde_bytes::ByteBuf;

pub fn create_sample_bcif(path: &str) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let bcif = BcifFile {
        version: "0.1.0".to_string(),
        encoder: "test-generator".to_string(),
        data_blocks: vec![
            DataBlock {
                header: "TEST_BLOCK_1".to_string(),
                categories: vec![
                    Category {
                        name: "_test_category".to_string(),
                        row_count: 3,
                        columns: vec![
                            Column {
                                name: "id".to_string(),
                                data: EncodedData {
                                    encoding: vec![Encoding::ByteArray { data_type: 1 }], // Int8
                                    data: ByteBuf::from(vec![1, 2, 3]),
                                },
                                mask: None,
                            }
                        ],
                    }
                ],
            },
            DataBlock {
                header: "TEST_BLOCK_2".to_string(),
                categories: vec![],
            }
        ],
    };

    rmp_serde::encode::write_named(&mut writer, &bcif)?;
    Ok(())
}
