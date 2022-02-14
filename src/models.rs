use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "authors")]
pub struct Author {
    pub id: String,
    pub avatar: String,
    pub nick: String,
    pub bio: String,
    pub registered_date: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "authors")]
pub struct Article {
    pub id: String,
    pub name_ref: String,
    pub name: String,
    pub content: String,
    pub description: String,
    pub thumbnail: String,
    pub edit_date: String,
    pub created_date: String,
    pub author_id: String,
}