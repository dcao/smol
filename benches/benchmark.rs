#[macro_use]
extern crate criterion;
extern crate smol;

mod metrics;
mod tokenize;

criterion_main! {
    metrics::metrics,
    tokenize::tokenize
}
