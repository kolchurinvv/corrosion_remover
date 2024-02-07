use std::{boxed, collections::HashMap, error::Error};

#[derive(Hash, Eq, PartialEq, Clone)]
enum Lang {
    #[allow(non_camel_case_types)]
    en_US,
    #[allow(non_camel_case_types)]
    ru_RU,
}
// trait Inheritence {
//     fn set_parent(&mut self, parent: Category);
//     fn has_parent(&self) -> Result<Option<&Category>, Box<dyn Error>>;
// }

#[derive(Clone)]
pub struct Category {
    pub existing_url: String,
    pub url: String,
    localizations: HashMap<Lang, String>,
    pub subcategories: Option<HashMap<String, Category>>,
    pub parent: Option<Box<Category>>,
}

impl Category {
    fn new(
        existing_url: String,
        url: String,
        localizations: Option<HashMap<Lang, String>>,
        subcategories: Option<HashMap<String, Category>>,
        // parent: Option<Box<Category>>,
        parent: Option<&Category>,
    ) -> Category {
        Category {
            url,
            existing_url,
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Кровля"));
                locale.insert(Lang::en_US, String::from("Roofing"));
                locale
            },
            subcategories: subcategories.clone(),
            parent: parent.cloned().map(Box::new),
        }
    }
    fn set_parent(&mut self, parent: Category) {
        self.parent = Some(Box::new(parent))
    }
    pub fn has_parent(&self) -> Result<Option<&Category>, Box<dyn Error>> {
        if let Some(ref parent) = self.parent {
            Ok(Some(parent.as_ref()))
        } else {
            Err("No parent category".into())
        }
    }
    pub fn print_full_url(&self) {
        let full_url_parts: Vec<&str> = self.collect_url_parts().collect();
        let full_url = full_url_parts.join("/");
        println!("Full url is: {}", full_url);
    }
    fn collect_url_parts<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        if let Some(parent) = &self.parent {
            Box::new(
                parent
                    .collect_url_parts()
                    .chain(std::iter::once(self.url.as_str())),
            )
        } else {
            Box::new(std::iter::once(self.url.as_str()))
        }
    }
}

// trait Printable {
//     fn print(&self);
// }
//
// impl Printable for Category {
//     fn print(&self) {
//         println!("Full url: {}", self.url);
//     }
// }

// pub fn add_category<'a>(
//     category_map: &'a mut HashMap<String, Category>,
//     url_path: &str,
//     name: &str,
// ) -> Result<&'a mut HashMap<String, Category>, Box<dyn Error>> {
//     let parts: Vec<&str> = url_path.split("/").collect();
//     let mut current_map = category_map;
//
//     for (i, part) in parts.iter().enumerate() {
//         if i < parts.len() - 1 {
//             let existing_category = current_map.entry(part.to_string()).or_insert(Category {
//                 existing_url: "/".to_string(),
//                 url: part.to_string(),
//                 localizations: HashMap::new(),
//                 subcategories: Some(HashMap::new()),
//             });
//
//             current_map = existing_category.subcategories.as_mut().unwrap();
//         } else {
//             let category = current_map.entry(part.to_string()).or_insert(Category {
//                 existing_url: format!("/{}", url_path),
//                 url: part.to_string(),
//                 localizations: HashMap::new(),
//                 subcategories: None,
//             });
//
//             category.localizations.insert(Lang::en_US, name.to_string());
//         }
//     }
//
//     Ok(current_map)
// }
// let _allowed_langs: HashSet<Lang> = Lang::iter().collect();

pub fn create_category_tree(base_url: String) -> Result<HashMap<String, Category>, Box<dyn Error>> {
    let mut categories: HashMap<String, Category> = HashMap::new();

    let categories_to_add = vec![
        Category {
            existing_url: String::from("krovlay"),
            url: String::from("roofing"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Кровля"));
                locale.insert(Lang::en_US, String::from("Roofing"));
                locale
            },
            subcategories: None,
            parent: None,
        },
        Category {
            existing_url: String::from("fasady"),
            url: String::from("facades"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Фасады"));
                locale.insert(Lang::en_US, String::from("Facades"));
                locale
            },
            subcategories: None,
            parent: None,
        },
        Category {
            existing_url: String::from("zabor"),
            url: String::from("fences"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Заборы"));
                locale.insert(Lang::en_US, String::from("Fences"));
                locale
            },
            subcategories: None,
            parent: None,
        },
        Category {
            existing_url: String::from("vodostochnye-sistemy"),
            url: String::from("drainage-systems"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Водосточные Системы"));
                locale.insert(Lang::en_US, String::from("Drainage Systems"));
                locale
            },
            subcategories: None,
            parent: None,
        },
        Category {
            existing_url: String::from("obustroystvo-krovli"),
            url: String::from("roofing-components"),
            localizations: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Обустройство Кровли"));
                locale.insert(Lang::en_US, String::from("Roofing Components"));
                locale
            },
            subcategories: None,
            parent: None,
        },
    ];

    for mut category in categories_to_add {
        if category.url == "roofing".to_string() {
            // println!("Found Roofing in {}", category.url);
            // test adding a subcategory
            let sub_cat = Category::new(
                "blya".to_string(),
                "better".to_string(),
                None,
                None,
                Some(&category),
                // Some(Box::new(category.clone())),
            );
            // sub_cat.set_parent(category.clone());
            let subcategories = category.subcategories.get_or_insert(HashMap::new());
            subcategories.insert(sub_cat.url.clone(), sub_cat);
        }

        categories.insert(category.url.clone(), category);
    }

    Ok(categories)
}
// if let Some(cat) = categories.get("/fences") {
//     let url = format!("{}{}", base_url, cat.existing_url);
//     for (lang, localization) in &cat.localizations {
//         if allowed_langs.contains(lang) {
//             match lang {
//                 Lang::en_US => println!("English localization: {}", localization),
//                 Lang::ru_RU => println!("Russian localization: {}", localization),
//             }
//         }
//     }
//     println!("Full url: {}", url);
// }
