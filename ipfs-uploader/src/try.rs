use std::collections::HashMap;
use  std::env;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};

use serde::{Deserialize, Serialize};
use std::error::Error;

// let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
// let pinata_secret_key = env::var("PINATA_API_SECRET").unwrap();
// println!("{:?}", pinata_api_key)

// i can make the program concurrent that is i can run multiple executions at once
// download the images on a list all at a go

const PINATA_BASE_URL: &str = "https://api.pinata.cloud/";
const FILE_ENDPOINT: &str = "pinning/pinFileToIPFS";

fn main() {

    // let url = "https://art.orbofi.com/sep-40197.webp";
    // let res = get_image_binary(url);
    // println!("Response: {:?}", res);
    let key = upload_images_to_ipfs();
    println!("Response: {:?}", key);


}

#[tokio::main]
async fn get_image_binary(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?;

    if res.status().is_success() {
        let content = res.bytes().await?;
        Ok(content.to_vec())

    } else {
        println!("Request failed with status code: {}", res.status());
        Err(From::from(format!("Request failed with status code: {}", res.status())))
    }
}

fn upload_images_to_ipfs()  {
    let client = Client::new();
    let pinata_url = format!("{}{}", PINATA_BASE_URL, FILE_ENDPOINT);
    let mut headers = HeaderMap::new();

    headers.insert("pinata_api_key", HeaderValue::from_str(&env::var("PINATA_API_KEY").unwrap()).unwrap());
    headers.insert("pinata_secret_key", HeaderValue::from_str(&env::var("PINATA_API_SECRET").unwrap()).unwrap());


    // let res = client.post(&pinata_url);


    println!("{:?}", &headers);
    // println!("{}", pinata_secret_key);


}