use cosmwasm_std::to_binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CurrentCountResponse {
    pub current_count: i32,
}

impl From<CurrentCountResponse> for cosmwasm_std::Binary {
    fn from(response: CurrentCountResponse) -> Self {
        to_binary(&response).unwrap()
    }
}
