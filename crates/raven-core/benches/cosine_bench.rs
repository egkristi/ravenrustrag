use criterion::{criterion_group, criterion_main, Criterion};
use raven_core::cosine_similarity;

fn bench_cosine_128(c: &mut Criterion) {
    let a: Vec<f32> = (0..128).map(|i| (i as f32) * 0.01).collect();
    let b: Vec<f32> = (0..128).map(|i| ((i + 1) as f32) * 0.01).collect();

    c.bench_function("cosine_128d", |bench| {
        bench.iter(|| cosine_similarity(&a, &b));
    });
}

fn bench_cosine_768(c: &mut Criterion) {
    let a: Vec<f32> = (0..768).map(|i| (i as f32) * 0.001).collect();
    let b: Vec<f32> = (0..768).map(|i| ((i + 1) as f32) * 0.001).collect();

    c.bench_function("cosine_768d", |bench| {
        bench.iter(|| cosine_similarity(&a, &b));
    });
}

fn bench_cosine_1536(c: &mut Criterion) {
    let a: Vec<f32> = (0..1536).map(|i| (i as f32) * 0.001).collect();
    let b: Vec<f32> = (0..1536).map(|i| ((i + 1) as f32) * 0.001).collect();

    c.bench_function("cosine_1536d", |bench| {
        bench.iter(|| cosine_similarity(&a, &b));
    });
}

criterion_group!(
    benches,
    bench_cosine_128,
    bench_cosine_768,
    bench_cosine_1536
);
criterion_main!(benches);
