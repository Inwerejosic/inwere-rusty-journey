//  After adding the header
use serde_json::Value;
use reqwest::{Client, Error};

/// Async function that fetches JSON with headers
async fn async_call(url: &str) -> Result<Value, Error> {
    let client = Client::new();

    let response = client
        .get(url)
        .header("accept", "application/json")
        // TODO: Replace with actual token if required
        // .header("Authorization", "Bearer <YOUR_TOKEN>")
        .send()
        .await?                // Await the HTTP request
        .json::<Value>()       // Parse as JSON
        .await?;               // Await the parsing

    Ok(response)
}

#[tokio::main]
async fn main() {
    let api_url = "https://sandbox.api.yellowcard.io/business/account";

    match async_call(api_url).await {
        Ok(response) => println!("✅ Response:\n{:#?}", response),
        Err(err) => eprintln!("❌ Error: {}", err),
    }
}

