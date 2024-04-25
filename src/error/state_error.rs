use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StateError {
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),
}

impl From<StateError> for cosmwasm_std::StdError {
    fn from(error: StateError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("State Error: {}", error.to_string()))
    }
}
