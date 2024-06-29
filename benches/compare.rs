use criterion::Criterion;

fn compare(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| todo!());
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, compare);
