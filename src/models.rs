use chrono::{NaiveDate, Datelike};
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
	pub name_ref: String,
	pub name: String,
	pub content: String,
	pub author_name: String,
	pub password: String,
	pub description: String,
}


impl ArticleSubmission {
	pub fn to_article(&self, author_id: i32) -> Article {
		let now = chrono::Utc::now();
		let curr_date = NaiveDate::from_ymd(now.year(), now.month(), now.day());

		Article {
			id: 0, // dummy
			author_id:    author_id,
			content:      self.content.clone(),
			created_date: curr_date,
			edit_date:    curr_date,
			description:  self.description.clone(),
			name:         self.name.clone(),
			name_ref:     self.name_ref.clone(),
			thumbnail:    "_default".into(), // TODO
		}
	}
}

pub enum SubmissionResult {
	Accepted(Article),
	Rejected(String),
}

#[derive(Serialize)]
pub struct SubmissionResponseJson {
	pub status: u16,
	pub message: String,
}

impl SubmissionResponseJson {
	pub fn new(message: String, status: u16) -> Self {
		Self {
			message: message,
			status: status,
		}
	}

	pub fn not_found() -> Self {
		Self::new("Článek neexistuje".into(), 404)
	}
	
	pub fn unauthorized() -> Self {
		Self::new("Nejste obeprávněni".into(), 401)
	}

	pub fn edited_succ() -> Self {
		Self::new("Článek úspěšně upraven".into(), 201)
	}

	pub fn created_succ() -> Self {
		Self::new("Článek úspěšně vytvořen".into(), 200)
	}
}