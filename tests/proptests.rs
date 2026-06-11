use open_bcif::encoding::decoders;
use proptest::prelude::*;
use serde_bytes::ByteBuf;

proptest! {
    #[test]
    fn test_decode_byte_array_does_not_panic(
        data in prop::collection::vec(any::<u8>(), 0..1024),
        data_type in 1..6i32
    ) {
        let byte_buf = ByteBuf::from(data);
        // It's allowed to return an error, but it should not panic.
        let _ = decoders::decode_byte_array(&byte_buf, data_type);
    }
}
