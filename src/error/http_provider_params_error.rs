use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HttpProviderParamsV2Error {
    #[error("URL does not match expected URL: expected {expected}, got {actual}")]
    UrlMismatchError { expected: String, actual: String },

    #[error("Method does not match expected Method: expected {expected}, got {actual}")]
    MethodMismatchError { expected: String, actual: String },
}

impl From<HttpProviderParamsV2Error> for cosmwasm_std::StdError {
    fn from(error: HttpProviderParamsV2Error) -> Self {
        cosmwasm_std::StdError::generic_err(format!("State Error: {}", error.to_string()))
    }
}
