use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::reclaim::data::claim::Proof;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PostPrice {
    pub proof: Proof,
}
