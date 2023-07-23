use reqwest;

pub async fn make_request(url: &str, _client: reqwest::Client) -> Result<String, reqwest::Error> {
    
    let response = _client.get(url).send().await?;

    let mut json_data = String::new();
    // Check if the response was successful (status code 200-299)
    if response.status().is_success() {
        // Get the JSON data from the response body
        json_data = response.text().await?;

    } else {
        println!("API call failed with status code: {}", response.status());
        Err()
    }

    Ok(json_data)
}