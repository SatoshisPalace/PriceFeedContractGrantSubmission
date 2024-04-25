use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::get_count::GetCount;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCount(GetCount),
}
