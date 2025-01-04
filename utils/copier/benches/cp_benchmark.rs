use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use copier::Copier;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Cpoyier bench", |b| b.iter(|| {
        let mut copier = Copier::new(black_box(4096));
        let src_file = fs::File::open("../../testfiles/test1.bin").unwrap();
        let des_file = fs::File::create("../../testfiles/des/test1.bin").unwrap();
        copier.copy(src_file, des_file, None, |_, _| {}).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);