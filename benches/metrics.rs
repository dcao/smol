use criterion::Criterion;
use smol::metrics::*;

fn ukkonen_small(c: &mut Criterion) {
    let s1 = "The quick brown fox jumps over the lazy dog.";
    let s2 = "I try very hard to find realistic test cases.";

    c.bench_function("ukkonen (small input)", |b| b.iter(|| ukkonen(s1, s2, 500)));
}

criterion_group!(metrics, ukkonen_small);
