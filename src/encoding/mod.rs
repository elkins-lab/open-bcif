pub mod decoders;

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedData {
    pub encoding: Vec<Encoding>,
    pub data: ByteBuf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Encoding {
    #[serde(rename = "StringArray")]
    StringArray {
        #[serde(rename = "dataEncoding")]
        data_encoding: Vec<Encoding>,
        #[serde(rename = "stringData")]
        string_data: String,
        #[serde(rename = "offsetEncoding")]
        offset_encoding: Vec<Encoding>,
        offsets: ByteBuf,
    },
    #[serde(rename = "ByteArray")]
    ByteArray {
        #[serde(rename = "type")]
        data_type: i32,
    },
    #[serde(rename = "IntegerPacking")]
    IntegerPacking {
        #[serde(rename = "byteCount")]
        byte_count: i32,
        #[serde(rename = "isUnsigned")]
        is_unsigned: bool,
        #[serde(rename = "srcSize")]
        src_size: i32,
    },
    #[serde(rename = "Delta")]
    Delta {
        #[serde(rename = "origin")]
        origin: i32,
        #[serde(rename = "srcType")]
        src_type: i32,
    },
    #[serde(rename = "RunLength")]
    RunLength {
        #[serde(rename = "srcSize")]
        src_size: i32,
        #[serde(rename = "srcType")]
        src_type: i32,
    },
    #[serde(rename = "FixedPoint")]
    FixedPoint {
        factor: f64,
        #[serde(rename = "srcType")]
        src_type: i32,
    },
    #[serde(rename = "IntervalQuantization")]
    IntervalQuantization {
        min: f64,
        max: f64,
        #[serde(rename = "numSteps")]
        num_steps: i32,
        #[serde(rename = "srcType")]
        src_type: i32,
    },
}

#[allow(dead_code)]
pub enum DataType {
    Int8 = 1,
    Int16 = 2,
    Int32 = 3,
    Uint8 = 4,
    Uint16 = 5,
    Uint32 = 6,
    Float32 = 32,
    Float64 = 33,
}
