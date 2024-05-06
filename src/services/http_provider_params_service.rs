use std::collections::HashSet;

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
    expected_method: &str,
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

pub fn assert_query_params_match(
    http_provider_params_v2: &HttpProviderParamsV2,
    expected_params: &HashSet<(String, String)>,
) -> Result<(), HttpProviderParamsV2Error> {
    let actual_params = http_provider_params_v2.get_query_params_as_set()?;

    if &actual_params != expected_params {
        return Err(HttpProviderParamsV2Error::QueryParamMismatchError {
            expected: expected_params.clone(),
            actual: actual_params,
        });
    }

    Ok(())
}

pub fn assert_query_param_keys_match(
    http_provider_params_v2: &HttpProviderParamsV2,
    expected_keys: &HashSet<String>,
) -> Result<(), HttpProviderParamsV2Error> {
    let actual_keys_list = http_provider_params_v2.get_query_param_keys_list()?;
    let actual_keys_set: HashSet<String> = actual_keys_list.into_iter().collect();

    if &actual_keys_set != expected_keys {
        return Err(HttpProviderParamsV2Error::QueryParamKeyMismatchError {
            expected: expected_keys.clone(),
            actual: actual_keys_set,
        });
    }

    Ok(())
}

pub fn assert_base_url_with_path_matches(
    http_provider_params_v2: &HttpProviderParamsV2,
    expected_base_url_with_path: &str,
) -> Result<(), HttpProviderParamsV2Error> {
    let actual_base_url_with_path = http_provider_params_v2.get_base_url_with_path()?;

    if actual_base_url_with_path != expected_base_url_with_path {
        return Err(HttpProviderParamsV2Error::BaseUrlWithPathMismatchError {
            expected: expected_base_url_with_path.to_string(),
            actual: actual_base_url_with_path,
        });
    }

    Ok(())
}
