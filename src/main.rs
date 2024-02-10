mod data;
// use std::collections::HashMap;

// mod web;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point
                                                                                    // let res = web::scraper::fetch_url(base_url).await?;
                                                                                    // println!("page: {}", res);
                                                                                    // let table_res = data::categories::create_category_tree(base_url);
    let table_res = data::categories::seed();
    match table_res {
        Ok(table) => {
            // if let Some(subcategory) = find_category_by_url(&table, "better") {
            //     subcategory.print_full_url();
            //     if let Ok(Some(parent)) = subcategory.has_parent() {
            //         println!("found parent: {}", parent.url)
            //     } else {
            //         println!("didn't find parent")
            //     }
            // } else {
            //     println!("not found");
            // }
            if let Some(url) =
                data::categories::construct_url("nails", table.get("roofig").expect("REASON"))
            {
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
