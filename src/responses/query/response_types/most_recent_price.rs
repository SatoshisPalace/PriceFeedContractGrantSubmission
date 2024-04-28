use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::price_posting::PricePosting;

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MostRecentPriceResponse {
    pub price_posting: PricePosting,
}
