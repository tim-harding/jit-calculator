use criterion::{criterion_group, criterion_main, Criterion};
use jit_calculator::Program;

const CODE: &str = "+*-/--**++//";

fn criterion_benchmark(c: &mut Criterion) {
    let p = Program::parse(CODE).unwrap();
    let j = p.jit();
    let input: [f64; 1_000_000] = std::array::from_fn(|i| i as f64);

    c.bench_function(format!("interpret {CODE}").as_str(), |b| {
        b.iter(|| {
            input.map(|i| p.interpret(i));
        })
    });

    c.bench_function(format!("exec {CODE}").as_str(), |b| {
        b.iter(|| {
            input.map(|i| j.exec(i));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
