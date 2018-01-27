//! failure. It happens to everyone.
//!
//! Defines broad types used for errors and failures.
//! Individual modules may also contain failure types of their own.

use failure::{Backtrace, Context, Fail};
use std::fmt;

// pub type Result<T> = ::std::result::Result<T, SmolError>;

#[derive(Debug)]
pub struct SmolError {
    inner: Context<SmolErrorKind>,
}

impl SmolError {
    pub fn kind(&self) -> SmolErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for SmolError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for SmolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<SmolErrorKind> for SmolError {
    fn from(kind: SmolErrorKind) -> SmolError {
        SmolError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<SmolErrorKind>> for SmolError {
    fn from(inner: Context<SmolErrorKind>) -> SmolError {
        SmolError { inner: inner }
    }
}

// TODO:
// Depending on how many errors there are, we might restructure them as follows:
// SmolErrorKind has variants TagError, TokenizeError, etc.
// TagError has Io(io::Error), Deserialize, Serialize, EmptyModel, etc.
// TokenizeError has its own errors.
//
// downsides: complexity, duplicate errors (both TagError and TokenizeError could have Io errors)
// upsides: easy to tell where an error originated and its type
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum SmolErrorKind {
    #[fail(display = "Couldn't deserialize a data structure.")] Deserialize,
    #[fail(display = "Can't use an empty model.")] EmptyModel,
    #[fail(display = "Couldn't serialize a data structure.")] Serialize,
    #[fail(display = "Error occurred while tagging.")] Write,
    #[fail(display = "A miscellaneous error ocurred")] Other,
}
