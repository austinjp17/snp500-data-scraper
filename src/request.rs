use reqwest;
use anyhow::{Result, anyhow};

pub async fn basic(url: &str) -> Result<String> {
    
    // println!("REQUEST");
    // Send the GET request and await the response
    let _client = reqwest::Client::new();
    let response = _client.get(url).send().await?;

    let mut json_data = String::new();
    // Check if the response was successful (status code 200-299)
    if response.status().is_success() {
        // Get the JSON data from the response body
        json_data = response.text().await?;
        // println!("API Response: {}", json_data);

    } else {
        // Print an error message if the API call was not successful
        let message = format!("API call failed with status code: {}", response.status());
        return Err(anyhow!(message));
    }

    Ok(json_data)
}