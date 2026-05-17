pub mod parser;

use serde::{Deserialize, Serialize};
use crate::encoding::EncodedData;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub version: String,
    pub encoder: String,
    pub data_blocks: Vec<DataBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBlock {
    pub header: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub row_count: u32,
    pub columns: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub name: String,
    pub data: EncodedData,
    pub mask: Option<EncodedData>,
}

