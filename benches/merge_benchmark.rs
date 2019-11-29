use timeseries_merge_benchmark::{run_heap, run_vec};
use criterion::{Criterion, black_box, criterion_group, criterion_main};


fn bench_merges(c: &mut Criterion) {
    c.bench(
        "HeapVecComparison",
        criterion::ParameterizedBenchmark::new("heap", |b, i| b.iter(|| run_heap(black_box(*i), black_box(10000))), 1..40)
        .with_function(
            "vec",
            |b, i| {
                b.iter(|| {
                    run_vec(black_box(*i), black_box(10000))
                })
            },
        ),
    );
}

criterion_group!(benches, bench_merges);
criterion_main!(benches);
