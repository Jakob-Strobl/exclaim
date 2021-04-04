use criterion::Criterion;
mod lexer_benches;

fn bench_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexer");

    group.bench_function("Long String Literal", |b| {
        lexer_benches::bench_string_literal(b)
    });

    group.finish();
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_lexer
);

criterion::criterion_main!(benches);