pub mod data {
    pub mod categories;
}
pub mod web {
    pub mod scraper;
}
pub use web::scraper::test_parser;

pub use data::categories::{construct_url, seed};
