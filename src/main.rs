

// API KEY: 8FCG2UU0IWQHWH6G
mod snp_list;
mod helpers;

#[tokio::main]
async fn main() {
    let res = snp_list::get_snp_list().await;
    match res {
        Ok(comp_data) => {
            println!("Shape: {:?}\nColumns: {:?}\nData Types: {:?}\nFirst Row: {:?}",
                comp_data.shape() ,
                comp_data.get_column_names(), 
                comp_data.dtypes(),
                comp_data.get_row(comp_data.shape().0-1).unwrap(),
            )
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
