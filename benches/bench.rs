use criterion::BenchmarkId;
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, Criterion};
use elias_fano::EliasFano;
use rand::Rng;

fn compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.sample_size(20);
        // We generate array only once
        let mut rng = rand::thread_rng();
        let mut vals: Vec<u64> = Vec::with_capacity(*size);
        for _ in 0..*size {
            vals.push(rng.gen());
        }
        vals.sort_unstable();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut elias_fano = EliasFano::new(*vals.last().unwrap(), vals.len() as u64);
                elias_fano.compress(vals.iter()).expect("to work");
            });
        });
    }
    group.finish();
}

criterion_group!(benches, compression);
criterion_main!(benches);
