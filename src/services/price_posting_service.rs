use std::collections::HashSet;

use cosmwasm_std::Storage;
use sp_secret_toolkit::{macros::identifiable::Identifiable, reclaim::data::claim::Proof};

use crate::{
    constants::{EXPECTED_BASE_URL_WITH_PATH, EXPECTED_METHOD, EXPECTED_QUERY_KEYS},
    data::{most_recent_price_posting::MOST_RECENT_PRICE_POSTING, price_posting::PricePosting},
    error::price_posting_error::PricePostingError,
    models::http_provider_params_v2::HttpProviderParamsV2,
};

use super::http_provider_params_service::{
    assert_base_url_with_path_matches, assert_method_matches, assert_query_param_keys_match,
};

pub fn add_price_posting(
    storage: &mut dyn Storage,
    proof: &Proof,
) -> Result<(), PricePostingError> {
    let params = HttpProviderParamsV2::from_json(proof.claimInfo().parameters())?;
    validate_params(&params)?;
    let price_posting = parse_price_posting(proof, &params)?;
    price_posting.keymap_save(storage)?;
    MOST_RECENT_PRICE_POSTING.save(storage, &price_posting.id())?;
    Ok(())
}

pub fn validate_params(params: &HttpProviderParamsV2) -> Result<(), PricePostingError> {
    assert_method_matches(params, EXPECTED_METHOD)?;
    assert_base_url_with_path_matches(params, EXPECTED_BASE_URL_WITH_PATH)?;

    // Convert EXPECTED_QUERY_KEYS array to a HashSet
    let expected_keys_set: HashSet<String> =
        EXPECTED_QUERY_KEYS.iter().map(|&k| k.to_string()).collect();
    assert_query_param_keys_match(params, &expected_keys_set)?;

    Ok(())
}

pub fn get_most_recent_price_posting(
    storage: &dyn Storage,
) -> Result<PricePosting, PricePostingError> {
    let id = MOST_RECENT_PRICE_POSTING.load(storage)?;
    let most_recent_price_posting = PricePosting::keymap_get_by_id(storage, &id).unwrap();
    Ok(most_recent_price_posting)
}

fn parse_price_posting(
    proof: &Proof,
    params: &HttpProviderParamsV2,
) -> Result<PricePosting, PricePostingError> {
    let time = proof.signedClaim().claim().timestampS();
    let price = parse_price(params)?;
    Ok(PricePosting::new(price, *time))
}

fn parse_price(params: &HttpProviderParamsV2) -> Result<u32, PricePostingError> {
    // Iterate over the responseMatches and attempt to parse each value as u32
    for match_data in params.responseMatches().iter() {
        if let Ok(price) = match_data.value().parse::<u32>() {
            return Ok(price);
        }
    }

    // If no parsable number is found, return an error
    Err(PricePostingError::PriceNotFound)
}
