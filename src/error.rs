use std::{error::Error, fmt};

//Refacto this
#[derive(Debug,Clone)]
pub struct RayTracerError {
    repr: Repr,
}

#[derive(Debug,Clone)]
enum Repr {
    Simple(ErrorEnum),
    // &str is a fat pointer, but &&str is a thin pointer.
    SimpleMessage(ErrorEnum, &'static &'static str),
}

/// A list specifying general categories of error.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorEnum {
    /// The matrix is not inversible
    NotInversible,
}

impl ErrorEnum {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorEnum::NotInversible => "Not Inversible",
        }
    }
}

impl RayTracerError {
    /// Creates a new error from a known kind of error as well as a    
    /// constant message.
    ///
    /// This function does not allocate.
    #[inline]
    pub(crate) const fn new(kind: ErrorEnum, message: &'static &'static str) -> RayTracerError {
        Self {
            repr: Repr::SimpleMessage(kind, message),
        }
    }

    /// Creates a new error from a known kind of error
    ///
    /// This function does not allocate.
    #[inline]
    pub(crate) const fn new_simple(kind: ErrorEnum) -> RayTracerError {
        Self {
            repr: Repr::Simple(kind),
        }
    }

    /// Returns the corresponding [`ErrorKind`] for this error.
    #[inline]
    pub fn kind(&self) -> ErrorEnum {
        match self.repr {
            Repr::Simple(kind) => kind,
            Repr::SimpleMessage(kind, _) => kind,
        }
    }
}

impl fmt::Display for RayTracerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
            Repr::SimpleMessage(_, &msg) => msg.fmt(fmt),
        }
    }
}

impl Error for RayTracerError {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(..) => self.kind().as_str(),
            Repr::SimpleMessage(_, &msg) => msg,
        }
    }
}
