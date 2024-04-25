use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::{
    msgs::execute::reset::Reset,
    responses::execute::current_count_response::CurrentCountResponse,
    services::state_service::{increment_counter, set_counter},
};

pub fn handle_increment(deps: &mut DepsMut) -> StdResult<Response> {
    let current_count = increment_counter(deps.storage)?;
    let data = CurrentCountResponse { current_count };
    Ok(Response::default().set_data(data))
}

pub fn handle_set_count(deps: &mut DepsMut, reset_msg: Reset) -> StdResult<Response> {
    let current_count = set_counter(deps.storage, &reset_msg.count)?;
    let data = CurrentCountResponse { current_count };
    Ok(Response::default().set_data(data))
}
