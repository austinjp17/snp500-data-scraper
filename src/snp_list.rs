use reqwest;


async fn make_request(url: &str, _client: reqwest::Client) -> Result<String, reqwest::Error> {
    
    println!("REQUEST");
    // Send the GET request and await the response
    let response = _client.get(url).send().await?;

    let mut json_data = String::new();
    // Check if the response was successful (status code 200-299)
    if response.status().is_success() {
        // Get the JSON data from the response body
        json_data = response.text().await?;
        // println!("API Response: {}", json_data);

    } else {
        // Print an error message if the API call was not successful
        println!("API call failed with status code: {}", response.status());
    }

    Ok(json_data)
}


async fn get_raw_snp() -> Result<String, reqwest::Error> {
    let endpoint = "https://en.wikipedia.org/w/api.php";
    let action = "query";
    let format = "json";
    let prop = "revisions";
    let titles = "List_of_S%26P_500_companies"; // Hardcoded page title
    let format_version = "2";
    let rvprop = "content";
    let rvslots = "*";

    let url = format!(
        "{}?action={}&format={}&prop={}&titles={}&formatversion={}&rvprop={}&rvslots={}",
        endpoint, action, format, prop, titles, format_version, rvprop, rvslots
    );
    // let url:String = String::from("
    // https://en.wikipedia.org/w/api.php?action=query&format=json&prop=revisions&titles=Pet_door&formatversion=2&rvprop=content&rvslots=*");
    println!("URL: {}", url);

    let client = reqwest::Client::new();
    
    let page_data = make_request(&url, client).await?;
    
    Ok(page_data)
}


fn parse_snp(raw_page_data:&str )  {
    use polars_core::prelude::*; //df package
    use wikitext_parser;
    

    let mut snp_data: DataFrame = DataFrame::default();
    println!("{}", raw_page_data);
    // struct wiki_resp {

    // }
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        a: i32,
        b: i32,
    }
    let test_json = String::from("{a:32, b:22}");
    let res_json:Test = serde_json::from_str(&test_json).expect("Error Parsing");
    
    // let res = wikitext_parser::parse_wikitext(
    //     raw_page_data, 
    //     String::from("asdf"), 
    // |err| {
    //     println!{"Err parse {:?}", err};
    // });
    
}

pub async fn get_snp_list() -> Result<(), reqwest::Error> {
    let raw_data_res = get_raw_snp().await;
    let mut raw_data = String::new();
    match raw_data_res {
        Ok(val) => {
            raw_data = val;
        }
        Err(e) => {
            panic!("Error");
        }
    }

    parse_snp(&raw_data);

    Ok(())
}