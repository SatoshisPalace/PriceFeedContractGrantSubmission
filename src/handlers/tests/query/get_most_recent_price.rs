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
                CLEAN_VALID_PARAMETERS, VALID_CONTEXT, VALID_EPOCH, VALID_IDENTIFIER, VALID_OWNER,
                VALID_PROVIDER, VALID_SIGNATURES, VALID_TIMESTAMP,
            },
            query::get_price_command::get_price_command,
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

        let price = PricePosting::new(Decimal::from_str("64860.13819338009").unwrap(), 1718827200);
        test_env.get_most_recent_price_success(price);
    }

    #[test]
    fn get_most_recent_price_posted_multiple_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.post_price_success(get_price_command(1));
        test_env.post_price_success(get_price_command(2));
        test_env.post_price_success(get_price_command(3));

        let price = PricePosting::new(Decimal::from_str("64860.13819338009").unwrap(), 1718827800);
        test_env.get_most_recent_price_success(price);
    }

    #[test]
    fn get_most_recent_price_posted_fail_none_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        test_env.get_most_recent_price_failure();
    }
}
