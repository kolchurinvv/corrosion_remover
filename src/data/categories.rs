use std::{collections::HashMap, error::Error};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Lang {
    #[allow(non_camel_case_types)]
    en_US,
    #[allow(non_camel_case_types)]
    ru_RU,
}
// #[derive(Clone)]
// pub struct Category {
//     pub existing_url: String,
//     pub url: String,
//     localizations: HashMap<Lang, String>,
//     pub subcategories: Vec<Rc<RefCell<Category>>>,
//     pub parent: Option<Rc<RefCell<Category>>>,
// }

// impl Category {
//     pub fn new(
//         existing_url: String,
//         url: String,
//         localizations: HashMap<Lang, String>,
//     ) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Self {
//             existing_url,
//             ur&l,
//             localizations,
//             subcategories: Vec::new(),
//             parent: None,
//         }))
//     }
//
//     pub fn collect_full_url(&self) -> String {
//         let mut parts = Vec::new();
//         parts.push(self.url.as_str());
//         let mut current = self.parent.clone();
//         while let Some(parent) = current {
//             let parent_borrowed: &std::borrow::Cow<Category> = parent.borrow();
//             parts.push(&parent_borrowed.url);
//             current = parent_borrowed.parent.clone();
//         }
//         parts.reverse();
//         parts.join("/")
//     }
// }

#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    existing_url: String,
    local: HashMap<Lang, String>,
    pub subcategories: Vec<Category>,
    pub products: Vec<Product>,
}

