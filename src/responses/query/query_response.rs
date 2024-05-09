use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::response_types::{most_recent_price::MostRecentPriceResponse, prices_by_ids::PricesByIdsResponse};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    MostRecentPriceResponse(MostRecentPriceResponse),
    PricesByIdsResponse(PricesByIdsResponse),
}
