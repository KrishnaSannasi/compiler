use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<E, P = Initial> {
    err: E,
    cause: Option<P>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Initial {}

impl<E, P> From<E> for Error<E, P> {
    fn from(err: E) -> Self {
        Self { err, cause: None }
    }
}

impl<E> Error<E, Initial> {
    pub fn initial(err: E) -> Self {
        Self { err, cause: None }
    }
}

impl<E, P> Error<E, P> {
    pub fn new(err: E) -> Self {
        Self { err, cause: None }
    }

    pub fn with_context<T>(self, err: T) -> Error<T, Self> {
        Error {
            err,
            cause: Some(self),
        }
    }

    pub fn err(&self) -> &E {
        &self.err
    }

    pub fn cause(&self) -> Option<&P> {
        self.cause.as_ref()
    }
}

impl<E: fmt::Display> fmt::Display for Error<E, Initial> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
