use crate::{errors::MyError, models::{*}};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_author(client: &Client, author_info: Article) -> Result<Author, MyError> {
    let _stmt = include_str!("../sql/add_author.sql");
    let _stmt = _stmt.replace("$table_fields", &Author::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &author_info.avatar,
                &author_info.nick,
                &author_info.bio,
            ],
        )
        .await?
        .iter()
        .map(|row| Author::from_row_ref(row).unwrap())
        .collect::<Vec<Author>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn add_article(client: &Client, article_info: Article) -> Result<Article, MyError> {
    let _stmt = include_str!("../sql/add_article.sql");
    let _stmt = _stmt.replace("$table_fields", &Article::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &author_info.name_ref,
                &author_info.name,
                &author_info.content,
                &author_info.description,
                &author_info.thumbnail,
                &author_info.author_id,
            ],
        )
        .await?
        .iter()
        .map(|row| Article::from_row_ref(row).unwrap())
        .collect::<Vec<Article>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_article(client: &Client, article_ref: &str) -> Result<Article, MyError> {
    let _stmt = "SELECT * FROM articles WHERE name_ref=$1 LIMIT 1";
    let _stmt = _stmt.replace("$table_fields", &Article::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                article_ref
            ],
        )
        .await?
        .iter()
        .map(|row| Article::from_row_ref(row).unwrap())
        .collect::<Vec<Article>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_articles(client: &Client, article_info: Article) -> Result<Article, MyError> {
    let _stmt = "SELECT * FROM articles";
    let _stmt = _stmt.replace("$table_fields", &Article::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[],
        )
        .await?
        .iter()
        .map(|row| Article::from_row_ref(row).unwrap())
        .collect::<Vec<Article>>()
        .pop()
        .ok_or(MyError::NotFound)
}