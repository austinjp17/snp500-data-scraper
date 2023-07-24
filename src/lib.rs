use polars_core::prelude::*; //df package
use anyhow::{Result, anyhow};
pub mod request;
mod parser;



/// Gets [List of SnP 500 companies](https://en.wikipedia.org/wiki/List_of_S%26P_500_companies)
/// page data using [MediaWiki's api](https://en.wikipedia.org/wiki/Special:ApiSandbox#action=query&format=json&prop=revisions&titles=List_of_S%26P_500_companies&formatversion=2&rvprop=content&rvslots=*)
/// Returns:
///     ```
///     Results<String, request::Error>
///     ```
async fn get_raw_snp() -> Result<String> {
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

    let page_data = request::basic(&url).await.unwrap();

    Ok(page_data)
}

pub mod group {
    use polars::prelude::GroupBy;

    use super::*;
    /// splits df w/ "industry" col into df's by industry
    pub fn by_sector (data: &DataFrame) -> GroupBy {
        let grouped = data.groupby(["sector"])
            .expect("grouping failure");
        // println!("??{:?}", grouped.groups());
        grouped
    }
}

/// getters continer
pub mod fetcher {
    use super::*;

    /// Scrapes Wiki for SnP data
    pub async fn snp_data() -> Result<DataFrame> {
        let raw_data_res = get_raw_snp().await;
        let mut raw_data = String::new();
        match raw_data_res {
            Ok(val) => {
                raw_data = val;
            }
            Err(e) => {
                return Err(anyhow!(format!("Error fetching: {}",e)));
            }
        }

        let snp_data = parser::parse_snp(&raw_data)
            .expect("Error creating dataframe");
        Ok(snp_data)
    }
}

use tokio::task;
use tokio_test::block_on;
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_raw_data() {
        let res = get_raw_snp().await
            .expect("Error getting raw Data");
        assert!(res.len()>0);
    }

    #[tokio::test]
    async fn parse() {
        let raw_data = get_raw_snp().await
            .expect("Error getting raw data");
        let formatted = parser::parse_snp(&raw_data)
            .expect("Error formatting");

        let num_of_constituants = 503;
        assert!(formatted.shape().0 == num_of_constituants);
        assert_eq!(formatted.get_column_names(), ["symbol", "sector", "industry", "date_added", "cik"])
    }
}
