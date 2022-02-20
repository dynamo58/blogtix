use crate::{errors::MyError, models::*};

use bcrypt::verify;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_article(client: &Client, article: Article) -> Result<(), MyError> {
	let _stmt = "INSERT INTO articles(name_ref, name, content, description, thumbnail, author_id) VALUES ($1, $2, $3, $4, $5, $6);";
	let stmt = client.prepare(&_stmt).await.unwrap();

	client
		.query(
			&stmt,
			&[
				&article.name_ref,
				&article.name,
				&article.content,
				&article.description,
				&article.thumbnail,
				&article.author_id,
			],
		)
		.await?;
	
	Ok(())
}

pub async fn update_article(client: &Client, article: &Article) -> Result<(), MyError> {
	let _stmt = "UPDATE articles SET name=$1, content=$2, description=$3 WHERE name_ref=$4;";
	let stmt = client.prepare(&_stmt).await.unwrap();

	client
		.query(
			&stmt,
			&[
				&article.name,
				&article.content,
				&article.description,
				&article.name_ref,
			]
		)
		.await
		.unwrap();
	
	Ok(())
}

pub async fn get_article(client: &Client, article_ref: String) -> Result<(Article, Author), MyError> {
	let _stmt = "SELECT * FROM articles JOIN authors ON articles.author_id=authors.id WHERE name_ref=$1 LIMIT 1;";
	let stmt = client.prepare(&_stmt).await.unwrap();

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
	let _stmt = "SELECT * FROM articles ORDER BY created_date DESC;";
	let stmt = client.prepare(&_stmt).await.unwrap();

	Ok(client
		.query(&stmt, &[])
		.await?
		.iter()
		.map(|row| Article::from_row_ref(row).unwrap())
		.collect::<Vec<Article>>())
}

pub async fn auth_submission(client: &Client, sbmsn: &ArticleSubmission) -> Result<SubmissionResult, MyError> {
	let _stmt = "SELECT * FROM authors WHERE nick=$1;";
	let stmt = client.prepare(&_stmt).await.unwrap();

	let author = client
		.query(&stmt, &[&sbmsn.author_name])
		.await?
		.iter()
		.map(|row| Author::from_row_ref(row).unwrap())
		.collect::<Vec<Author>>()
		.pop();

	if let None = author {
		return Ok(SubmissionResult::Rejected("Nickname and author do not match".into()))
	}

	let author = author.unwrap();
	let valid = verify(sbmsn.password.clone(), &author.password).unwrap();

	if valid {
		let article = sbmsn.to_article(author.id);
		return Ok(SubmissionResult::Accepted(article));
	}

	Ok(SubmissionResult::Rejected("Nickname and password do not match".into()))
}

pub async fn article_exists(
	client: &Client,
	sbmsn: &ArticleSubmission
) -> Result<bool, MyError> {
	let _stmt = "SELECT id FROM articles WHERE name_ref=$1;";
	let stmt = client.prepare(&_stmt).await.unwrap();

	let articles = client
		.query(&stmt, &[&sbmsn.name_ref])
		.await?
		.iter()
		.map(|row| row.get("id"))
		.collect::<Vec<i32>>();
	
	match articles.len() {
		0 => Ok(false),
		_ => Ok(true), // should be 1 or 0, but whatever
	}
}
