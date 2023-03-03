use std::collections::HashMap;
use  std::env;
use reqwest::{Client, ClientBuilder, header::HeaderMap, multipart::{Form, Part}, Response};
use dotenv::dotenv;
#[derive(Debug)]


struct UploadedData {
    hash: String,
    size: u64,
    name: String,
    description: String,
}


const PINATA_BASE_URL: &str = "https://api.pinata.cloud/";
const FILE_ENDPOINT: &str = "pinning/pinFileToIPFS";
const JSON_ENDPOINT: &str = "pinning/pinJSONToIPFS";


#[tokio::main]
async fn main() {
    dotenv().ok(); 
    let url = "https://art.orbofi.com/sep-17126.webp";
    // let res = get_image_binary(&url).await;
    // // // println!("Response: {:?}", res.unwrap());
    // // // print_type_of(&res.unwrap());
    // match res {
    //     Ok(value) => {
    //         // println!("Response: {:?}", value);
    //         upload_images_to_ipfs(&value).await;
    //     },
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //     }
    // }

    // pin_json().await;
    // let secret_key = env::var("PINATA_API_SECRET").expect("PINATA_SECRET_KEY must be set");

    test_authentication().await;
    // let api_key = env::var("PINATA_API_KEY").unwrap();
    // let secret_key = env::var("PINATA_API_SECRET").unwrap();

    // println!("{:?}", secret_key);

   

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


async fn get_image_binary(url: &str) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    if res.status().is_success() {
        let content = res.bytes().await?;
        Ok(content)

    } else {
        
        Err(From::from(format!("Request failed with status code: {}", res.status())))
    }
}


async fn upload_images_to_ipfs(binary: &bytes::Bytes)  {
    let client = reqwest::Client::new();
    let pinata_url = format!("{}{}", PINATA_BASE_URL, FILE_ENDPOINT);
    let mut headers = HeaderMap::new();

 
    headers.insert("pinata_api_key", HeaderValue::from_str("ff").unwrap());
    headers.insert("pinata_secret_key", HeaderValue::from_str("dd").unwrap());

    println!("{:?}", &headers);

    // let res = client.post(&pinata_url).header(headers).body(binary).send().await;
    

    let form = reqwest::multipart::Form::new()
    .part("file", reqwest::multipart::Part::bytes(binary.to_vec())
        .file_name("seds.png"))
    .text("description", "file description");

    let res = client.post(&pinata_url).headers(headers).multipart(form).send().await;

    println!("{:?}", res.unwrap());

}

async fn pin_json() {

    let client = reqwest::Client::new();
    let pinata_url = format!("{}{}", PINATA_BASE_URL, JSON_ENDPOINT);
    let mut headers = HeaderMap::new();
    let mut jsonFile = HashMap::new();

    // print pinata url
    println!("{:?}", &pinata_url);

    // make sure the headers are set and they are not empty
   
    headers.insert("pinata_api_key", HeaderValue::from_str("9699375107c2c2636d00").unwrap());
    headers.insert("pinata_secret_key", HeaderValue::from_str("623ce3d1c8334272b03a8023b3dea700891075e18474e8844a56c6d2cdc9956e").unwrap());

    jsonFile.insert("name", "testRust");
    jsonFile.insert("description", "test description rust");

    let res = client.post(&pinata_url).headers(headers).json(&jsonFile).send().await;

    println!("{:?}", res.unwrap());




}

 async fn test_authentication() {

    let api_key = env::var("PINATA_API_KEY").expect("PINATA_API_KEY must be set");
    let secret_key = env::var("PINATA_API_SECRET").expect("PINATA_SECRET_KEY must be set");


    
    let mut headers = HeaderMap::new();
    headers.insert("pinata_api_key", (&api_key).parse().unwrap());
    headers.insert("pinata_secret_key", (& secret_key).parse().unwrap());

    // println!("{:?}", api_key);
    // println!("{:?}", secret_key);

    let client = ClientBuilder::new()
    .default_headers(headers)
    .build()
    .await;


    // // print headers
    println!("{:?}", &headers);
    let response = client.get("https://api.pinata.cloud/data/testAuthentication")
      .send()
      .await;

    println!("{:?}", response);

    
  }