use crate::{errors::MyError, models::*};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_author(client: &Client, author_info: Author) -> Result<Author, MyError> {
	let _stmt =
		"INSERT INTO authors(avatar, nick, bio) VALUES ($1, $2, $3) RETURNING $table_fields;";
	let _stmt = _stmt.replace("$table_fields", &Author::sql_table_fields());
	let stmt = client.prepare(&_stmt).await.unwrap();

	client
		.query(
			&stmt,
			&[&author_info.avatar, &author_info.nick, &author_info.bio],
		)
		.await?
		.iter()
		.map(|row| Author::from_row_ref(row).unwrap())
		.collect::<Vec<Author>>()
		.pop()
		.ok_or(MyError::NotFound)
}

pub async fn add_article(client: &Client, article_info: Article) -> Result<Article, MyError> {
	let _stmt = "INSERT INTO articles(name_ref, name, content, description, thumbnail, author_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING $table_fields;";
	let _stmt = _stmt.replace("$table_fields", &Article::sql_table_fields());
	let stmt = client.prepare(&_stmt).await.unwrap();

	client
		.query(
			&stmt,
			&[
				&article_info.name_ref,
				&article_info.name,
				&article_info.content,
				&article_info.description,
				&article_info.thumbnail,
				&article_info.author_id,
			],
		)
		.await?
		.iter()
		.map(|row| Article::from_row_ref(row).unwrap())
		.collect::<Vec<Article>>()
		.pop()
		.ok_or(MyError::NotFound)
}

pub async fn get_article(client: &Client, article_ref: String) -> Result<(Article, Author), MyError> {
	let _stmt = "SELECT * FROM articles JOIN authors ON articles.author_id=authors.id WHERE name_ref=$1 LIMIT 1;";
	let stmt = client.prepare(&_stmt).await.unwrap();
	// there must be a better way to do this
	// how the fuck do I use inner join here?
	client
		.query(&stmt, &[&article_ref])
		.await?
		.iter()
		.map(|row| {
				(
					Article::from_row_ref(row).unwrap(),
					Author::from_row_ref(row).unwrap(),
				)
		})
		.collect::<Vec<(Article, Author)>>()
		.pop()
		.ok_or(MyError::NotFound)
}

pub async fn get_articles(client: &Client) -> Result<Vec<Article>, MyError> {
	let _stmt = "SELECT * FROM articles";
	// let _stmt = _stmt.replace("$table_fields", &Article::sql_table_fields());
	let stmt = client.prepare(&_stmt).await.unwrap();

	Ok(client
		.query(&stmt, &[])
		.await?
		.iter()
		.map(|row| Article::from_row_ref(row).unwrap())
		.collect::<Vec<Article>>())
}
