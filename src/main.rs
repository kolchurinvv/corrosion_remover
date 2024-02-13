use corrosion_remover::data::categories::Category;
use corrosion_remover::{construct_url, seed, test_parser};
use lazy_static::lazy_static;
use reqwest::Error;
use std::collections::HashMap;

lazy_static! {
    static ref SEED: HashMap<String, Box<Category>> = seed().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point

    let _categories: Vec<&str> = SEED.iter().map(|cat| cat.1.existing_url.as_str()).collect();
    let get_values = test_parser("https://spb.metallprofil.ru/shop/catalog/krovlya/metallocherepitsa/supermonterray/metallocherepitsa-mp-supermonterrey-normanmp-pe01801705--320800/".to_string(), _base_url, _categories);
    match get_values {
        Ok((sub_cats, prod_name, root_parent_category)) => {
            println!("sub-categories: {:?}", sub_cats);
            println!("root: {}", root_parent_category);
            println!("product: {}", prod_name);
        }
        Err(e) => println!("error: {}", e),
    }
    Ok(())
}
