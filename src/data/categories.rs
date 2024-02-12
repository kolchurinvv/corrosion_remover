use std::{collections::HashMap, error::Error};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Lang {
    #[allow(non_camel_case_types)]
    en_US,
    #[allow(non_camel_case_types)]
    ru_RU,
}

#[derive(Clone, Debug)]
pub struct Category {
    url: String,
    pub name: String,
    pub existing_url: String,
    pub local: HashMap<Lang, String>,
    pub subcategories: Vec<Category>,
    pub products: Vec<Product>,
}

impl Category {
    fn new(name: &str, existing_url: &str, local: HashMap<Lang, String>) -> Self {
        Category {
            name: name.to_string(),
            url: name.to_string(),
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
#[derive(Debug, Clone)]
struct Color {
    name: String,
    url: String,
    // value: String,
}

#[derive(Clone, Debug)]
pub struct Product {
    name: String,
    char: Option<String>,
    colors: Option<Vec<Color>>,
    desc: Option<String>,
}

impl Product {
    fn new(name: &str, char: Option<&str>, colors: Option<Vec<Color>>, desc: Option<&str>) -> Self {
        let mut prod = Product {
            name: name.to_string(),
            char: None,
            colors: None,
            desc: None,
        };
        if let Some(char) = char {
            prod.char = Some(char.to_string());
        };
        if let Some(colors) = colors {
            prod.colors = Some(colors);
        };
        if let Some(desc) = desc {
            prod.desc = Some(desc.to_string());
        };
        prod
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
    let mut categories: HashMap<String, Box<Category>> = HashMap::new();

    let categories_to_add = vec![
        Category {
            existing_url: String::from("krovlya"),
            url: String::from("roofing"),
            name: String::from("roofing"),
            local: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Кровля"));
                locale.insert(Lang::en_US, String::from("Roofing"));
                locale
            },
            subcategories: Vec::new(),
            products: Vec::new(),
        },
        Category {
            existing_url: String::from("fasady"),
            url: String::from("facades"),
            name: String::from("facades"),
            local: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Фасады"));
                locale.insert(Lang::en_US, String::from("Facades"));
                locale
            },
            subcategories: Vec::new(),
            products: Vec::new(),
        },
        Category {
            existing_url: String::from("zabor"),
            url: String::from("fences"),
            name: String::from("fences"),
            local: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Заборы"));
                locale.insert(Lang::en_US, String::from("Fences"));
                locale
            },
            subcategories: Vec::new(),
            products: Vec::new(),
        },
        Category {
            existing_url: String::from("vodostochnye-sistemy"),
            url: String::from("drainage-systems"),
            name: String::from("drainage-systems"),
            local: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Водосточные Системы"));
                locale.insert(Lang::en_US, String::from("Drainage Systems"));
                locale
            },
            subcategories: Vec::new(),
            products: Vec::new(),
        },
        Category {
            existing_url: String::from("obustroystvo-krovli"),
            url: String::from("roofing-components"),
            name: String::from("roofing-components"),
            local: {
                let mut locale = HashMap::new();
                locale.insert(Lang::ru_RU, String::from("Обустройство Кровли"));
                locale.insert(Lang::en_US, String::from("Roofing Components"));
                locale
            },
            subcategories: Vec::new(),
            products: Vec::new(),
        },
    ];

    for category in categories_to_add {
        categories.insert(category.url.clone(), Box::new(category));
    }

    Ok(categories)
}
