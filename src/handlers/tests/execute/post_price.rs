#[cfg(test)]
mod tests {
    use sp_secret_toolkit::reclaim::data::claim::{ClaimInfo, CompleteClaimData, Proof, SignedClaim};

    use crate::{handlers::tests::{constants::{CLEAN_INVALID_PARAMETERS_METHOD, CLEAN_INVALID_PARAMETERS_PARAMS, CLEAN_INVALID_PARAMETERS_URL, CLEAN_VALID_PARAMETERS, VALID_CONTEXT, VALID_EPOCH, VALID_IDENTIFIER, VALID_OWNER, VALID_PROVIDER, VALID_SIGNATURES, VALID_TIMESTAMP}, test_env::tests::TestEnv}, msgs::execute::commands::post_price::PostPrice};

    //////TESTS////////
    #[test]
    fn user_posts_valid_proof() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let claim_info = ClaimInfo::new(VALID_PROVIDER.to_string(), CLEAN_VALID_PARAMETERS.to_string(), VALID_CONTEXT.to_string());
        let complete_claim_data = CompleteClaimData::new(VALID_IDENTIFIER.to_string(), VALID_OWNER.to_string(), VALID_EPOCH, VALID_TIMESTAMP);
        let signatures = vec![VALID_SIGNATURES.to_string()];
        let signed_claim = SignedClaim::new(complete_claim_data, signatures);
        let proof = Proof::new(claim_info, signed_claim);

        let command = PostPrice {
            proof
        };
        test_env.post_price_success(command);
    }

    #[test]
    fn user_posts_invalid_proof_method() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let claim_info = ClaimInfo::new(VALID_PROVIDER.to_string(), CLEAN_INVALID_PARAMETERS_METHOD.to_string(), VALID_CONTEXT.to_string());
        let complete_claim_data = CompleteClaimData::new(VALID_IDENTIFIER.to_string(), VALID_OWNER.to_string(), VALID_EPOCH, VALID_TIMESTAMP);
        let signatures = vec![VALID_SIGNATURES.to_string()];
        let signed_claim = SignedClaim::new(complete_claim_data, signatures);
        let proof = Proof::new(claim_info, signed_claim);

        let command = PostPrice {
            proof
        };
        test_env.post_price_failure(command);
    }

    #[test]
    fn user_posts_invalid_proof_params() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let claim_info = ClaimInfo::new(VALID_PROVIDER.to_string(), CLEAN_INVALID_PARAMETERS_PARAMS.to_string(), VALID_CONTEXT.to_string());
        let complete_claim_data = CompleteClaimData::new(VALID_IDENTIFIER.to_string(), VALID_OWNER.to_string(), VALID_EPOCH, VALID_TIMESTAMP);
        let signatures = vec![VALID_SIGNATURES.to_string()];
        let signed_claim = SignedClaim::new(complete_claim_data, signatures);
        let proof = Proof::new(claim_info, signed_claim);

        let command = PostPrice {
            proof
        };
        test_env.post_price_failure(command);
    }

    #[test]
    fn user_posts_invalid_proof_url() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let claim_info = ClaimInfo::new(VALID_PROVIDER.to_string(), CLEAN_INVALID_PARAMETERS_URL.to_string(), VALID_CONTEXT.to_string());
        let complete_claim_data = CompleteClaimData::new(VALID_IDENTIFIER.to_string(), VALID_OWNER.to_string(), VALID_EPOCH, VALID_TIMESTAMP);
        let signatures = vec![VALID_SIGNATURES.to_string()];
        let signed_claim = SignedClaim::new(complete_claim_data, signatures);
        let proof = Proof::new(claim_info, signed_claim);

        let command = PostPrice {
            proof
        };
        test_env.post_price_failure(command);
    }

}