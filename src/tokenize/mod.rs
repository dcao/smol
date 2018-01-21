//! Tokenize strings.
//!
//! All these tokenizers currently deal with English only (or at least, they've only been tested
//! with English in mind).
//!
//! Thanks to <http://nitschinger.at/Text-Analysis-in-Rust-Tokenization/> for heavy inspiration.
// TODO: Should input types be Cow?

pub mod chr;
pub mod word;
pub mod regex;

// Re-exports
pub use self::chr::*;
pub use self::word::*;
pub use self::regex::*;

use std::borrow::Cow;

/// Anything which can turn a raw string into an iterator of tokens.
pub trait Tokenizer<'a> {
    /// The iterator which will return the tokens.
    type TokenIter: Iterator<Item = Token<'a>>;

    /// Takes an input string and returns an iterator over its tokens.
    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

/// A single token.
#[derive(Clone)]
pub struct Token<'a> {
    // TODO: Maybe just give start/end offset? Make getting text a method
    /// The term of text of the token itself.
    pub term: Cow<'a, str>,
    /// The starting byte offset of the token in the overall string.
    pub offset: usize,
    /// The index of the token amongst all tokens in the iterator.
    pub index: usize,
}
