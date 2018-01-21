use criterion::Criterion;
use smol::tokenize::*;

static INPUT: &'static str =
    "In addition to conventional static typing, before version 0.4, Rust also supported \
     typestates. The typestate system modeled assertions before and after program statements, \
     through use of a special check statement. Discrepancies could be discovered at compile time, \
     rather than when a program was running, as might be the case with assertions in C or C++ \
     code. The typestate concept was not unique to Rust, as it was first introduced in the \
     language NIL. Typestates were removed because in practice they found little use, though the \
     same functionality can still be achieved with branding patterns.

The style changed between \
     0.2, 0.3 and 0.4. Version 0.2 introduced classes for the first time, with version 0.3 adding \
     a number of features including destructors and polymorphism through the use of interfaces. \
     In Rust 0.4, traits were added as a means to provide inheritance; In January 2014, the \
     editor-in-chief of Dr Dobb's, Andrew Binstock, commented on Rust's chances to become a \
     competitor to C++.";

fn whitespace_tokenizer(c: &mut Criterion) {
    let t = WhitespaceTokenizer;
    c.bench_function("whitespace", |b| b.iter(|| t.tokenize(INPUT).last()));
}

fn regex_word_tokenizer(c: &mut Criterion) {
    let t = RegexWordPunctTokenizer;
    c.bench_function("regex word", |b| b.iter(|| t.tokenize(INPUT).last()));
}

criterion_group!(tokenize, whitespace_tokenizer, regex_word_tokenizer);
