pub const VALID_CONTEXT: &str =
    r#"{"providerHash":"0xb4cdf4867a96a61e52f35d12ee74c03a1c3c91d045362f8e13f42630db6081f1"}"#;

pub const CLEAN_VALID_PARAMETERS: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_VALID_PARAMETERS2: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A05%3A00.000Z"}"#;
pub const CLEAN_VALID_PARAMETERS3: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A10%3A00.000Z"}"#;

pub const VALID_PROVIDER: &str = r#"http"#;
pub const VALID_EPOCH: u64 = 1;
pub const VALID_IDENTIFIER: &str =
    r#"0xa2685a00ea350d75ec3a7f551508a10fb4434cf5d86fc562af58d35e5877467f"#;
pub const VALID_OWNER: &str = r#"0xaa55fb19149a3f173bd6fd1f9d1d716d83c0247c"#;
pub const VALID_TIMESTAMP: u64 = 1718836800;
pub const VALID_TIMESTAMP2: u64 = 1718837100;
pub const VALID_TIMESTAMP3: u64 = 1718837400;

pub const VALID_SIGNATURES: &str = r#"0xea1e634e16d41468063caf59b9b96cd1a83b5f0b2aa896defa1738894ec0986220dfa9338daa6508a38bc097ba17bccf53ce551b89d86b417011360f422e158c1c"#;

pub const CLEAN_INVALID_PARAMETERS_METHOD: &str = r#"{"method": "GTT","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_URL: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coonmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_ID: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=2&convert=USD&interval=5m&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_CONVERT: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USSD&interval=5m&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_INTERVAL: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5h&count=1&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_COUNT: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=2&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_TIMEEND: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&time_ennd=2024-06-19T20%3A00%3A00.000Z"}"#;
pub const CLEAN_INVALID_PARAMETERS_PARAMS_BONUS: &str = r#"{"method": "GET","responseMatches": [{"type": "contains","value": "price\": 64860.13819338009"}],"responseRedactions":[],"url":"https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical?id=1&convert=USD&interval=5m&count=1&color=red&time_end=2024-06-19T20%3A00%3A00.000Z"}"#;
