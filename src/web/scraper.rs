use reqwest::Error;
// use scraper::Html;

pub async fn fetch_url(url: String) -> Result<String, Error> {
    let res = reqwest::get(url).await?.text().await?;
    // let document = Html::parse_document(&res);

    Ok(res)
}
