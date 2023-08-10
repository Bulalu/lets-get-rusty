use sarufi::{Sarufi, ApiError};
// use dotenv::dotenv;
use serde_json::{ Value};
use std::{collections::HashMap};

 fn main() {
    test().await?;
  
}

async fn test() -> Result<(),ApiError> {


    let name = "My Rusty Chatbot";
    let description = Some("A rusty chatbot created using Sarufi API");
    let industry = Some("Technology");
    let flow: Option<HashMap<String, Value>> = None;
    let intents: Option<HashMap<String, Vec<String>>> = None;
    let webhook_url = Some("https://example.com/webhook");
    let webhook_trigger_intents: Option<Vec<String>> = None;
    let visible_on_community = Some(true);  

        let bot = api.create_bot(
            name,
            description,
            industry,
            flow,
            intents,
            webhook_url,
            webhook_trigger_intents,
            visible_on_community,
    ).await?;

    println!("{:?}", bot.name);

    Ok(())
    
}