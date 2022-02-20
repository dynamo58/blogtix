use crate::db;
use crate::errors::MyError;
use crate::files::{build_html};
use crate::{Page, Meta, models::{ArticleSubmission, SubmissionResult, SubmissionResponseJson}};

use actix_web::{web, Error, HttpResponse, Responder, get, put, post};
use deadpool_postgres::{Client, Pool};

#[get("/")]
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

#[get("/about")]
pub async fn get_about() -> impl Responder {
	let md: String = include_str!("../assets/md/about.md").into();
	let meta = Meta::from([("{{ TITLE }}".into(), "O mně - smolik.xyz".into())]);

	let html = build_html(md, meta, Page::About);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/articles/{article_ref}")]
pub async fn get_article(
	db_pool: web::Data<Pool>,
	article_ref: web::Path<String>
) -> Result<HttpResponse, Error> {
	let article_ref = article_ref.into_inner();
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let (article, author) = db::get_article(&client, article_ref.clone()).await?;
	
	let meta = Meta::from([
		("TITLE".into(),     format!("{} - smolik.xyz", article.name)),
		("CREATED".into(),   format!("{}", article.created_date)),
		("LAST_EDIT".into(), format!("{}", article.edit_date)), 
		("AUTHOR".into(),    author.nick.into()), 
		("AVATAR".into(),    format!("{}", author.avatar)), 
	]);

	let html = build_html(article.content, meta, Page::Article);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

pub async fn not_found() -> impl Responder {
	let md = include_str!("../assets/md/400.md");
	let html = build_html(md.into(), Meta::new(), Page::Error);

	HttpResponse::Ok()
		.content_type("text/html")
		.body(html)
}

#[get("/edit/{article_ref}")]
pub async fn edit_article(
	db_pool: web::Data<Pool>,
	article_ref: web::Path<String>
) -> Result<HttpResponse, Error> {
	let article_ref = article_ref.into_inner();
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;


	let (article, author) = db::get_article(&client, article_ref).await?;

	let meta = Meta::from([
		("TITLE".into(), format!("Úprava „{}“ - smolik.xyz", article.name)),
		("NAME".into(), article.name),
		("DESCRIPTION".into(), article.description),
		("AUTHOR".into(), author.nick),

	]);

	let html = build_html(article.content, meta, Page::EditingArticle);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[get("/new/{article_ref}")]
pub async fn add_article() -> Result<HttpResponse, Error> {
	let meta = Meta::from([
		("TITLE".into(),       "Tvorba nového článku - smolik.xyz".into()),
		("NAME".into(),        "".into()),
		("DESCRIPTION".into(), "".into()),
		("AUTHOR".into(),      "".into()),

	]);

	let html = build_html("".into(), meta, Page::EditingArticle);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

// ------------------------------------------------------
// API
// - called from frontend
// ------------------------------------------------------

#[put("/api/article")]
pub async fn put_article(
	db_pool: web::Data<Pool>,
	article: web::Json<ArticleSubmission>,
) -> Result<HttpResponse, Error> {

	let submission = article.into_inner();
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
	
	let auth = crate::db::auth_submission(&client, &submission).await?;

	if let SubmissionResult::Rejected(_) = auth {
		return Ok(HttpResponse::Ok().json(SubmissionResponseJson::unauthorized()));
	}

	let article = match auth {
		SubmissionResult::Accepted(a) => a,
		_ => unreachable!(),
	};
	
	let exists = db::article_exists(&client, &submission).await?;
	
	if !exists {
		return Ok(HttpResponse::Ok().json(SubmissionResponseJson::not_found()));
	}

	db::update_article(&client, &article).await?;

    Ok(HttpResponse::Ok().json(SubmissionResponseJson::edited_succ()))
}

#[post("/api/article")]
pub async fn post_article(
    article: web::Json<ArticleSubmission>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let submission: ArticleSubmission = article.into_inner();

	let auth = db::auth_submission(&client, &submission).await?;

	match auth {
		SubmissionResult::Accepted(article) => {
			db::add_article(&client, article).await?;
			Ok(HttpResponse::Ok().json(SubmissionResponseJson::created_succ()))
		},
		SubmissionResult::Rejected(_) => {
			Ok(HttpResponse::Ok().json(SubmissionResponseJson::unauthorized()))
		}
	}
}
