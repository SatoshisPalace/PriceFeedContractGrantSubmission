#[cfg(test)]
mod tests {

    use crate::handlers::tests::test_env::tests::TestEnv;

    #[test]
    fn initialize() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
    }

}