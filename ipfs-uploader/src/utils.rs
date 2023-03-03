use crate::errors::{ApiError, Error};

static BASE_URL: &'static str = "https://api.pinata.cloud";


// Ensure that the keys arent empty bro 👀
pub(crate) fn validate_keys(api_key: &str, secret_key: &str) -> Result<(), Error> {
    if api_key.is_empty() {
        return Err(Error::Api(ApiError::InvalidApiKey));
    }

    if secret_key.is_empty() {
        return Err(Error::Api(ApiError::InvalidSecretKey));
    }

    Ok(())
}

pub(crate) fn api_url(endpoint: &str) -> String {
    format!("{}/{}", BASE_URL, endpoint)
}
