use sp_secret_toolkit::reclaim::data::claim::{ClaimInfo, CompleteClaimData, Proof, SignedClaim};

use crate::{handlers::tests::constants::{CLEAN_VALID_PARAMETERS, CLEAN_VALID_PARAMETERS2, CLEAN_VALID_PARAMETERS3, VALID_CONTEXT, VALID_EPOCH, VALID_IDENTIFIER, VALID_OWNER, VALID_PROVIDER, VALID_SIGNATURES, VALID_TIMESTAMP, VALID_TIMESTAMP2, VALID_TIMESTAMP3}, msgs::execute::commands::post_price::PostPrice};

pub fn get_price_command(id: u8) -> PostPrice {
    let claim_info = ClaimInfo::new(
        VALID_PROVIDER.to_string(),
        CLEAN_VALID_PARAMETERS.to_string(),
        VALID_CONTEXT.to_string(),
    );
    let claim_info2 = ClaimInfo::new(
        VALID_PROVIDER.to_string(),
        CLEAN_VALID_PARAMETERS2.to_string(),
        VALID_CONTEXT.to_string(),
    );
    let claim_info3 = ClaimInfo::new(
        VALID_PROVIDER.to_string(),
        CLEAN_VALID_PARAMETERS3.to_string(),
        VALID_CONTEXT.to_string(),
    );

    let complete_claim_data = CompleteClaimData::new(
        VALID_IDENTIFIER.to_string(),
        VALID_OWNER.to_string(),
        VALID_EPOCH,
        VALID_TIMESTAMP,
    );
    let complete_claim_data2 = CompleteClaimData::new(
        VALID_IDENTIFIER.to_string(),
        VALID_OWNER.to_string(),
        VALID_EPOCH,
        VALID_TIMESTAMP2,
    );
    let complete_claim_data3 = CompleteClaimData::new(
        VALID_IDENTIFIER.to_string(),
        VALID_OWNER.to_string(),
        VALID_EPOCH,
        VALID_TIMESTAMP3,
    );

    let signatures = vec![VALID_SIGNATURES.to_string()];
    let signed_claim = SignedClaim::new(complete_claim_data, signatures.clone());
    let signed_claim2 = SignedClaim::new(complete_claim_data2, signatures.clone());
    let signed_claim3 = SignedClaim::new(complete_claim_data3, signatures.clone());

    let proof = Proof::new(claim_info, signed_claim);
    let proof2 = Proof::new(claim_info2, signed_claim2);
    let proof3 = Proof::new(claim_info3, signed_claim3);

    if id == 1 {
        return PostPrice { proof };
    } else if id == 2 {
        return PostPrice { proof: proof2 };
    } else {
        return PostPrice { proof: proof3 };
    }

}
