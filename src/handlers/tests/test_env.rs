#[cfg(test)]
pub mod tests {
    use crate::{
        contract::instantiate, data::price_posting::PricePosting, handlers::{execute_handler::handle_post_price, query_handlers::{handle_get_most_recent_price, handle_get_prices_by_ids}}, msgs::{execute::commands::post_price::PostPrice, instantiate_msg::InstantiateMsg, query::commands::get_prices_by_ids::GetPricesByIds}, responses::query::query_response::QueryResponse
    };
    use cosmwasm_std::{
        coins, from_binary, testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage}, Addr, ContractInfo, Empty, OwnedDeps
    };
    
    // Test environment struct
    pub struct TestEnv {
        deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        info: cosmwasm_std::MessageInfo,
        env: cosmwasm_std::Env,
    }

    impl TestEnv {
        // Constructor for TestEnv
        pub fn new() -> Self {
            let deps = mock_dependencies();
            let info = mock_info("creator", &coins(1000, "coin"));
            let env = mock_env();
            TestEnv { deps, info, env }
        }

        pub fn set_sender(&mut self, sender: String) {
            self.info = mock_info(&sender.to_owned(), &coins(1000, "coin"));
        }

        // Contract Specific Below

        pub fn initialize(&mut self) {
            let reclaim_contract = ContractInfo {
                address: Addr::unchecked("reclaim address"),
                code_hash: "reclaim code hash".to_owned(),
            };
            let msg = InstantiateMsg { reclaim_contract };
            let _res = instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
                .expect("contract initialization failed");
        }

        pub fn post_price_success(
            &mut self,
            command: PostPrice,
        ) {
            let response = handle_post_price(
                self.deps.as_mut(),
                command,
            );
            response.expect("Failed to post valid Price Proof");
        }
        pub fn post_price_failure(
            &mut self,
            command: PostPrice,
        ) {
            let response = handle_post_price(
                self.deps.as_mut(),
                command,
            );
            assert!(
                response.is_err(),
                "Expected proof posting to fail, but succeded"
            );        
        }

        pub fn get_most_recent_price_success(&mut self, price: PricePosting) {

                let response_result = handle_get_most_recent_price(self.deps.as_ref());
                assert!(
                    response_result.is_ok(),
                    "Expected a PricePosting but failed"
                );

                match from_binary::<QueryResponse>(&response_result.unwrap()) {
                    Ok(claim_response) => match claim_response {
                        QueryResponse::MostRecentPriceResponse(response) => {
                            // Successfully deserialized and matched the Claim variant.
                            // You can now use `claim_data` here.
                            assert_eq!(
                                response.price_posting, price,
                                "Expected Different Price Info"
                            )
                        }
                        _ => assert!(false, "Could not deserialize price response"),
                    },
                    Err(_e) => assert!(false, "Could not deserialize price response"),
                }

        }
        pub fn get_most_recent_price_failure(&mut self) {
            let response_result = handle_get_most_recent_price(self.deps.as_ref());
            assert!(
                response_result.is_err(),
                "Expected a failure, but succeeded"
            );
        }
        pub fn get_prices_by_ids_success(&mut self, prices: Vec<PricePosting>, command: GetPricesByIds) {

            let response_result = handle_get_prices_by_ids(self.deps.as_ref(), command);
            assert!(
                response_result.is_ok(),
                "Expected a vec of PricePosting but failed"
            );

            match from_binary::<QueryResponse>(&response_result.unwrap()) {
                Ok(claim_response) => match claim_response {
                    QueryResponse::PricesByIdsResponse(response) => {
                        // Successfully deserialized and matched the Claim variant.
                        // You can now use `claim_data` here.
                        assert_eq!(
                            response.prices, prices,
                            "Expected Different prices"
                        )
                    }
                    _ => assert!(false, "Could not deserialize prices response"),
                },
                Err(_e) => assert!(false, "Could not deserialize prices response"),
            }
        }
        pub fn get_prices_by_ids_failure(&mut self, command: GetPricesByIds) {
            let response_result = handle_get_prices_by_ids(self.deps.as_ref(), command);
            assert!(
                response_result.is_err(),
                "Expected a failure, but succeeded"
            );
        }
    }
}
