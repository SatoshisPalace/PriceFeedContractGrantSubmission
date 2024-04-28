extern crate serde;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct HttpProviderParamsV2 {
    method: String,
    responseMatches: Vec<ResponseMatch>,
    responseRedactions: Vec<String>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct ResponseMatch {
    #[serde(rename = "type")]
    match_type: String,
    value: String,
}

// Implementing from JSON
impl HttpProviderParamsV2 {
    pub fn from_json(parameters: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(parameters)
    }
}
