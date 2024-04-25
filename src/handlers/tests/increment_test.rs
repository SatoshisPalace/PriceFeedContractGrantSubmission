#[cfg(test)]
mod tests {

    use crate::handlers::tests::test_env::tests::TestEnv;

    #[test]
    fn count_correct_single_increment() {
        let mut test_env = TestEnv::new();
        let mut expected_count = 1;
        test_env.initialize(&expected_count);

        test_env.increment_success();
        expected_count += 1;

        test_env.assert_count_is(&expected_count)
    }

    #[test]
    fn count_correct_many_increment() {
        let mut test_env = TestEnv::new();
        let mut expected_count = 1;
        test_env.initialize(&expected_count);

        test_env.increment_success();
        test_env.increment_success();
        test_env.increment_success();
        test_env.increment_success();

        expected_count += 4;

        test_env.assert_count_is(&expected_count)
    }

    #[test]
    fn increment_after_set() {
        let mut test_env = TestEnv::new();
        let mut expected_count = 1;
        test_env.initialize(&expected_count);

        expected_count = 42069;
        test_env.set_count_success(&expected_count);

        test_env.increment_success();
        expected_count += 1;

        test_env.assert_count_is(&expected_count)
    }
}
