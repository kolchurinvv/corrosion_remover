// mod data;
// use std::collections::HashMap;
use corrosion_remover::data::categories::Lang;
use corrosion_remover::{construct_url, seed, test_parser};
// mod web;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point

    // let table = seed();

    test_parser("https://spb.metallprofil.ru/shop/catalog/krovlya/metallocherepitsa/supermonterray/metallocherepitsa-mp-supermonterrey-normanmp-pe01801705--320800/".to_string(), _base_url);
    Ok(())
}
