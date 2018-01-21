//! Tokenizers which work on the word level.

use super::*;

/// A tokenizer which uses regular expressions to split a string into alphabetic and
/// non-alphabetic tokens
pub struct RegexWordPunctTokenizer;

impl<'a> Tokenizer<'a> for RegexWordPunctTokenizer {
    type TokenIter = RegexTokenIter<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        RegexTokenIter::new(input, r"\w+|[^\w\s]+")
    }
}
