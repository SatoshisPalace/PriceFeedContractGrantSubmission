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
    fn get_prices_by_ids_one_posted_one_not() {
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

        let price = PricePosting::new(Decimal::from_str("66024.62150979548").unwrap(), 1715811300);

        let ids = vec![1715811300, 1715811304];
        let prices = vec![price];
        test_env.get_prices_by_ids_success(prices, ids);
    }

    #[test]
    fn get_prices_by_ids_multiple_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.post_price_success(get_price_command(1));
        test_env.post_price_success(get_price_command(2));
        test_env.post_price_success(get_price_command(3));

        let price = PricePosting::new(Decimal::from_str("66024.62150979548").unwrap(), 1715811300);
        let price2 = PricePosting::new(Decimal::from_str("66044.62150979548").unwrap(), 1715811600);
        let price3 = PricePosting::new(Decimal::from_str("66064.62150979548").unwrap(), 1715811900);

        let ids = vec![1715811300, 1715811600, 1715811900];

        let prices = vec![price, price2, price3];
        test_env.get_prices_by_ids_success(prices, ids);
    }

    #[test]
    fn get_prices_by_ids_fail_none_posted() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let ids = vec![1715037458, 1715037468, 1715037478];

        test_env.get_prices_by_ids_failure(ids);
    }
}
