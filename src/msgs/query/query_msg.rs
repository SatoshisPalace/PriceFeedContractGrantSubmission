use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::{get_most_recent_price::GetMostRecentPrice, get_prices_by_ids::GetPricesByIds};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetMostRecentPrice(GetMostRecentPrice),
    GetPricesByIds(GetPricesByIds),
}
