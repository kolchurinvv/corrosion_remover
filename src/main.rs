mod data;
use std::collections::HashMap;

// mod web;
use crate::data::categories::Category;
use reqwest::Error;

fn find_category_by_url<'a>(map: &'a HashMap<String, Category>, url: &str) -> Option<&'a Category> {
    for cat in map.values() {
        if cat.url == url {
            return Some(cat);
        }
        if let Some(subcategories) = &cat.subcategories {
            if let Some(found_cat) = find_category_by_url(subcategories, url) {
                return Some(found_cat);
            }
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point
                                                                                   // let res = web::scraper::fetch_url(base_url).await?;
                                                                                   // println!("page: {}", res);
    let table_res = data::categories::create_category_tree(base_url);
    match table_res {
        Ok(table) => {
            if let Some(subcategory) = find_category_by_url(&table, "better") {
                subcategory.print_full_url();
                if let Ok(Some(parent)) = subcategory.has_parent() {
                    println!("found parent: {}", parent.url)
                } else {
                    println!("didn't find parent")
                }
            } else {
                println!("not found");
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }

    Ok(())
}
