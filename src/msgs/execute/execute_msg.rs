use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{increment::Increment, reset::Reset};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment(Increment),
    Reset(Reset),
}
