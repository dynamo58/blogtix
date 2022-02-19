use crate::files::build_html;
use crate::{Page, Meta};

use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum MyError {
	NotFound,
	NotAuthorized,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
			MyError::NotFound => {
				let md = include_str!("../assets/md/400.md");
				let meta = Meta::from([
					("TITLE".into(),
					"Nenalezeno - smolik.xyz".into())
				]);
				let html = build_html(md.into(), meta, Page::Error);

				HttpResponse::Ok()
					.content_type("text/html")
					.body(html)
			},
			MyError::NotAuthorized => {
				let md = include_str!("../assets/md/401.md");
				let meta = Meta::from([
					("TITLE".into(),
					"Nejste oprávněni - smolik.xyz".into())
				]);
				let html = build_html(md.into(), meta, Page::Error);

				HttpResponse::Ok()
					.content_type("text/html")
					.body(html)
			},
    		_ => {
				let md = include_str!("../assets/md/500.md");
				let html = build_html(md.into(), Meta::new(), Page::Error);

				HttpResponse::Ok()
					.content_type("text/html")
					.body(html)
			}
        }
    }
}