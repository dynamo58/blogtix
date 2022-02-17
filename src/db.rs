use crate::{errors::MyError, models::*};


use bcrypt::verify;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use chrono::{NaiveDate, Datelike};
use std::time::SystemTime;
// pub async fn add_author(client: &Client, author_info: Author) -> Result<Author, MyError> {
// 	let _stmt =
// 		"INSERT INTO authors(avatar, nick, bio) VALUES ($1, $2, $3) RETURNING $table_fields;";
// 	let _stmt = _stmt.replace("$table_fields", &Author::sql_table_fields());
// 	let stmt = client.prepare(&_stmt).await.unwrap();

// 	client
// 		.query(
// 			&stmt,
// 			&[&author_info.avatar, &author_info.nick, &author_info.bio],
// 		)
// 		.await?
// 		.iter()
// 		.map(|row| Author::from_row_ref(row).unwrap())
// 		.collect::<Vec<Author>>()
// 		.pop()
// 		.ok_or(MyError::NotFound)
// }

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
	let _stmt = "SELECT * FROM articles";
	let stmt = client.prepare(&_stmt).await.unwrap();

	Ok(client
		.query(&stmt, &[])
		.await?
		.iter()
		.map(|row| Article::from_row_ref(row).unwrap())
		.collect::<Vec<Article>>())
}


pub async fn auth_submission(client: &Client, sbmsn: ArticleSubmission) -> Result<SubmissionResult, MyError> {
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
		let stamp = format!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros());
		let now = chrono::Utc::now();
		let curr_date = NaiveDate::from_ymd(now.year(), now.month(), now.day());

		let article = Article {
			id: 0,
			author_id: author.id,
			content: sbmsn.content,
			created_date: curr_date,
			edit_date: curr_date,
			description: sbmsn.description,
			name: sbmsn.name,
			name_ref: stamp,
			thumbnail: "_default".into(), // TODO
		};

		return Ok(SubmissionResult::Accepted(article));
	}

	Ok(SubmissionResult::Rejected("".into()))
}
