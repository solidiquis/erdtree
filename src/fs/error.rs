use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    sync::{Arc, PoisonError},
};

#[derive(Debug)]
pub enum Error {
    OutstandingStrongRefs,
    PoisonedMutex,
    TraversalError,
    UnsafeRcMutAttempt
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutstandingStrongRefs => write!(f, "Outstanding strong refs"),
            Error::PoisonedMutex => write!(f, "Poisoned mutex"),
            Error::TraversalError => write!(f, "No directories traversed"),
            Error::UnsafeRcMutAttempt => write!(f, "Attempt to mutate object with strong reference count greater than 1"),
        }
    }
}

impl StdError for Error {}

impl<T> From<PoisonError<T>> for Error {
    fn from(_e: PoisonError<T>) -> Self {
        Error::PoisonedMutex
    }
}

impl<T> From<Arc<T>> for Error {
    fn from(_e: Arc<T>) -> Self {
        Error::OutstandingStrongRefs
    }
}
