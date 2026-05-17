use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_bcif::encoding::decoders;

fn bench_byte_array_decoding(c: &mut Criterion) {
    let data = vec![42u8; 1_000_000]; // 1MB of data
    c.bench_function("decode_byte_array_int32_1mb", |b| {
        b.iter(|| {
            decoders::decode_byte_array(black_box(&data), 3).unwrap()
        })
    });
}

fn bench_integer_packing_decoding(c: &mut Criterion) {
    let data = vec![127u8; 1_000_000]; // 1MB of packed data
    c.bench_function("decode_integer_packing_1mb", |b| {
        b.iter(|| {
            decoders::decode_integer_packing(black_box(&data), 1, false, 1_000_000).unwrap()
        })
    });
}

fn bench_delta_decoding(c: &mut Criterion) {
    let data = vec![1.0f64; 1_000_000];
    c.bench_function("decode_delta_1m_elements", |b| {
        b.iter(|| {
            decoders::decode_delta(black_box(data.clone()), 0)
        })
    });
}

criterion_group!(benches, bench_byte_array_decoding, bench_integer_packing_decoding, bench_delta_decoding);
criterion_main!(benches);
