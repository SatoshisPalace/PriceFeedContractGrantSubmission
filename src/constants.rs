pub static EXPECTED_METHOD: &str = "GET";
pub static EXPECTED_BASE_URL_WITH_PATH: &str =
    "https://min-api.cryptocompare.com/data/v2/histominute";
pub static EXPECTED_QUERY_PARAMS: &[(&str, &str)] = &[
        ("fsym", "BTC"),
        ("tsym", "USD"),
        ("limit", "1"),
        // "toTs" should be included as a key but with a dynamic value
        ("toTs", ""),
    ];