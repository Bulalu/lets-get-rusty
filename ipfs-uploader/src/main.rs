use  std::env;
use reqwest::Client;
use std::error::Error;

// let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
// let pinata_secret_key = env::var("PINATA_API_SECRET").unwrap();
// println!("{:?}", pinata_api_key)

fn main() {
    // println!("Hello, world!");
    get_image_binary();

}


#[tokio::main]
async fn get_image_binary() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client.get("https://art.orbofi.com/sep-40197.webp").send().await?;

    println!("Status: {}", res.status());

    Ok(())

}