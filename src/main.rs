mod web;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_url: String = "https://spb.metallprofil.ru/shop/catalog".to_string(); // catalog entry point
    let res = web::scraper::fetch_url(base_url).await?;
    println!("page: {}", res);
    Ok(())
}
