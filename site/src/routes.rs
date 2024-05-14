use crate::db;

use super::CONFIG;
use actix_web::{error, get, http::StatusCode, web, Error, HttpResponse};
use tera::Context;

/// tests if the post id is a valid i32 integer bigger than zero
/// assert(!(id_valid("2147483648").0))
/// assert(!(id_valid("-1").0))
/// assert(id_valid("1").0))
pub fn id_valid(post_id: String) -> (bool, i32) {
    match post_id.parse::<i32>() {
        Err(_) => (false, 0),
        Ok(id) => {
            if id < 1 {
                (false, id)
            } else {
                (true, id)
            }
        }
    }
}

/// replaces the \n character with a <br> html tag
/// assert(replace_newlines("test\ntest") == "test<br>test")
pub fn replace_newlines(x: &str) -> String {
    x.replace("\n", "<br>")
}

/// replaces the \n character with a <br> html tag
/// assert(replace_newlines("test<br>test") == "test\ntest")
pub fn replace_br_tags(x: &str) -> String {
    x.replace("<br>", "\n")
}

#[get("/about")]
async fn about(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert(
        "username",
        &CONFIG.username
    );
    context.insert("email", &CONFIG.email);

    match &CONFIG.accounts.github { 
        Some(acc) => context.insert("github_account", &acc),
        None => ()
    };
    match &CONFIG.accounts.twitter { 
        Some(acc) => context.insert("twitter_account", &acc),
        None => ()
    };
    match &CONFIG.accounts.mastodon { 
        Some(acc) => context.insert("mastodon_account", &acc),
        None => ()
    };
    match &CONFIG.accounts.reddit { 
        Some(acc) => context.insert("reddit_account", &acc),
        None => ()
    };
    match &CONFIG.accounts.discord { 
        Some(acc) => context.insert("discord_account", &acc),
        None => ()
    };

    let result = tmpl
        .render("about.html", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error\n{}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/")]
async fn blog(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let posts = db::get_last_five_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("username", &CONFIG.username);

    let result = tmpl
        .render("blog.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/all")]
async fn blog_all(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let posts = db::get_all_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("username", &CONFIG.username);

    let result = tmpl
        .render("blog-all-posts.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/id/{post_id}")]
async fn blog_by_id(
    tmpl: web::Data<tera::Tera>,
    post_id: web::Path<String>
    // web::Path(post_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let (valid, id) = id_valid(post_id.into_inner());
    if valid {
        let post = db::get_post_by_id(id as i32);

        if !post.published {
            return Ok(HttpResponse::new(StatusCode::UNAUTHORIZED));
        }

        let mut context = Context::new();
        context.insert("post", &post);
        context.insert("username", &CONFIG.username);

        let result = tmpl
            .render("blog-by-id.html", &context)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

        return Ok(HttpResponse::Ok().content_type("text/html").body(result));
    } else {
        return Ok(HttpResponse::new(StatusCode::NOT_FOUND));
    }
}

#[get("/submit")]
async fn blog_submit(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", "");
    context.insert("body", "");

    let result = tmpl
        .render("submit.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    return Ok(HttpResponse::Ok().content_type("text/html").body(result));
}

#[get("/edit")]
async fn blog_edit(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("posts", &db::get_all_posts());
    context.insert("username", &CONFIG.username);

    let result = tmpl
        .render("edit.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/edit/{post_id}")]
async fn blog_edit_by_id(
    tmpl: web::Data<tera::Tera>,
    post_id: web::Path<String>
) -> Result<HttpResponse, Error> {
    let (valid, id) = id_valid(post_id.into_inner());
    if valid {
        let mut post = db::get_post_by_id(id as i32);

        post.title = replace_br_tags(&post.title);
        post.body = replace_br_tags(&post.body);

        let mut context = Context::new();
        context.insert("title", &post.title);
        context.insert("body", &post.body);
        context.insert("id", &id);

        let result = tmpl
            .render("edit-form.html", &context)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(result))
    } else {
        Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
    }
}
