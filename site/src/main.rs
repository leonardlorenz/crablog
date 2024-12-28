mod api;
mod config;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use config::CONFIG;
use env_logger::Env;
use once_cell::sync::Lazy;
use tera::Tera;

static SESSION_COOKIE_SECRET_KEY: Lazy<Key> = Lazy::new(|| Key::generate());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut tera =
            Tera::new(format!("{}{}", CONFIG.root_path, "/templates/*").as_str()).unwrap();
        tera.autoescape_on(vec![".sql"]);

        env_logger::Builder::from_env(Env::default().default_filter_or("debug"));

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                SESSION_COOKIE_SECRET_KEY.clone(),
            ))
            .app_data(Data::new(tera))
            .service(routes::about)
            .service(routes::blog)
            .service(routes::blog_all)
            .service(routes::blog_by_id)
            .service(routes::blog_submit)
            .service(routes::blog_edit)
            .service(routes::blog_edit_by_id)
            .service(api::blog_get_posts_json)
            .service(api::blog_create_post)
            .service(api::blog_edit_post)
            .service(api::blog_hide_post)
            .service(api::blog_delete_post)
            .service(fs::Files::new(
                "/static",
                format!("{}{}", CONFIG.root_path, "/static"),
            ))
            .wrap(Logger::new("%a %r %t"))
    })
    .bind(format!("0.0.0.0:{}", CONFIG.bind_port))?
    .run()
    .await
}
