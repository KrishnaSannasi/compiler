use std::fmt;

pub trait WithContext<E> {
    type Output;

    fn with_context(self, err: E) -> Self::Output;
}

impl<N, E, P> WithContext<N> for Error<E, P> {
    type Output = Error<N, Self>;

    #[inline]
    fn with_context(self, err: N) -> Self::Output {
        Error {
            err,
            cause: Some(self),
        }
    }
}

impl<T, N, E, P> WithContext<N> for Result<T, Error<E, P>> {
    type Output = Result<T, Error<N, Error<E, P>>>;

    #[inline]
    fn with_context(self, err: N) -> Self::Output {
        match self {
            Ok(value) => Ok(value),
            Err(cause) => Err(Error {
                err,
                cause: Some(cause),
            }),
        }
    }
}

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
