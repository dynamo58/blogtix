use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "authors")]
pub struct Author {
	pub id: i32,
    pub avatar: String,
    pub nick: String,
    pub bio: String,
    pub registered_date: chrono::NaiveDate,
	pub password: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "authors")]
pub struct Article {
	pub id: i32,
    pub name_ref: String,
    pub name: String,
    pub content: String,
    pub description: String,
    pub thumbnail: String,
    pub edit_date: chrono::NaiveDate,
    pub created_date: chrono::NaiveDate,
    pub author_id: i32,
}

#[derive(Deserialize)]
pub struct ArticleSubmission {
	pub name: String,
	pub content: String,
	pub author_name: String,
	pub password: String,
	pub description: String,
}

pub enum SubmissionResult {
	Accepted(Article),
	Rejected(String),
}