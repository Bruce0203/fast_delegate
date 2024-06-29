use criterion::Criterion;

fn compare(c: &mut Criterion) {
    c.bench_function("handwritten", |b| {
        b.iter(|| {

        });
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, compare);
