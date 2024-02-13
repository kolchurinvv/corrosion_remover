pub mod data {
    pub mod categories;
}
pub mod web {
    pub mod scraper;
}
pub mod operators {
    pub mod populate_product_tree;
}

pub use data::categories::{construct_url, seed};
pub use web::scraper::test_parser;
