use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kctf::KctfPow;
use kctf_pow::KctfPow as kctf_pow;

fn kctf_solve(difficulty: u32) -> () {
    let challenge = KctfPow::gen_challenge(difficulty);
    challenge.solve();
}

fn kctf_pow_solve(difficulty: u32) -> () {
    let pow = kctf_pow::new();
    let challenge = pow.generate_challenge(difficulty);
    challenge.solve();
}

fn kctf_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kctf Benchmark Group");
    group
        .sampling_mode(criterion::SamplingMode::Flat)
        .sample_size(20);

    for difficulty in [100, 500, 1000, 1337, 31337].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(difficulty),
            difficulty,
            |b, &difficulty| b.iter(|| kctf_solve(black_box(difficulty))),
        );
    }

    group.finish();
}

fn kctf_pow_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kctf-pow Benchmark Group");
    group
        .sampling_mode(criterion::SamplingMode::Flat)
        .sample_size(20);

    for difficulty in [100, 500, 1000, 1337, 31337].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(difficulty),
            difficulty,
            |b, &difficulty| b.iter(|| kctf_pow_solve(black_box(difficulty))),
        );
    }

    group.finish();
}

criterion_group!(benches, kctf_benchmark, kctf_pow_benchmark);
criterion_main!(benches);
