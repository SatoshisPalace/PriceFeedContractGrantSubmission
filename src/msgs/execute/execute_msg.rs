use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::post_price::PostPrice;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    PostPrice(PostPrice),
}
