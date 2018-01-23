//! Tokenizers which act on the sentence level.

// Smol note to self:
// Having a sentence tokenizer which returns lists of lists of words can be done like:
// ```rust
// let t = SentenceTokenizer;
// let w = WordTokenizer;
// t.tokenize(str).map(|x| w.tokenize(x.term).collect()).collect()

// trait SentenceTokenizer {} ?