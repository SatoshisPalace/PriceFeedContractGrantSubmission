pub static EXPECTED_METHOD: &str = "GET";
pub static EXPECTED_BASE_URL_WITH_PATH: &str =
    "https://pro-api.coinmarketcap.com/v3/cryptocurrency/quotes/historical";
pub static EXPECTED_QUERY_PARAMS: &[(&str, &str)] = &[
        ("id", "1"),
        ("convert", "USD"),
        ("interval", "5m"),
        ("count", ""),
        ("time_end", ""),
    ];
pub static DAYS_IN_MONTHS: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
