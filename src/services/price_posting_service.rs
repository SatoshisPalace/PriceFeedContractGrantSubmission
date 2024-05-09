use std::collections::HashMap;

use cosmwasm_std::{Decimal, Storage};
use sp_secret_toolkit::{macros::identifiable::Identifiable, reclaim::data::claim::Proof};

use crate::{
    constants::{EXPECTED_BASE_URL_WITH_PATH, EXPECTED_METHOD, EXPECTED_QUERY_PARAMS},
    data::{most_recent_price_posting::MOST_RECENT_PRICE_POSTING, price_posting::PricePosting},
    error::price_posting_error::PricePostingError, models::price_data::PriceData,
};

use sp_secret_toolkit::reclaim::{services::http_provider_params_service::{
    assert_base_url_with_path_matches, assert_method_matches,
    assert_query_param_keys_and_values_match}, data::http_provider_params_v2::HttpProviderParamsV2,

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

    // Convert EXPECTED_QUERY_PARAMS to a HashMap
    let expected_params: HashMap<String, String> = EXPECTED_QUERY_PARAMS
        .iter()
        .map(|&(k, v)| (k.to_string(), v.to_string()))
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


fn parse_price_posting(
    proof: &Proof,
    params: &HttpProviderParamsV2,
) -> Result<PricePosting, PricePostingError> {
    let time = proof.signedClaim().claim().timestampS();
    let price = parse_price(params)?;
    Ok(PricePosting::new(price, *time))
}

fn parse_price(params: &HttpProviderParamsV2) -> Result<Decimal, PricePostingError> {
    for match_data in params.responseMatches().iter() {
        // Handle parsing error with specific error type
        let price_data_vec = PriceData::from_json(match_data.value()).map_err(|err| {
            PricePostingError::PriceTypeError {
                error: format!("Failed to parse {}: {}", match_data.value(), err),
            }
        })?;

        if let Some(price_data) = price_data_vec.get(1) {
            return Ok(*price_data.close());
        }
    }

    Err(PricePostingError::PriceNotFound)
}
