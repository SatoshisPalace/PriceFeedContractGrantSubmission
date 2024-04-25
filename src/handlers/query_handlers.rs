use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::{responses::query::count_response::CountResponse, services::state_service::get_count};

pub fn handle_get_count_query(deps: Deps) -> StdResult<Binary> {
    let count = get_count(deps.storage)?;
    let data = CountResponse { count };
    return to_binary(&data);
}
