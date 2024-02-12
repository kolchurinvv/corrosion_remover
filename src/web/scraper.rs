use std::collections::HashMap;

use reqwest::Error;
use scraper::{Html, Selector};

use crate::{data::categories::Category, seed};

pub async fn fetch_url(url: String) -> Result<Html, Error> {
    let res = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&res);
    Ok(document)
}

pub fn parse_anchors(doc: Html, base_url: String, categories: Vec<&str>) {
    for a in doc.select(&Selector::parse("a").unwrap()) {
        let href = match a.value().attr("href") {
            Some(href) => href,
            None => continue,
        };

        if href
            .replace(&base_url, "")
            .split("/")
            .any(|part| categories.contains(&part))
        {
            println!("found a match");
        }
    }
}

pub fn test_parser(href: String, base_url: String) {
    let categories: Vec<String> = seed()
        .unwrap()
        .iter()
        .map(|cat| cat.1.existing_url.to_string())
        .collect();

    let url_parts: Vec<&str> = href
        .trim_start_matches(&base_url)
        .split("/")
        .filter(|part| !part.is_empty())
        .collect();

    for part in &url_parts {
        if categories.contains(&part.to_string()) {
            println!("part: {}", part);
            let root_parent_category = &part.to_string();
            let prod_name = url_parts.last().expect("last part of the url should exist");
            let sub_cats: Vec<&str> = url_parts
                .iter()
                .skip_while(|x| x != &part && x != &prod_name && x != &root_parent_category)
                .skip(1)
                .cloned()
                .collect();

            println!("Sub-categories: {:?}", sub_cats);
            println!("Root category is: {}", root_parent_category);
            println!("Product name is: {}", prod_name);
        }
    }
}
// if let Some(cat) = href
//     .replace(&base_url, "")
//     .split("/")
//     .find(|part| categories.contains(&part.to_string()))
// {
//     println!("found a match: {}", cat);
// } else {
//     println!("not found for");
// }
// }