impl Category {
    fn new(name: &str, existing_url: &str, local: HashMap<Lang, String>) -> Self {
        Category {
            name: name.to_string(),
            subcategories: Vec::new(),
            products: Vec::new(),
            existing_url: existing_url.to_string(),
            local,
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.subcategories.push(category);
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn find_product_path(&self, product_name: &str) -> Option<Vec<String>> {
        for product in &self.products {
            if product.name == product_name {
                return Some(vec![self.name.clone()]);
            }
        }
        for subcat in &self.subcategories {
            if let Some(mut path) = subcat.find_product_path(product_name) {
                path.insert(0, self.name.clone());
                return Some(path);
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct Product {
    name: String,
}

impl Product {
    fn new(name: &str) -> Self {
        Product {
            name: name.to_string(),
        }
    }
}

pub fn construct_url(product_name: &str, root_category: &Category) -> Option<String> {
    if let Some(path) = root_category.find_product_path(product_name) {
        let path_str = path.join("/");
        Some(format!("{}", path_str))
    } else {
        None
    }
}

pub fn seed() -> Result<HashMap<String, Box<Category>>, Box<dyn Error>> {
    let mut roofing_localizations = HashMap::new();
    roofing_localizations.insert(Lang::ru_RU, "Кровля".to_string());
    roofing_localizations.insert(Lang::en_US, "Roofing".to_string());

    let mut roofing = Category::new("roofing", "krovlya", roofing_localizations);

    let mut subcategory_localizatinos = HashMap::new();
    subcategory_localizatinos.insert(Lang::ru_RU, "Обустройство Кровли".to_string());
    subcategory_localizatinos.insert(Lang::en_US, "Roofing Components".to_string());

    let nails = Product::new("nails");

    let mut roofing_supplies = Category::new(
        "roofing-supplies",
        "obustroystvo-krovli",
        subcategory_localizatinos,
    );

    roofing_supplies.add_product(nails);
    roofing.add_category(roofing_supplies);

    let mut tree: HashMap<String, Box<Category>> = HashMap::new();

    tree.insert("roofing".to_string(), Box::new(roofing));
    Ok(tree)
}

// pub fn create_category_tree() -> Result<VecDeque<Rc<RefCell<Category>>>, Box<dyn Error>> {
//     let mut categories = VecDeque::new();
//
//     let mut roofing_localizations = HashMap::new();
//     roofing_localizations.insert(Lang::ru_RU, "Кровля".to_string());
//     roofing_localizations.insert(Lang::en_US, "Roofing".to_string());
//
//     let roofing = Category::new(
//         "krovlya".to_string(),
//         "roofing".to_string(),
//         roofing_localizations,
//     );
//     Category::add_category(&mut categories, roofing, "");
//     // let roofing = Category::new(
//     //     "krovlya".to_string(),
//     //     "roofing".to_string(),
//     //     roofing_localizations,
//     // );
//
//     let mut subcategory_localizatinos = HashMap::new();
//     subcategory_localizatinos.insert(Lang::ru_RU, "Обустройство Кровли".to_string());
//     subcategory_localizatinos.insert(Lang::en_US, "Roofing Components".to_string());
//
//     let subcategory = Category::new(
//         "obustroystvo-krovli".to_string(),
//         "roofing-components".to_string(),
//         subcategory_localizatinos,
//     );
//     Category::add_category(&mut categories, subcategory, "roofing");
//
//     // categories.push(roofing);
//
//     Ok(categories)
// }

// pub fn create_category_tree_v1(
//     base_url: String,
// ) -> Result<HashMap<String, Category<'static>>, Box<dyn Error>> {
//     let mut categories: HashMap<String, Category> = HashMap::new();
//
//     let categories_to_add = vec![
//         Category {
//             existing_url: String::from("krovlay"),
//             url: String::from("roofing"),
//             localizations: {
//                 let mut locale = HashMap::new();
//                 locale.insert(Lang::ru_RU, String::from("Кровля"));
//                 locale.insert(Lang::en_US, String::from("Roofing"));
//                 locale
//             },
//             subcategories: None,
//             parent: None,
//         },
//         Category {
//             existing_url: String::from("fasady"),
//             url: String::from("facades"),
//             localizations: {
//                 let mut locale = HashMap::new();
//                 locale.insert(Lang::ru_RU, String::from("Фасады"));
//                 locale.insert(Lang::en_US, String::from("Facades"));
//                 locale
//             },
//             subcategories: None,
//             parent: None,
//         },
//         Category {
//             existing_url: String::from("zabor"),
//             url: String::from("fences"),
//             localizations: {
//                 let mut locale = HashMap::new();
//                 locale.insert(Lang::ru_RU, String::from("Заборы"));
//                 locale.insert(Lang::en_US, String::from("Fences"));
//                 locale
//             },
//             subcategories: None,
//             parent: None,
//         },
//         Category {
//             existing_url: String::from("vodostochnye-sistemy"),
//             url: String::from("drainage-systems"),
//             localizations: {
//                 let mut locale = HashMap::new();
//                 locale.insert(Lang::ru_RU, String::from("Водосточные Системы"));
//                 locale.insert(Lang::en_US, String::from("Drainage Systems"));
//                 locale
//             },
//             subcategories: None,
//             parent: None,
//         },
//         Category {
//             existing_url: String::from("obustroystvo-krovli"),
//             url: String::from("roofing-components"),
//             localizations: {
//                 let mut locale = HashMap::new();
//                 locale.insert(Lang::ru_RU, String::from("Обустройство Кровли"));
//                 locale.insert(Lang::en_US, String::from("Roofing Components"));
//                 locale
//             },
//             subcategories: None,
//             parent: None,
//         },
//     ];
//
//     for mut category in categories_to_add {
//         if category.url == "roofing".to_string() {
//             // println!("Found Roofing in {}", category.url);
//             // test adding a subcategory
//             let sub_cat = Category::new(
//                 "blya".to_string(),
//                 "better".to_string(),
//                 None,
//                 None,
//                 Some(&category),
//             );
//             // sub_cat.set_parent(category.clone());
//             let subcategories = category.subcategories.get_or_insert(HashMap::new());
//             subcategories.insert(sub_cat.url.clone(), &sub_cat);
//         }
//
//         categories.insert(category.url.clone(), category);
//     }
//
//     Ok(categories)
// }
