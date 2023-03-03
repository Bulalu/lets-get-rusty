use reqwest::{Client, ClientBuilder, header::HeaderMap, multipart::{Form, Part}, Response};
use serde::{Serialize};
use serde::de::DeserializeOwned;
use errors::Error;

use utils::api_url;


mod utils;

pub struct PinataClient {
    client: Client,
}

impl  PinataClient {

    // We a generic type S to ensure that any argument passed in can be converted into a String
    pub fn new<S: Into<String>>(api_key: S, secret_api_key: S) -> Result<PinataClient, Error> {
        let owned_key = api_key.into();
        let owned_secret = secret_key.into();

        utils::validate_keys(&owned_key, &owned_secret)?;
        
        let mut default_headers = HeaderMap::new();
        default_headers.insert("pinata_api_key", (&owned_key).parse().unwrap());
        default_headers.insert("pinata_api_key", (&owned_key).parse().unwrap());
        
        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(PinataClient { client })   
    }

    /// Test if your credentials are correct.  It returns an error if credentials are invalid.
    pub async fn test_authentication(&self) -> Result<(), ApiError> {
        let response = self.client.get(&api_url("data/testAuthentication")).send().await?;

        self.parse_ok_result(response).await
    }

    /// Pin any JSON serializable object to Pinata IPFS Nodes
    pub async fn pin_json<S>(&self, pin_data: PinByJSON<S>) -> Result<PinnedObject, ApiError> 
    where
        S: Serialize,
    {
        let response = self.client.post(&api_url("/pinning/pinJSONToIPFS")).json(&pin_data).send().await?;

        self.parse_ok_result(response).await
    }

    /// Pin By file
    /// Unpin By Hash
    /// PinataApiError iko kwa internal.rs
    
    async fn parse_ok_result(&self, response: Respose) -> Result<(), ApiError> {
        if response.status().is_success() {
            Ok(())
        } else {
            let error = response.json::<PinataApiError>().await?;
            Err(ApiError::GenericError(error.message))
        }
    }


}





