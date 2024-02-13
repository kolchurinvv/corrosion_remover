use std::error::Error;

use scraper::{Html, Selector};

pub async fn fetch_url(url: String) -> Result<Html, Box<dyn Error>> {
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

        let url_parts: Vec<&str> = href
            .trim_start_matches(&base_url)
            .split("/")
            .filter(|part| !part.is_empty())
            .collect();

        for part in &url_parts {
            if categories.contains(part) {
                println!("part: {}", part);
                let root_parent_category = part;
                let prod_name = url_parts.last().expect("last part of the url should exist");
                let sub_cats: Vec<&str> = url_parts
                    .iter()
                    .filter(|x| x != &part && x != &prod_name && x != &root_parent_category)
                    .copied()
                    .collect();

                println!("Sub-categories: {:?}", sub_cats);
                println!("Root category is: {}", root_parent_category);
                println!("Product name is: {}", prod_name);
            }
        }
    }
}

pub fn test_parser(
    href: String,
    base_url: String,
    categories: Vec<&str>,
) -> Result<(Vec<String>, String, String), Box<dyn Error>> {
    let url_parts: Vec<&str> = href
        .trim_start_matches(&base_url)
        .split("/")
        .filter(|part| !part.is_empty())
        .collect();

    let mut sub_cats: Vec<String> = Vec::new();
    let mut prod_name: String = String::new();
    let mut root_parent_category: String = String::new();
    for part in &url_parts {
        if categories.contains(part) {
            println!("part: {}", part);
            root_parent_category = part.to_string();
            prod_name = url_parts
                .last()
                .expect("last part of the url should exist")
                .to_string();
            sub_cats.extend(
                url_parts
                    .iter()
                    .filter(|x| *x != part && **x != prod_name && **x != root_parent_category)
                    .map(|x| x.to_string()),
            );
        }
    }
    Ok((sub_cats, prod_name, root_parent_category))
}
