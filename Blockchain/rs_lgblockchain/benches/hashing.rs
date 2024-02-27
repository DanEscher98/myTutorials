use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lgblockchain::utils;
use sha2::{Digest, Sha256};

fn calculate_hash(data: &[u8]) -> Vec<u8> {
    Sha256::digest(data).as_slice().to_vec()
}

// old version
fn hash2binary(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = "hello world".as_bytes(); // or any other data you want to hash
    let hash = hex::encode(calculate_hash(data));

    c.bench_function("hash2binary-for_loop", |b| {
        b.iter(|| hash2binary(black_box(hash.as_bytes())))
    });

    c.bench_function("hash2binary-map_iter", |b| {
        b.iter(|| utils::hash2binary(black_box(hash.as_bytes())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
