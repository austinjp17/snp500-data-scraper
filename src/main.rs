use std::fmt::format;

// API KEY: 8FCG2UU0IWQHWH6G
use reqwest;
use wikitext_parser::TextPiece;

async fn make_request(url: &str, _client: reqwest::Client) -> Result<String, reqwest::Error> {
    
    // println!("REQUEST");
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
    // println!("URL: {}", url);

    let client = reqwest::Client::new();
    
    let page_data = make_request(&url, client).await?;
    
    Ok(page_data)
}


fn parse_snp(raw_page_data:&str )  {
    use polars_core::prelude::*; //df package
    use wikitext_parser;
    use wikitext_parser::{Headline, TextPiece, Text};
    

    let mut snp_data: DataFrame = DataFrame::default();

    // println!("{}", raw_page_data);
    let raw_page_data = String::from(raw_page_data);

    let c_index = raw_page_data.find("\"content\"").expect("error finding");
    let mut content = raw_page_data.chars().collect::<Vec<_>>();
    // let content_ 

    content.drain(0..12);
    content.drain(content.len()-9..);
    
    // 

    let content_str = content.iter().collect::<String>();
    
    let res = wikitext_parser::parse_wikitext(
        &content_str, 
        String::from("asdf"), 
    |err| {
        println!{"Err parse {:?}", err};
    });
    
    let mut comp_info: Vec<TextPiece> = Vec::new();
    let mut comp_symbol: Vec<TextPiece> = Vec::new();
    res.root_section.list_plain_text(&mut comp_info);
    res.root_section.list_double_brace_expressions(&mut comp_symbol);

    // Trim to only table data
    comp_symbol.drain(0..3);
    comp_symbol.drain(comp_symbol.len()-251..);
    comp_info.drain(0..17);
    comp_info.drain(comp_info.len()-953..);

    // println!("{:?}", out);

    // Strip Whitespace and un-needed lines
for i in 0..comp_info.len() {

        // Remove beginning/ending whitespace
        match &mut comp_info[i] {
            TextPiece::Text { formatting, text } => {
                // println!("Formatting: {}", formatting);
                *text = text.trim().to_owned();
                
            }
            _ => {
                println!{"OTHER"};
            }
        }  
    }

    // Remove non-table lines
    comp_info.retain(|variant| {
        let disallowed_lines: Vec<&str> = vec![",", "\\n|", ";"];
        let mut keep = true;
        if let TextPiece::Text { formatting, text } = variant {
            // println!("{}", text.starts_with('<'));
            
            if text.starts_with('<') || disallowed_lines.contains(&text.as_str()) {
                // println!("FOUND");
                keep = false;
            }   
        }
        keep
    });

    // Combine comp info into single element
    let mut comp_info = comp_info
        .chunks_exact(2)
        .map(|pair| {
        // Concatenate the text fields from the Text variants in the pair
        pair.iter()
            .map(|text_piece| match text_piece {
                TextPiece::Text { text, .. } => text.clone(),
                _ => panic!("Unexpected variant in the vector."),
            })
            .collect::<String>()
            
        })
        .collect::<Vec<String>>();

    comp_info = comp_info
        .into_iter()
        .map(|info| {
            info.replace("||", "|")
        })
        .collect();

    comp_info = comp_info
        .into_iter()
        .map(|info| {
            info.replace("||", "|")
        })
        .collect();

    // let symbols: Vec<String> = comp_symbol
    //     .iter()
    //     .map(|symbol| match symbol {
            
    //     })

    let comp_data_str: Vec<String> = comp_symbol
        .iter()
        .zip(comp_info.iter())
        .map(|(symbol, info)| match symbol {
            TextPiece::DoubleBraceExpression { tag, attributes } => {
                if let Some(first_attr) = attributes.get(0) {
                    if let Some(first_piece) = first_attr.value.pieces.get(0) {
                        match first_piece {
                            TextPiece::Text {formatting, text} => format!("{} {}", text, info),
                            _ => "OTHER".to_string()
                        }
                    } else {
                        "No pieces".to_string()
                    }
                } else {
                    "No attr".to_string()
                }
            }
            other => format!("Err, {}", other)
        })
        .collect();


    for i in 0..comp_info.len() {
        // if i == 2
        if 10 > i
        {
            // println!("{}, {}", comp_symbol[i], comp_info[i]);
            println!("{}", comp_data_str[i])
        }
        
    }
    println!("INFO LEN: {}", comp_info.len());
    println!("SYM LEN: {}", comp_symbol.len());
    println!("Combined LEN: {}", comp_data_str.len());

    struct CompData {
        sector: String,
        industry: String,
        date_added: String, //TODO: Date Obj
        founded: String, //date obj
        cik: i32,
    }


}

async fn get_snp_list() -> Result<(), reqwest::Error> {
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

#[tokio::main]
async fn main() {
    // let client = reqwest::Client::new();
    // let url = "https://jsonplaceholder.typicode.com/posts/1";

    // let url = "https://en.wikipedia.org/wiki/List_of_S%26P_500_companies";
    if let Err(e) = get_snp_list().await {
            println!("{}", e);
        };
    // if let Err(e) = make_request(url, client).await {
    //     println!("{}", e);
    // };
}
