// mod data;
// use std::collections::HashMap;

use corrosion_remover::{construct_url, seed};
// mod web;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point
                                                                                    // let res = web::scraper::fetch_url(base_url).await?;
                                                                                    // println!("page: {}", res);
                                                                                    // let table_res = data::categories::create_category_tree(base_url);
    let table_res = seed();
    // let table_res = data::categories::seed();
    match table_res {
        Ok(table) => {
            if let Some(url) = construct_url("nails", table.get("roofing").expect("REASON")) {
                println!("url: {}", url)
            } else {
                println!("not found url for nails")
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }

    Ok(())
}
