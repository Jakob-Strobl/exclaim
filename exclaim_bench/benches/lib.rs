use criterion::{
    Criterion, 
    black_box
};
mod lexer_benches;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn bench_test(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_test
);

criterion::criterion_main!(benches);