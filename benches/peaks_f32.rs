use bikeshedwaveform::peaks::SlicePeakExt;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn bench_peak_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("Peak f32");
    let mut rng = rand::thread_rng();

    let i: &[f32] = &[0; 15].map(|_| rng.gen_range(-50.0..50.0));
    group.bench_with_input(BenchmarkId::new("Small - AVX2", 15), i, |b, i| {
        b.iter(|| SlicePeakExt::peak_avx2(i))
    });
    group.bench_with_input(BenchmarkId::new("Small - Naive", 15), i, |b, i| {
        b.iter(|| SlicePeakExt::peak_naive(i))
    });

    let i: &[f32] = &[0; 400].map(|_| rng.gen_range(-50.0..50.0));
    group.bench_with_input(BenchmarkId::new("Medium - AVX2", 400), i, |b, i| {
        b.iter(|| SlicePeakExt::peak_avx2(i))
    });
    group.bench_with_input(BenchmarkId::new("Medium - Naive", 400), i, |b, i| {
        b.iter(|| SlicePeakExt::peak_naive(i))
    });

    group.finish();
}

criterion_group!(benches, bench_peak_f32);
criterion_main!(benches);
