//! Provides tokenizers which act on the character level.
use super::*;

/// An iterator which returns each character as a token.
pub struct CharTokenIter<'a> {
    input: &'a str,
    /// A filtering function which filters certain characters.
    filter: fn(&(usize, (usize, char))) -> bool,
    byte_offset: usize,
    char_offset: usize,
    index: usize,
}

impl<'a> CharTokenIter<'a> {
    pub fn new(input: &'a str, filter: fn(&(usize, (usize, char))) -> bool) -> Self {
        CharTokenIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            index: 0,
        }
    }
}

impl<'a> Iterator for CharTokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;
        for (cidx, (bidx, c)) in self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .filter(&self.filter)
        {
            let char_len = c.len_utf8();
            if cidx - skipped_chars == 0 {
                self.byte_offset += char_len;
                self.char_offset += 1;
                skipped_bytes += char_len;
                skipped_chars += 1;
                continue;
            }

            let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
            let token = Token {
                term: slice.into(),
                offset: self.char_offset,
                index: self.index,
            };
            self.char_offset = self.char_offset + slice.chars().count() + 1;
            self.index += 1;
            self.byte_offset = self.byte_offset + bidx + char_len - skipped_bytes;
            return Some(token);
        }

        if self.byte_offset < self.input.len() {
            let slice = &self.input[self.byte_offset..];
            let token = Token {
                term: slice.into(),
                offset: self.char_offset,
                index: self.index,
            };
            self.byte_offset = self.input.len();
            Some(token)
        } else {
            None
        }
    }
}

/// A tokenizer which finds all occurrences of whitespace.
pub struct WhitespaceTokenizer;

/// A helper function which matches the type signature of the `filter` function in the
/// `CharTokenIter` struct.
#[inline]
fn is_whitespace(input: &(usize, (usize, char))) -> bool {
    let (_, (_, c)) = *input;
    c.is_whitespace()
}

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = CharTokenIter<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        CharTokenIter::new(input, is_whitespace)
    }
}
