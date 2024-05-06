use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct PriceData {
    time: u64,
    high: f64,
    low: f64,
    open: f64,
    volumefrom: f64,
    volumeto: f64,
    close: f64,
    conversionType: String,
    conversionSymbol: String,
}

impl PriceData {
    /// Deserialize a JSON string into a vector of PriceData.
    pub fn from_json(json_str: &str) -> Result<Vec<PriceData>, cosmwasm_std::StdError> {
        // Clean the input JSON string by replacing escaped double quotes
        let cleaned_json = json_str.replace("\\\"", "\"");

        // Using Binary for compatibility with potential CosmWasm usage
        let binary = cosmwasm_std::Binary::from(cleaned_json.as_bytes());

        // Attempt to deserialize the binary data into Vec<PriceData>
        cosmwasm_std::from_binary(&binary).map_err(|err| {
            cosmwasm_std::StdError::generic_err(format!("Failed to parse price data: {}", err))
        })
    }
}
