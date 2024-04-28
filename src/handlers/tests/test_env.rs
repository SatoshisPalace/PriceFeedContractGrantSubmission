#[cfg(test)]
pub mod tests {
    use crate::{
        contract::instantiate,
        handlers::{
            execute_handler::{handle_increment, handle_set_count},
            query_handlers::handle_get_most_recent_price,
        },
        msgs::{execute::reset::Reset, instantiate_msg::InstantiateMsg},
        responses::query::count_response::CountResponse,
    };
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps,
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

        pub fn initialize(&mut self, count: &i32) {
            let msg = InstantiateMsg { count: *count };
            let _res = instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
                .expect("contract initialization failed");
        }

        pub fn increment_success(&mut self) {
            let result = handle_increment(&mut self.deps.as_mut());
            assert!(result.is_ok())
        }

        pub fn set_count_success(&mut self, new_count: &i32) {
            let reset_msg = Reset { count: *new_count };
            let result = handle_set_count(&mut self.deps.as_mut(), reset_msg);
            assert!(result.is_ok())
        }

        pub fn assert_count_is(&mut self, expected_count: &i32) {
            let result = handle_get_most_recent_price(self.deps.as_ref());
            assert!(result.is_ok());
            let response: CountResponse = from_binary(&result.unwrap()).unwrap();
            assert_eq!(response.count, *expected_count);
        }
    }
}
