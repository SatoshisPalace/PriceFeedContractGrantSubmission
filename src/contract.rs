use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    data::state::State,
    handlers::{
        execute_handler::{handle_increment, handle_set_count},
        query_handlers::handle_get_count_query,
    },
    msgs::{
        execute::execute_msg::ExecuteMsg, instantiate_msg::InstantiateMsg,
        query::query_msg::QueryMsg,
    },
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State::new(msg.count, info.sender.clone());
    state.singleton_save(deps.storage)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment(_) => handle_increment(&mut deps),
        ExecuteMsg::Reset(reset_command) => handle_set_count(&mut deps, reset_command),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount(_) => handle_get_count_query(deps),
    }
}
