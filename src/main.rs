use polars_core::prelude::DataFrame;

// API KEY: 8FCG2UU0IWQHWH6G
mod snp_list;
mod helpers;

#[tokio::main]
async fn main() -> Result<DataFrame, String> {
    let res = snp_list::get_snp_list().await;
    let mut snp_df = DataFrame::default();
    match res {
        Ok(comp_data) => {
            snp_df = comp_data;
        }
        Err(e) => {
            println!("Error: {e}")
        }
    }
    Ok(snp_df)
}
