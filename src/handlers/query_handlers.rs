use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::{
    responses::query::{
        query_response::QueryResponse, response_types::most_recent_price::MostRecentPriceResponse,
    },
    services::price_posting_service::get_most_recent_price_posting,
};

pub fn handle_get_most_recent_price(deps: Deps) -> StdResult<Binary> {
    let most_recent_price_posting = get_most_recent_price_posting(deps.storage)?;
    let data = QueryResponse::MostRecentPriceResponse(MostRecentPriceResponse {
        price_posting: most_recent_price_posting,
    });
    return to_binary(&data);
}
