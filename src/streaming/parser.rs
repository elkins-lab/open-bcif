use std::io::Read;
use crate::streaming::{DataBlock, Category, Column};
use rmp::decode;
use anyhow::Context;

pub struct StreamingParser<R: Read> {
    reader: R,
}

pub struct DataBlockHeader {
    pub header: String,
    pub category_count: u32,
}

pub struct CategoryHeader {
    pub name: String,
    pub column_count: u32,
    pub row_count: u32,
}

impl<R: Read> StreamingParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn parse_file_metadata(&mut self) -> anyhow::Result<(String, String, u32)> {
        let _root_map_len = decode::read_map_len(&mut self.reader).context("Failed to read root map")?;
        
        let mut version = String::new();
        let mut encoder = String::new();
        let mut block_count = 0;

        for _ in 0..3 {
            let key = self.read_string()?;
            match key.as_str() {
                "version" => version = self.read_string()?,
                "encoder" => encoder = self.read_string()?,
                "dataBlocks" => block_count = decode::read_array_len(&mut self.reader)?,
                _ => self.skip_value()?,
            }
        }

        Ok((version, encoder, block_count))
    }

    /// Yields just the header of the next DataBlock and prepares for category iteration.
    pub fn next_data_block_header(&mut self) -> anyhow::Result<DataBlockHeader> {
        let _block_map_len = decode::read_map_len(&mut self.reader)?;
        let mut header = String::new();
        
        for _ in 0..2 {
            let key = self.read_string()?;
            match key.as_str() {
                "header" => header = self.read_string()?,
                "categories" => {
                    let category_count = decode::read_array_len(&mut self.reader)?;
                    return Ok(DataBlockHeader { header, category_count });
                }
                _ => self.skip_value()?,
            }
        }
        anyhow::bail!("DataBlock incomplete")
    }

    /// Yields just the header of the next Category and prepares for column iteration.
    /// In BinaryCIF, the 'columns' key MUST be the last key in the category map
    /// for efficient granular streaming.
    pub fn next_category_header(&mut self) -> anyhow::Result<CategoryHeader> {
        let cat_map_len = decode::read_map_len(&mut self.reader)?;
        let mut name = String::new();
        let mut row_count = 0;
        let mut column_count = 0;

        for i in 0..cat_map_len {
            let key = self.read_string()?;
            match key.as_str() {
                "name" => name = self.read_string()?,
                "rowCount" => row_count = decode::read_int::<u32, _>(&mut self.reader)?,
                "columns" => {
                    column_count = decode::read_array_len(&mut self.reader)?;
                    
                    // VALIDATION: 'columns' must be the last key to allow streaming its elements.
                    if i != cat_map_len - 1 {
                        anyhow::bail!("Streaming Error: 'columns' is not the last key in category '{}'. Full block loading required.", name);
                    }
                    
                    return Ok(CategoryHeader { name, row_count, column_count });
                }
                _ => self.skip_value()?,
            }
        }
        anyhow::bail!("Category incomplete: 'columns' key not found")
    }

    /// Reads the next column fully. Since individual columns are small compared to total categories,
    /// this is the reasonable "leaf" of our streaming.
    pub fn next_column(&mut self) -> anyhow::Result<Column> {
        let column: Column = rmp_serde::from_read(&mut self.reader)?;
        Ok(column)
    }

    /// LEGACY: Maintained for compatibility during refactor, but uses internal streaming.
    pub fn next_data_block(&mut self) -> anyhow::Result<DataBlock> {
        let header_info = self.next_data_block_header()?;
        let mut categories = Vec::with_capacity(header_info.category_count as usize);
        for _ in 0..header_info.category_count {
            let cat_header = self.next_category_header()?;
            let mut columns = Vec::with_capacity(cat_header.column_count as usize);
            for _ in 0..cat_header.column_count {
                columns.push(self.next_column()?);
            }
            categories.push(Category {
                name: cat_header.name,
                row_count: cat_header.row_count,
                columns,
            });
        }
        Ok(DataBlock { header: header_info.header, categories })
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        let len = decode::read_str_len(&mut self.reader)?;
        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    fn skip_value(&mut self) -> anyhow::Result<()> {
        let _ = rmpv::decode::read_value(&mut self.reader)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_sample_bcif;
    use std::io::BufReader;
    use std::fs::File;

    #[test]
    fn test_granular_streaming() {
        let path = "test_granular.bcif";
        create_sample_bcif(path).unwrap();
        
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut parser = StreamingParser::new(reader);
        
        let (_, _, block_count) = parser.parse_file_metadata().unwrap();
        assert_eq!(block_count, 2);
        
        let block_header = parser.next_data_block_header().unwrap();
        assert_eq!(block_header.header, "TEST_BLOCK_1");
        
        let cat_header = parser.next_category_header().unwrap();
        assert_eq!(cat_header.name, "_test_category");
        assert_eq!(cat_header.column_count, 1);
        
        let col = parser.next_column().unwrap();
        assert_eq!(col.name, "id");
        
        std::fs::remove_file(path).unwrap();
    }
}
