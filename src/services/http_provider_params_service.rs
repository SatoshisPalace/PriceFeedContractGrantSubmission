use crate::{
    error::http_provider_params_error::HttpProviderParamsV2Error,
    models::http_provider_params_v2::HttpProviderParamsV2,
};

pub fn assert_url_matches(
    http_provider_params_v2: &HttpProviderParamsV2,
    expected_url: &String,
) -> Result<(), HttpProviderParamsV2Error> {
    let actual_url = http_provider_params_v2.url();

    if actual_url != expected_url {
        return Err(HttpProviderParamsV2Error::UrlMismatchError {
            expected: expected_url.to_string(),
            actual: actual_url.to_string(),
        });
    }

    Ok(())
}

pub fn assert_method_matches(
    http_provider_params_v2: &HttpProviderParamsV2,
    expected_method: &String,
) -> Result<(), HttpProviderParamsV2Error> {
    let actual_method = http_provider_params_v2.method();

    if actual_method != expected_method {
        return Err(HttpProviderParamsV2Error::MethodMismatchError {
            expected: expected_method.to_string(),
            actual: actual_method.to_string(),
        });
    }

    Ok(())
}
