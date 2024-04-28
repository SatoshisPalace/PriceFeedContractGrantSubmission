use cosmwasm_std::Storage;
use sp_secret_toolkit::reclaim::data::claim::Proof;

use crate::{
    data::price_posting::PricePosting, error::price_posting_error::PricePostingError,
    models::http_provider_params_v2::HttpProviderParamsV2,
};

use super::http_provider_params_service::{assert_method_matches, assert_url_matches};

pub fn add_price_posting(
    storage: &mut dyn Storage,
    proof: &Proof,
) -> Result<(), PricePostingError> {
    validate_params(proof)?;
    let price_posting: PricePosting = parse_price_posting(proof)?;
    price_posting.append_store_push(storage, None)?;
    Ok(())
}

pub fn validate_params(proof: &Proof) -> Result<(), PricePostingError> {
    let params = HttpProviderParamsV2::from_json(proof.claimInfo().parameters())?;
    assert_method_matches(&params, &"GET".to_string())?;
    assert_url_matches(
        &params,
        &"https://api.coingecko.com/api/v3/simple/price?vs_currencies=usd&ids=bitcoin".to_string(),
    )?;
    Ok(())
}

pub fn get_most_recent_price_posting(
    storage: &dyn Storage,
) -> Result<PricePosting, PricePostingError> {
    let most_recent_price_posting: PricePosting = PricePosting::append_store_peek(storage, None)?;
    Ok(most_recent_price_posting)
}

fn parse_price(proof: &Proof) -> Result<u32, PricePostingError> {
    let params: HttpProviderParamsV2 =
        HttpProviderParamsV2::from_json(proof.claimInfo().parameters())?;

    // Iterate over the responseMatches and attempt to parse each value as u32
    for match_data in params.responseMatches().iter() {
        if let Ok(price) = match_data.value().parse::<u32>() {
            return Ok(price);
        }
    }

    // If no parsable number is found, return an error
    Err(PricePostingError::PriceNotFound)
}

fn parse_price_posting(proof: &Proof) -> Result<PricePosting, PricePostingError> {
    let time = proof.signedClaim().claim().timestampS();
    let price = parse_price(proof)?;
    Ok(PricePosting::new(price, *time))
}
