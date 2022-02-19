#[allow(dead_code)]

pub mod config;
pub mod db;
pub mod errors;
pub mod files;
pub mod handlers;
pub mod models;

pub type Meta = std::collections::HashMap<String, String>;

pub trait MetaTraits {
	fn new() -> Self;
	fn from_title(t: &str) -> Self;
}

impl MetaTraits for Meta {
	fn new() -> Meta {
		std::collections::HashMap::new()
	}

	fn from_title(title: &str) -> Meta {
		std::collections::HashMap::from([
			("{{ TITLE }}".into(), title.into())
		])
	}
}

pub enum Page {
    About,
    Article,
    Error,
    Home,
	EditingArticle,
}
