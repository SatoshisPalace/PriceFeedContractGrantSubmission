use crate::{data::state::State, error::state_error::StateError};

pub fn increment_counter(storage: &mut dyn cosmwasm_std::Storage) -> Result<i32, StateError> {
    let mut state = State::singleton_load(storage)?;
    state.increment_count();
    state.singleton_save(storage)?;
    Ok(state.get_count())
}

pub fn set_counter(
    storage: &mut dyn cosmwasm_std::Storage,
    new_count: &i32,
) -> Result<i32, StateError> {
    let mut state = State::singleton_load(storage)?;
    state.set_count(*new_count);
    state.singleton_save(storage)?;
    Ok(state.get_count())
}

pub fn get_count(storage: &dyn cosmwasm_std::Storage) -> Result<i32, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.get_count())
}
