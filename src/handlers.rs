use crate::db;
use crate::errors::MyError;
use crate::models::{*};
use crate::files::{build_html};
use crate::{Page, Meta};

use actix_web::{web, Error, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn add_author(
    user: web::Json<Author>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let author_info: Author = user.into_inner();
    let new_author = db::add_author(&client, author_info).await?;

    Ok(HttpResponse::Ok().json(new_author))
}

pub async fn add_article(
    article: web::Json<Article>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let article_info: Article = article.into_inner();
    let new_article = db::add_article(&client, article_info).await?;

    Ok(HttpResponse::Ok().json(new_article))
}

pub async fn get_home(
	db_pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let articles = db::get_articles(&client).await?;
	let mut html: String = include_str!("../assets/html/default.html").into();
	let mut links = String::new();

	for a in articles.into_iter() {
		links += &format!("
			<a href=\"/articles/{}\">
				<div class=\"article\">
					<h2 class=\"article-name\">{}</h2>
					<hr>
					<div class=\"article-body_wrapper\">
						<img class=\"article-img\" src=\"/thumbnails/{}.webp\">
						<div class=\"article-desc\">{}</div>
					</div>
				</div>
			</a>
		", a.name_ref, a.name, a.thumbnail, a.description);
	}
	html = html
		.replace("{{ CONTENT }}", &links)
		.replace("{{ TITLE }}", "smolik.xyz");

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

pub async fn get_about() -> impl Responder {
	let md: String = include_str!("../assets/md/about.md").into();
	let meta = Meta::from([("{{ TITLE }}".into(), "O mně - smolik.xyz".into())]);

	let html = build_html(md, meta, Page::About);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub async fn get_article(
	db_pool: web::Data<Pool>,
	article_ref: String,
) -> Result<HttpResponse, Error> {
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let article = db::get_article(&client, article_ref).await?;
	let meta = Meta::from([
		("{{ TITLE }}".into(), format!("{} - smolik.xyz", article.name_ref)),
		("{{ CREATED }}".into(), article.created_date),
		("{{ LAST_EDIT }}".into(), article.edit_date), 
		("{{ AUTHOR }}".into(), article.), 
		("{{ AVATAR }}".into(), article.created_date), 
	]);

	let html = build_html(article.content, meta: Meta, Page::Article);


    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<p>Content of article <b>{}</b></p>", article_ref))
}

pub async fn not_found() -> impl Responder {
	HttpResponse::Ok()
		.content_type("text/html")
		.body("<p>Route not found</p>")
}
