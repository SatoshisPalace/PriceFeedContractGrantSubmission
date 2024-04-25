use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::current_count_response::CurrentCountResponse;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteResponse {
    CurrentCountResponse(CurrentCountResponse),
}
