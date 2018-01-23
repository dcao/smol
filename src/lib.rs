extern crate bincode;
extern crate failure;
extern crate itertools;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod metrics;
pub mod tokenize;
pub mod tag;

#[cfg(test)]
mod tests {}
