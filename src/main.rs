// https://crates.io/crates/actix-web
// https://github.com/actix/examples/tree/master/database_interactions/pg
// https://www.youtube.com/watch?v=PMa2m0Fe-Q4
// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
// https://crates.io/crates/bcrypt

pub mod config;
pub mod db;
pub mod errors;
pub mod files;
pub mod handlers;
pub mod models;

use rusty_blog::{Page, Meta};
use handlers::{*};

use actix_files::Files;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use tokio_postgres::NoTls;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(pool.clone()))
				.service(web::resource("/").route(web::get().to(get_home)))
				.service(web::resource("/about").route(web::get().to(get_about)))
				.service(web::resource("/articles/{article_ref}").route(web::get().to(get_article)))
				.service(web::resource("/authors").route(web::post().to(add_author)))
				.service(web::resource("/articles").route(web::post().to(add_article)))
			.service(Files::new("/", "./static/"))
			.default_service(web::to(|| not_found()))
	})
		.bind(config.server_addr.clone())?
		.run();

    println!("Server running at http://{}/", config.server_addr);
    server.await
}
