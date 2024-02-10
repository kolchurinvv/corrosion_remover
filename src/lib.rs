pub mod data {
    pub mod categories;
}
mod web;

pub use data::categories::{construct_url, seed};
