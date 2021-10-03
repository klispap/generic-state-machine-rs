use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, StateMachineError>;

#[derive(Error, Debug)]
pub enum StateMachineError {
    #[error("current state ($1) does not accept event $2")]
    UnknownNextState,

    #[error("State machine has no current state")]
    UnknownCurrState,
}
