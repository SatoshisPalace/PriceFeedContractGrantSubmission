use std::collections::HashSet;

use cosmwasm_std::{from_binary, Binary, StdResult};
use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::http_provider_params_error::HttpProviderParamsV2Error;
// use simd_json::{serde::from_slice, OwnedValue};

#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct HttpProviderParamsV2 {
    method: String,
    responseMatches: Vec<ResponseMatch>,
    responseRedactions: Vec<String>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
pub struct ResponseMatch {
    #[serde(rename = "type")]
    match_type: String,
    value: String,
}

// Implementing from JSON
impl HttpProviderParamsV2 {
    pub fn from_json(parameters: &String) -> StdResult<Self> {
        let binary = Binary::from(parameters.as_bytes());
        from_binary(&binary)
    }

    // Helper method to parse the URL and handle errors
    fn parse_url(&self) -> Result<Url, HttpProviderParamsV2Error> {
        Url::parse(&self.url).map_err(|e| HttpProviderParamsV2Error::UrlParseError {
            error: e.to_string(),
        })
    }

    pub fn get_base_url(&self) -> Result<String, HttpProviderParamsV2Error> {
        let parsed_url = self.parse_url()?;
        Ok(parsed_url.join("/").unwrap().to_string())
    }

    pub fn get_base_url_with_path(&self) -> Result<String, HttpProviderParamsV2Error> {
        let parsed_url = self.parse_url()?;
        Ok(format!(
            "{}://{}{}",
            parsed_url.scheme(),
            parsed_url.host_str().unwrap_or(""),
            parsed_url.path()
        ))
    }

    // Method to get the query parameter keys from the URL as a Vec
    pub fn get_query_param_keys_list(&self) -> Result<Vec<String>, HttpProviderParamsV2Error> {
        let parsed_url = self.parse_url()?;
        let query_keys: Vec<String> = parsed_url
            .query_pairs()
            .map(|(k, _)| k.to_string())
            .collect();
        Ok(query_keys)
    }

    // Method to get the query parameters from the URL as a HashSet of (key, value) pairs
    pub fn get_query_params_as_set(
        &self,
    ) -> Result<HashSet<(String, String)>, HttpProviderParamsV2Error> {
        let parsed_url = self.parse_url()?;
        let query_params: HashSet<(String, String)> = parsed_url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Ok(query_params)
    }
}
