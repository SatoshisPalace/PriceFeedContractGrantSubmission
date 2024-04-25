use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use sp_secret_toolkit::macros::singleton::SingletonStorage;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, SingletonStorage)]
pub struct State {
    count: i32,
    owner: Addr,
}

// Implementation block for State
impl State {
    // Constructor method to create a new State
    pub fn new(count: i32, owner: Addr) -> Self {
        State { count, owner }
    }

    // Getter for count
    pub fn get_count(&self) -> i32 {
        self.count
    }

    // Setter for count
    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    // Setter for count
    pub fn increment_count(&mut self) {
        self.count += 1;
    }

    // Getter for owner
    pub fn get_owner(&self) -> &Addr {
        &self.owner
    }

    // Setter for owner - Note: Requires ownership to change, hence consumes the original owner
    pub fn set_owner(&mut self, owner: Addr) {
        self.owner = owner;
    }
}
