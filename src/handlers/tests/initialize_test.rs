#[cfg(test)]
mod tests {

    use crate::handlers::tests::test_env::tests::TestEnv;

    #[test]
    fn initialize() {
        let mut test_env = TestEnv::new();
        test_env.initialize(&1);
    }

    #[test]
    fn count_correct_on_insantiation() {
        let mut test_env = TestEnv::new();
        let expected_count = 1;
        test_env.initialize(&expected_count);
        test_env.assert_count_is(&expected_count)
    }
}
