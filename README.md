# SnP 500 Constituents Info Fetcher

This is a tool that fetches up-to-date information about the constituents of the S&P 500 index from Wikipedia. It retrieves minimal essential details: symbol, sector, industry, date added to S&P 500, and CIK identifier for each company and returns them contained in a dataframe with the companies ordered A-Z. 

## Introduction

The Standard & Poor's 500 Index, commonly known as the S&P 500, is a stock market index that tracks the performance of 500 large publicly traded companies in the United States. This program allows you to retrieve key information about the current constituents of the S&P 500 index from Wikipedia. The goal is to provide an easy short-cut to updated information on the United States most tracked financial index.

## Example

## Example
```
use snp500_data;
use polars_core::prelude::*;

#[tokio::main]
async fn main() {
    let test_df: DataFrame = snp500_data::fetcher::snp_data().await.unwrap();

    // --- Columns ---
    //["symbol", "sector", "industry", "date_added", "cik"] 
    //[ String , String  ,   String  ,  String     ,String]
    
    println!("{:?}", test_df.get_column_names()) 
    println!("{:?}", test_df.get_row(0));
}
```

## Installation

Before running the program, ensure that you have the latest stable version of Rust installed on your system. To build and install the program, use the following command:
```
cargo add snp500_data
```


## Disclaimer

This dataset is intended for general analysis. Please be aware that the data is sourced from Wikipedia, which means there is a possibility of inaccuracies, and there may be a slight lag of a few days.