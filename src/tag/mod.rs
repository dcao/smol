pub mod perceptron;

// Re-exports
pub use self::perceptron::*;

use tokenize::*;

/// A part-of-speech tagger.
pub trait Tagger<'a> {
    type Tag;

    fn tag<I: Iterator<Item = Token<'a>>>(&self, tokens: I) -> Vec<(Token<'a>, Self::Tag)>;
}

pub struct Pipeline<K, G> {
    tokenizer: K,
    tagger: G,
}

impl<'a, K: Tokenizer<'a>, G: Tagger<'a>> Pipeline<K, G> {
    pub fn new(tokenizer: K, tagger: G) -> Pipeline<K, G> {
        Pipeline {
            tokenizer: tokenizer,
            tagger: tagger,
        }
    }

    pub fn pos(&self, input: &'a str) -> Vec<(Token<'a>, G::Tag)> {
        self.tagger.tag(self.tokenizer.tokenize(input))
    }

    pub fn tokenizer(&self) -> &K {
        &self.tokenizer
    }

    pub fn tagger(&self) -> &G {
        &self.tagger
    }
}
