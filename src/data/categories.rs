use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Clone)]
enum Lang {
    #[allow(non_camel_case_types)]
    en_US,
    #[allow(non_camel_case_types)]
    ru_RU,
}

struct Category {
    existing_url: String,
    url: String,
    localizations: HashMap<Lang, String>,
    subcategories: Option<HashMap<String, Category>>,
}
pub fn create_category_tree() {
    let allowed_langs: HashSet<Lang> = [Lang::en_US, Lang::ru_RU].iter().cloned().collect();

    let mut categories: HashMap<String, Category> = HashMap::new();

    let categories_to_add = vec![
        Category {
            existing_url: String::from("/krovlay"),
            url: String::from("/roofing"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Кровля"));
                locale.insert(Lang::en_US, String::from("Roofing"));
                locale
            },
            subcategories: None,
        },
        Category {
            existing_url: String::from("/fasady"),
            url: String::from("/facades"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Фасады"));
                locale.insert(Lang::en_US, String::from("Facades"));
                locale
            },
            subcategories: None,
        },
        Category {
            existing_url: String::from("/zabor"),
            url: String::from("/fences"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Заборы"));
                locale.insert(Lang::en_US, String::from("Fences"));
                locale
            },
            subcategories: None,
        },
        Category {
            existing_url: String::from("/vodostochnye-sistemy"),
            url: String::from("/drainage-systems"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Водосточные Системы"));
                locale.insert(Lang::en_US, String::from("Drainage Systems"));
                locale
            },
            subcategories: None,
        },
        Category {
            existing_url: String::from("/obustroystvo-krovli"),
            url: String::from("/roofing-components"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Обустройство Кровли"));
                locale.insert(Lang::en_US, String::from("Roofing Components"));
                locale
            },
            subcategories: None,
        },
    ];

    for category in categories_to_add {
        categories.insert(category.url.clone(), category);
    }

    if let Some(cat) = categories.get("/fences") {
        let url = format!("{}{}", base_url, cat.existing_url);
        for (lang, localization) in &cat.localizations {
            if allowed_langs.contains(lang) {
                match lang {
                    Lang::en_US => println!("English localization: {}", localization),
                    Lang::ru_RU => println!("Russian localization: {}", localization),
                }
            }
        }
        println!("Full url: {}", url);
    }
}
