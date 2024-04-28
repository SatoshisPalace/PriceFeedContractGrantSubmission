use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::current_count_response::PostPriceResponse;
//Unused

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteResponse {
    PostPriceResponse(PostPriceResponse),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}
