pub mod corpus;
pub mod perceptron;

// Re-exports
pub use self::corpus::*;
pub use self::perceptron::*;

use error::*;
use tokenize::*;

/// A part-of-speech tagger.
pub trait Tagger {
    type Tag;

    fn tag<'a>(&mut self, tokens: &[Token<'a>]) -> Result<Vec<(Token<'a>, Self::Tag)>, SmolError>;
}
