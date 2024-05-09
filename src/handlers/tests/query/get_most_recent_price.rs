#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::Decimal;
    use sp_secret_toolkit::reclaim::data::claim::{
        ClaimInfo, CompleteClaimData, Proof, SignedClaim,
    };

    use crate::{
        data::price_posting::PricePosting,
        handlers::tests::{
            constants::{
                CLEAN_VALID_PARAMETERS, CLEAN_VALID_PARAMETERS2, CLEAN_VALID_PARAMETERS3, VALID_CONTEXT, VALID_EPOCH, VALID_IDENTIFIER, VALID_OWNER, VALID_PROVIDER, VALID_SIGNATURES, VALID_TIMESTAMP, VALID_TIMESTAMP2, VALID_TIMESTAMP3
            },
            test_env::tests::TestEnv,
        },
        msgs::execute::commands::post_price::PostPrice,
    };

    //////TESTS////////
    #[test]
    fn get_most_recent_price_posted_one_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let claim_info = ClaimInfo::new(
            VALID_PROVIDER.to_string(),
            CLEAN_VALID_PARAMETERS.to_string(),
            VALID_CONTEXT.to_string(),
        );
        let complete_claim_data = CompleteClaimData::new(
            VALID_IDENTIFIER.to_string(),
            VALID_OWNER.to_string(),
            VALID_EPOCH,
            VALID_TIMESTAMP,
        );
        let signatures = vec![VALID_SIGNATURES.to_string()];
        let signed_claim = SignedClaim::new(complete_claim_data, signatures);
        let proof = Proof::new(claim_info, signed_claim);

        let command = PostPrice { proof };
        test_env.post_price_success(command);

        let price = PricePosting::new(Decimal::from_str("58205.46").unwrap(), 1715037458);
        test_env.get_most_recent_price_success(price);
    }

    #[test]
    fn get_most_recent_price_posted_multiple_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        
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

        let command = PostPrice { proof };
        let command2 = PostPrice { proof: proof2 };
        let command3 = PostPrice { proof: proof3 };

        test_env.post_price_success(command);
        test_env.post_price_success(command2);
        test_env.post_price_success(command3);


        let price = PricePosting::new(Decimal::from_str("58207.46").unwrap(), 1715037478);
        test_env.get_most_recent_price_success(price);
    }

    #[test]
    fn get_most_recent_price_posted_fail_none_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        test_env.get_most_recent_price_failure();
    }

}
