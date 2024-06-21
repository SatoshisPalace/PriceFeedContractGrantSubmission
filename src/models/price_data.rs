use std::str::FromStr;

use cosmwasm_std::{Decimal, StdError};
use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
pub struct PriceInfo {
    #[getset(get = "pub")]
    price: Decimal,
}

impl PriceInfo {
    /// Deserialize a JSON string into a PriceInfo struct.
    pub fn from_json(json_str: &str) -> Result<Self, StdError> {
        let cleaned_json = clean_json_string(json_str);
        let temp_str = cleaned_json.as_str();
        let price = Decimal::from_str(temp_str).map_err(|err| {
            StdError::generic_err(format!("Failed to convert price to Decimal: {}", err))
        })?;

        Ok(PriceInfo { price })
    }
}

// Note: Regex is impossible due to upload inability, speak to Kenny or Caitlyn for details
// Function to modify a JSON string to ensure all floating point numbers are treated as strings
fn clean_json_string(input_str: &str) -> String {
    input_str
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.')
        .collect()
}
