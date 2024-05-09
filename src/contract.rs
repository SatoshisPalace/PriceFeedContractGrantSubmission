use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use sp_secret_toolkit::reclaim::Reclaim;

use crate::{
    handlers::{execute_handler::handle_post_price, query_handlers::{handle_get_most_recent_price, handle_get_prices_by_ids}},
    msgs::{
        execute::execute_msg::ExecuteMsg, instantiate_msg::InstantiateMsg,
        query::query_msg::QueryMsg,
    },
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    Reclaim::new(&msg.reclaim_contract).singleton_save(deps.storage)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::PostPrice(command) => handle_post_price(deps, command),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMostRecentPrice(_) => handle_get_most_recent_price(deps),
        QueryMsg::GetPricesByIds(command) => handle_get_prices_by_ids(deps, command),
    }
}
