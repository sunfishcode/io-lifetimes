use failure::{self, Backtrace, Context, Fail};
use std::fmt;

/// A list enumerating the error categories.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it. It is used with the [`Error`] struct.
///
/// This list is non-exhaustive.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// The lock is already held.
    #[fail(display = "The fd is locked")]
    Locked,
    /// Any error not part of this list.
    #[fail(display = "Generic error.")]
    Other,
}

/// A specialized `Error` type.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    /// Access the [`ErrorKind`] member.
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    pub fn kind(&self) -> &ErrorKind {
        &*self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        let inner = Context::new(kind);
        Error { inner }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
