pub mod perceptron;

// Re-exports
pub use self::perceptron::*;

use tokenize::*;

/// A part-of-speech tagger.
pub trait Tagger {
    type Tag;

    fn tag(&mut self, tokens: &[Token]) -> Vec<(Token, Self::Tag)>;
}
