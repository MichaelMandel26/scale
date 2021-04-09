use criterion::{black_box, criterion_group, criterion_main, Criterion};

use scale;

fn bench_directory(c: &mut Criterion) {
    c.bench_function("get_dir_size", |b| {
        b.iter(|| scale::get_dir_size(black_box("../"), false))
    });
}

fn bench_file(c: &mut Criterion) {
    let path = "../Cargo.toml".to_string();
    c.bench_function("get_file_size", |b| {
        b.iter(|| scale::get_file_size(black_box(&path)))
    });
}

criterion_group!(benches, bench_directory, bench_file);
criterion_main!(benches);
