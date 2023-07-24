use wikitext_parser::TextPiece;
use polars_core::prelude::*; //df package
use anyhow::{Result, anyhow};
use chrono::NaiveDate;


/// Parses raw wiki page data
/// Returns:
///     ```
///     polars_core::prelude::Dataframe()
///     ```
pub fn parse_snp(raw_page_data: &str) -> Result<DataFrame> {
    let raw_page_data = String::from(raw_page_data);

    let mut content = raw_page_data.chars().collect::<Vec<_>>();

    content.drain(0..12);
    content.drain(content.len() - 9..);

    let content_str = content.iter().collect::<String>();

    let res = wikitext_parser::parse_wikitext(&content_str, String::from("asdf"), |err| {
        println! {"Err parse {:?}", err};
    });

    let mut comp_info: Vec<TextPiece> = Vec::new();
    let mut comp_symbol: Vec<TextPiece> = Vec::new();
    res.root_section.list_plain_text(&mut comp_info);
    res.root_section
        .list_double_brace_expressions(&mut comp_symbol);

    // Trim to only table data
    comp_symbol.drain(0..3);
    comp_symbol.drain(comp_symbol.len() - 251..);
    comp_info.drain(0..17);
    comp_info.drain(comp_info.len() - 953..);

    // Strip Whitespace and un-needed lines
    for i in 0..comp_info.len() {
        // Remove beginning/ending whitespace
        match &mut comp_info[i] {
            TextPiece::Text {
                formatting: _,
                text,
            } => {
                // println!("Formatting: {}", formatting);
                *text = text.trim().to_owned();
            }
            _ => {
                println! {"OTHER"};
            }
        }
    }

    // Remove non-table lines
    comp_info.retain(|variant| {
        let disallowed_lines: Vec<&str> = vec![",", "\\n|", ";"];
        let mut keep = true;
        if let TextPiece::Text {
            formatting: _,
            text,
        } = variant
        {
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
        .map(|info| info.replace("||", "|"))
        .collect();

    comp_info = comp_info
        .into_iter()
        .map(|info| info.replace("||", "|"))
        .collect();

    // combine symbol and info
    let comp_data_str: Vec<String> = comp_symbol
        .iter()
        .zip(comp_info.iter())
        .map(|(symbol, info)| match symbol {
            TextPiece::DoubleBraceExpression { tag: _, attributes } => {
                if let Some(first_attr) = attributes.get(0) {
                    if let Some(first_piece) = first_attr.value.pieces.get(0) {
                        match first_piece {
                            TextPiece::Text {
                                formatting: _,
                                text,
                            } => format!("{} {}", text, info),
                            _ => "OTHER".to_string(),
                        }
                    } else {
                        "No pieces".to_string()
                    }
                } else {
                    "No attr".to_string()
                }
            }
            other => format!("Err, {}", other),
        })
        .collect();

    // println!("--- Combined ---");
    // for i in 0..comp_data_str.len() {
    //     if i < 6 {
    //         println!("{:?}", comp_data_str[i])
    //     }
    // }
    // println!("---");

    // println!("INFO LEN: {}", comp_info.len());
    // println!("SYM LEN: {}", comp_symbol.len());
    // println!("Combined LEN: {}", comp_data_str.len());

    // Split into fields
    let mut fields: Vec<Vec<String>> = comp_data_str
        .iter()
        .map(|data| data.split('|').map(|s| s.to_string()).collect())
        .collect();

    // println!("--- Unusual Fields ---");
    // for i in 0..fields.len() {
    //     // if i < 6
    //     if fields[i].len() != 8
    //     {
    //         println!("Len: {:?}, index: {}\nLine:{:?}\n", fields[i].len(), i, fields[i]);
    //     }
    // }
    // println!("---");

    let location_len = 7;
    for i in 0..fields.len() {
        // Check if the vector has at least two elements before popping.
        if i == fields.len() - 1 {
            fields[i].pop();
        } else if fields[i].len() != 7 {
            fields[i].pop();
            fields[i].pop();
        }

        if fields[i].len() == location_len && i != fields.len() - 1 {
            fields[i].remove(3);
        }

        for row in &mut fields {
            for cell in row {
                *cell = cell.trim().replace("\\n", "");
            }
        }
    }

    // println!("--- Fields Normed ---");
    // for i in 0..fields.len() {
    //     // if i < 20
    //     if fields[i].len() != 6
    //     {
    //         println!("Unexpected Len: {:?}", fields[i]);
    //         // println!("{:?}", fields[i][fields[i].len()-1])
    //     }
    // }
    // println!("---");

    let mut symbols = vec![];
    let mut sectors = vec![];
    let mut industries = vec![];
    let mut dates_added_str = vec![];
    let mut ciks = vec![];
    for data_row in fields {
        if data_row.len() != 6 {
            println!("Unexpected len: {}, {:?}", data_row.len(), data_row)
        }
        symbols.push(data_row[0].to_string());
        sectors.push(data_row[1].to_string());
        industries.push(data_row[2].to_string());
        dates_added_str.push(data_row[3].to_string());
        ciks.push(data_row[4].to_string());
    }

    // convert dates_added to Date type
    // 1957-03-04
    let mut dates_added = vec![];
    let fmt = "%Y-%m-%d";
    for i in 0..dates_added_str.len() {
        if let Ok(date_obj) = NaiveDate::parse_from_str(&dates_added_str[i], fmt) {
            dates_added.push(Some(date_obj))
        } else {
            dates_added.push(None)
        }
    }
    // println!("Date Dates: {:?}", dates_added.len());

    let symbols = Series::new("symbol", symbols);
    let sectors = Series::new("sector", sectors);
    let industries = Series::new("industry", industries);
    let dates_added = Series::new("date_added", dates_added);
    let ciks = Series::new("cik", ciks);

    let mut snp_df = DataFrame::new(
        vec![symbols, sectors, industries, dates_added, ciks]
    );

    let mut snp_data: DataFrame = DataFrame::default();
    match snp_df {
        Ok(df) => snp_data = df,
        Err(e) => {
            return Err(anyhow!(format!("Error creating Dataframe: {}",e)));
        }
    }
    Ok(snp_data)
}