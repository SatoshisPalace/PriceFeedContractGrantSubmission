use cosmwasm_std::{DepsMut, Response, StdResult};
use sp_secret_toolkit::reclaim::Reclaim;

use crate::{
    msgs::execute::commands::post_price::PostPrice,
    services::price_posting_service::add_price_posting,
};

pub fn handle_post_price(deps: DepsMut, command: PostPrice) -> StdResult<Response> {
    let PostPrice { proof } = command;

    add_price_posting(deps.storage, &proof)?;

    //Verify proof afterwords, all will be reverted if proof fails validation
    let reclaim = Reclaim::singleton_load(deps.storage)?;
    let msg = reclaim.create_verify_proof_msg(&proof)?;

    Ok(Response::default().add_message(msg))
}
