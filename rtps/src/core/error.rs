use std::{self, fmt::{self, Display}};

use failure::{Backtrace, Context, Fail};

/// Convenient wrapper around `std::Result`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Error type.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind_ref(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        ErrorKind::Io(err).into()
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        ErrorKind::TryFromInt(err).into()
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        ErrorKind::SystemTime(err).into()
    }
}

/// The kind of an error.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)] Io(#[cause] std::io::Error),
    #[fail(display = "{}", _0)] TryFromInt(#[cause] std::num::TryFromIntError),
    #[fail(display = "{}", _0)] SystemTime(#[cause] std::time::SystemTimeError),
}
