use thiserror::Error;

use super::http_provider_params_error::HttpProviderParamsV2Error;

#[derive(Error, Debug)]
pub enum PricePostingError {
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    HttpProviderParamsV2Error(#[from] HttpProviderParamsV2Error),

    #[error("Price not found or invalid format")]
    PriceNotFound,

    #[error(transparent)]
    JsonParsingError(#[from] simd_json::Error),
}

impl From<PricePostingError> for cosmwasm_std::StdError {
    fn from(error: PricePostingError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("State Error: {}", error.to_string()))
    }
}
