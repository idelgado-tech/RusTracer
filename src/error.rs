use std::{error::Error, fmt};

#[derive(Debug)]
pub struct TracerError {
    repr: Repr,
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    // &str is a fat pointer, but &&str is a thin pointer.
    SimpleMessage(ErrorKind, &'static &'static str),
}

/// A list specifying general categories of error.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The matrix is not inversible
    NotInversible,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::NotInversible => "Not Inversible",
        }
    }
}

impl TracerError {
    /// Creates a new error from a known kind of error as well as a    
    /// constant message.
    ///
    /// This function does not allocate.
    #[inline]
    pub(crate) const fn new(kind: ErrorKind, message: &'static &'static str) -> TracerError {
        Self {
            repr: Repr::SimpleMessage(kind, message),
        }
    }

    /// Creates a new error from a known kind of error
    ///
    /// This function does not allocate.
    #[inline]
    pub(crate) const fn new_simple(kind: ErrorKind) -> TracerError {
        Self {
            repr: Repr::Simple(kind),
        }
    }

    /// Returns the corresponding [`ErrorKind`] for this error.
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Simple(kind) => kind,
            Repr::SimpleMessage(kind, _) => kind,
        }
    }
}

impl fmt::Display for TracerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
            Repr::SimpleMessage(_, &msg) => msg.fmt(fmt),
        }
    }
}

impl Error for TracerError {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(..) => self.kind().as_str(),
            Repr::SimpleMessage(_, &msg) => msg,
        }
    }
}
