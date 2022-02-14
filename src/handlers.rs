use crate::{db, errors::MyError, models::{*}};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_author(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let author_info: User = user.into_inner();
    let new_author = db::add_user(&client, author_info).await?;

    Ok(HttpResponse::Ok().json(new_author))
}

pub async fn add_article(
    article: web::Json<Article>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let article_info: User = article.into_inner();
    let new_article = db::add_article(&client, article_info).await?;

    Ok(HttpResponse::Ok().json(new_article))
}

async fn get_home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<p>Content of home</p>".into())
}

async fn get_about() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<p>Content of about</p>".into())
}

async fn get_article(web::Path((article_ref)): web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<p>Content of article <b>{}</b></p>", article_ref))
}