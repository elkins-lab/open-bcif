use open_bcif::encoding::decoders;

#[test]
fn test_complex_decoding_chain() {
    // Chain: Delta -> RunLength -> ByteArray(Int8)
    // Data (RLE): (Value=1, Count=2), (Value=2, Count=1) -> [1, 1, 2]
    // Delta (Origin=10): [11, 12, 14]
    
    let raw_data = vec![1, 2, 2, 1]; // RLE encoded [1, 1, 2]
    
    // 1. Decode ByteArray
    let byte_array = decoders::decode_byte_array(&raw_data, 1).unwrap();
    assert_eq!(byte_array, vec![1.0, 2.0, 2.0, 1.0]);
    
    // 2. Decode RunLength
    let rle_decoded = decoders::decode_run_length(byte_array, 3);
    assert_eq!(rle_decoded, vec![1.0, 1.0, 2.0]);
    
    // 3. Decode Delta
    let delta_decoded = decoders::decode_delta(rle_decoded, 10);
    assert_eq!(delta_decoded, vec![11.0, 12.0, 14.0]);
}

#[test]
fn test_fixed_point_integer_packing_chain() {
    // Chain: FixedPoint -> IntegerPacking -> ByteArray
    // Data: [1000, 2000] (FixedPoint factor 100) -> [10.0, 20.0]
    
    let raw_data = vec![232i32 as u8, 3, 0, 0, 208i32 as u8, 7, 0, 0]; // [1000, 2000] as i32 LE
    
    // 1. Decode ByteArray (Int32)
    let byte_array = decoders::decode_byte_array(&raw_data, 3).unwrap();
    
    // 2. Decode FixedPoint
    let fp_decoded = decoders::decode_fixed_point(byte_array, 100.0);
    assert_eq!(fp_decoded, vec![10.0, 20.0]);
}
