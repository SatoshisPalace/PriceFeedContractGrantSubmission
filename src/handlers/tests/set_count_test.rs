#[cfg(test)]
mod tests {

    use crate::handlers::tests::test_env::tests::TestEnv;

    #[test]
    fn set_count_after_iniitalization() {
        let mut test_env = TestEnv::new();
        let mut expected_count = 1;
        test_env.initialize(&expected_count);

        expected_count = 42069;
        test_env.set_count_success(&expected_count);
        test_env.assert_count_is(&expected_count);
    }

    #[test]
    fn set_count_after_increment() {
        let mut test_env = TestEnv::new();
        let mut expected_count = 1;
        test_env.initialize(&expected_count);

        test_env.increment_success();

        expected_count = 42069;
        test_env.set_count_success(&expected_count);
        test_env.assert_count_is(&expected_count)
    }
}
