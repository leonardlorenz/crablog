mod api;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;
use once_cell::sync::Lazy;
use std::{collections::HashMap, env, sync::RwLock};
use tera::Tera;

pub static CONFIG_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let mut config: HashMap<String, String> = HashMap::new();

    let required_env_vars = [
        "SUBMIT_TOKEN",
        "ROOT_PATH",
        "USERNAME",
        "EMAIL",
        "BIND_PORT",
    ];

    let optional_env_vars = [
        "GITHUB_ACCOUNT",
        "TWITTER_ACCOUNT",
        "MASTODON_ACCOUNT",
        "DISCORD_ACCOUNT",
        "REDDIT_ACCOUNT",
    ];

    // Test if variable is set. If not, panic.
    let mut insert_required_env = |env: &str| {
        let env_string = String::from(env);
        config.insert(
            env_string.clone(), // env var name
            env::var(env_string).expect(format!("`{}` variable not set.", env).as_str()), // env var content
        )
    };

    for var in required_env_vars.iter() {
        insert_required_env(var);
    }

    // Test if variable is set. If it is insert into config.
    let mut insert_optional_env = |env: &str| {
        if let Ok(var_content) = env::var(String::from(env)) {
            config.insert(String::from(env), var_content.clone());
        }
    };

    for var in optional_env_vars.iter() {
        insert_optional_env(var);
    }

    // Print some info about the current configuration
    println!("Submit token = `{}`", config.get("SUBMIT_TOKEN").unwrap());
    println!(
        "Current working directory = `{}`",
        env::current_dir().unwrap().to_str().unwrap()
    );
    println!("Root path = `{}`", config.get("ROOT_PATH").unwrap());
    println!(
        "Template path = `{}/templates/*`",
        config.get("ROOT_PATH").unwrap()
    );
    println!("Launching on 0.0.0.0:{}", config.get("BIND_PORT").unwrap());
    RwLock::new(config)
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tera = Tera::new(
            format!(
                "{}{}",
                CONFIG_MAP.read().unwrap().get("ROOT_PATH").unwrap(),
                "/templates/*"
            )
            .as_str(),
        )
        .unwrap();
        tera.autoescape_on(vec![".sql"]);

        env_logger::Builder::from_env(Env::default().default_filter_or("info"));

        App::new()
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
                format!(
                    "{}{}",
                    CONFIG_MAP.read().unwrap().get("ROOT_PATH").unwrap(),
                    "/static"
                ),
            ))
            .wrap(Logger::new("%a %r %t"))
    })
    .bind(format!(
        "0.0.0.0:{}",
        CONFIG_MAP.read().unwrap().get("BIND_PORT").unwrap()
    ))?
    .run()
    .await
}
