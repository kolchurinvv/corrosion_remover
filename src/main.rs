use reqwest::Error;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: String = " https://spb.metallprofil.ru".to_string();
    let res = reqwest::get(url).await?.text().await?;
    // println!("{}", res);

    let document = Html::parse_document(&res);
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap_or("No href attribute");
        println!("Found link: {}", link);
    }

    Ok(())
}
