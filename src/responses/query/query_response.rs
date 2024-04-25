use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::count_response::CountResponse;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    CountResponse(CountResponse),
}
