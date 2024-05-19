use std::collections::HashMap;

use cosmwasm_std::Storage;
use sp_secret_toolkit::{macros::identifiable::Identifiable, reclaim::data::claim::Proof};

use crate::{
    constants::{EXPECTED_BASE_URL_WITH_PATH, EXPECTED_METHOD, EXPECTED_QUERY_PARAMS},
    data::{most_recent_price_posting::MOST_RECENT_PRICE_POSTING, price_posting::PricePosting},
    error::price_posting_error::PricePostingError, models::price_data::PriceInfo,
};

use sp_secret_toolkit::reclaim::{services::http_provider_params_service::{
    assert_base_url_with_path_matches, assert_method_matches,
    assert_query_param_keys_and_values_match}, data::http_provider_params_v2::HttpProviderParamsV2,

};

use super::time_service::convert_timestamp_to_unix;

pub fn add_price_posting(
    storage: &mut dyn Storage,
    proof: &Proof,
) -> Result<(), PricePostingError> {
    let params = HttpProviderParamsV2::from_json(proof.claimInfo().parameters())?;
    validate_params(&params)?;
    let price_postings = parse_price_postings(&params)?;
    
    for price_posting in price_postings {
        price_posting.keymap_save(storage)?;
        MOST_RECENT_PRICE_POSTING.save(storage, &price_posting.id())?;
    }

    Ok(())
}

pub fn validate_params(params: &HttpProviderParamsV2) -> Result<(), PricePostingError> {
    assert_method_matches(params, EXPECTED_METHOD)?;
    assert_base_url_with_path_matches(params, EXPECTED_BASE_URL_WITH_PATH)?;

    // Convert EXPECTED_QUERY_PARAMS to a HashMap
    let expected_params: HashMap<String, String> = EXPECTED_QUERY_PARAMS
        .iter()
        .map(|&(key, value)| (key.to_string(), value.to_string()))
        .collect();
    assert_query_param_keys_and_values_match(params, &expected_params)?;

    Ok(())
}

pub fn get_most_recent_price_posting(
    storage: &dyn Storage,
) -> Result<PricePosting, PricePostingError> {
    let id = MOST_RECENT_PRICE_POSTING.load(storage)?;
    get_price_posting(storage, &id)
}

pub fn get_list_of_price_postings(
    storage: &dyn Storage,
    ids: Vec<u64>,
) -> Result<Vec<PricePosting>, PricePostingError> {
    ids
        .into_iter()
        .map(|id| get_price_posting(storage, &id))
        .collect()
}

fn get_price_posting(
    storage: &dyn Storage, 
    id: &u64
) -> Result<PricePosting, PricePostingError> {
    PricePosting::keymap_get_by_id(storage, id)
        .ok_or(PricePostingError::PriceNotFound)
}

fn parse_price_postings(params: &HttpProviderParamsV2) -> Result<Vec<PricePosting>, PricePostingError> {
    let mut price_postings: Vec<PricePosting> = vec![];
    for match_data in params.responseMatches() {
        // Handle parsing error with specific error type
        let price_info = PriceInfo::from_json(match_data.value()).map_err(|err| {
            PricePostingError::PriceTypeError {
                error: format!("Failed to parse {}: {}", match_data.value(), err),
            }
        })?;

        // Access the first element in the quotes vector and extract the price
        for quote in price_info.quotes() {
            price_postings.push( PricePosting::new(quote.quote().USD().price().clone(), convert_timestamp_to_unix(quote.timestamp()).unwrap()));
        }
    }
    Ok(price_postings)
}
