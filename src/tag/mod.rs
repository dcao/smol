mod perceptron;

use tokenize::*;

pub use self::perceptron::*;

/// A part-of-speech tagger.
pub trait Tagger<'a> {
    type Tag;
    // Should this just be anything which can turn into an iterator of these things
    type TagIter: Iterator<Item = (Token<'a>, Self::Tag)>;

    fn tag<I: Iterator<Item = Token<'a>>>(&self, tokens: I) -> Self::TagIter;
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

    pub fn pos(&self, input: &'a str) -> G::TagIter {
        self.tagger.tag(self.tokenizer.tokenize(input))
    }

    pub fn tokenizer(&self) -> &K {
        &self.tokenizer
    }

    pub fn tagger(&self) -> &G {
        &self.tagger
    }
}
