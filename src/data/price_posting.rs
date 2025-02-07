use cosmwasm_std::Decimal;
use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

#[derive(
    Getters, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage,
)]
#[getset(get = "pub")]
pub struct PricePosting {
    price: Decimal,
    time: u64,
}

impl PricePosting {
    pub fn new(price: Decimal, time: u64) -> Self {
        PricePosting { price, time }
    }
}

impl Identifiable for PricePosting {
    type ID = u64; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        self.time
    }
}
