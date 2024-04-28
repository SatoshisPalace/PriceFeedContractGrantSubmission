use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::append_store::AppendStore;

#[derive(Getters, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, AppendStore)]
pub struct PricePosting {
    price: u32,
    time: u64,
}

impl PricePosting {
    pub fn new(price: u32, time: u64) -> Self {
        PricePosting { price, time }
    }
}
