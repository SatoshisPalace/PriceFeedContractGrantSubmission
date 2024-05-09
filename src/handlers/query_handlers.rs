use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::{
    msgs::query::commands::get_prices_by_ids::GetPricesByIds, responses::query::{
        query_response::QueryResponse, response_types::{most_recent_price::MostRecentPriceResponse, prices_by_ids::PricesByIdsResponse},
    }, services::price_posting_service::{get_list_of_price_postings, get_most_recent_price_posting}
};

pub fn handle_get_most_recent_price(deps: Deps) -> StdResult<Binary> {
    let most_recent_price_posting = get_most_recent_price_posting(deps.storage)?;
    let data = QueryResponse::MostRecentPriceResponse(MostRecentPriceResponse {
        price_posting: most_recent_price_posting,
    });
    return to_binary(&data);
}

pub fn handle_get_prices_by_ids(deps: Deps, command: GetPricesByIds) -> StdResult<Binary> {
    let prices = get_list_of_price_postings(deps.storage, command.ids)?;
    let data = QueryResponse::PricesByIdsResponse(PricesByIdsResponse{
        prices
    });
    return to_binary(&data);
}