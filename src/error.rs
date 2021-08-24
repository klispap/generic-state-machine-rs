use std::fmt::Debug;
use thiserror::Error;

pub type Result<S> = std::result::Result<S, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("State Machine is not initialized")]
    NoCurrentState,

    #[error("Initial state can only be set once.")]
    InitialStateDoubleSet,

    #[error("Access mutex error: {0}")]
    AccessMutex(String),

    #[error("Current state does not have transition to the incoming event: {0}")]
    EventNotMachingState(String),
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Error::AccessMutex(e.to_string())
    }
}

impl<S: Debug, E: Debug> From<ContexError<S, E>> for Error {
    fn from(e: ContexError<S, E>) -> Self {
        Error::EventNotMachingState(e.to_string())
    }
}

#[derive(Error, Debug)]
pub enum ContexError<S: Debug, E: Debug> {
    #[error("State {0:?} does not have transsition for event {1:?}")]
    EventNotMachingState(S, E),
}
