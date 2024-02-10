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
