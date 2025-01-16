use crate::config::CONFIG;
use crate::db::*;
use crate::routes::{id_valid, replace_newlines};
use actix_identity::Identity;
use actix_web::{get, http::StatusCode, post, web, web::Form, HttpResponse, Responder};
use actix_web::{HttpMessage, HttpRequest};

use crate::form_data::NewPostForm;
use crate::form_data::{BlogActionForm, LoginForm};

#[get("/")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[post("/login")]
async fn blog_login(form: Form<LoginForm>, req: HttpRequest) -> impl Responder {
    let submitted_login_token = form.login_token.clone();
    if submitted_login_token == CONFIG.login_token {
        // attach a verified user identity to the active session
        Identity::login(&req.extensions(), "default_user".into()).unwrap();

        HttpResponse::Ok()
    } else {
        HttpResponse::Unauthorized()
    }
}

#[post("/logout")]
async fn blog_logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[post("/api/blog/create")]
async fn blog_create_post(form: Form<NewPostForm>) -> impl Responder {
    if CONFIG.submit_token == form.token {
        create_post(&form.title.as_str(), replace_newlines(&form.body).as_str());
        println!("New blog post created.");
    } else {
        println!("Unauthorized new blog post");
    }

    HttpResponse::MovedPermanently()
        .insert_header(("LOCATION", "/"))
        .finish()
}

#[post("/api/blog/posts/edit/{post_id}")]
async fn blog_edit_post(post_id: web::Path<String>, form: Form<NewPostForm>) -> impl Responder {
    let (valid, id) = id_valid(post_id.into_inner());
    if valid && CONFIG.submit_token == form.token {
        edit_post_by_id(
            id as i32,
            &form.title.as_str(),
            replace_newlines(&form.body).as_str(),
        );
        println!("Edited post: {}", id);
    } else {
        println!("Unauthorized blog post edit.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .insert_header(("LOCATION", "/"))
        .finish();
}

#[post("/api/blog/posts/delete/{post_id}")]
async fn blog_delete_post(post_id: web::Path<String>) -> impl Responder {
    let (valid, id) = id_valid(post_id.into_inner());
    // TODO
    if valid && AUTHENTICATED {
        println!("Deleted post: {}", id);
        delete_post_by_id(id as i32);
    } else {
        println!("Unauthorized blog post deletion.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .insert_header(("LOCATION", "/"))
        .finish();
}

#[post("/api/blog/posts/hide/{post_id}")]
async fn blog_hide_post(post_id: web::Path<String>, form: Form<BlogActionForm>) -> impl Responder {
    let (valid, id) = id_valid(post_id.into_inner());
    if valid && CONFIG.submit_token == form.token {
        println!("Hid post: {}", id);
        hide_post_by_id(id as i32);
    } else {
        println!("Unauthorized blog post hiding.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .insert_header(("LOCATION", "/"))
        .finish();
}

#[get("/api/blog/posts")]
async fn get_posts_json(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        let posts = get_all_posts();
        HttpResponse::Ok().json(posts)
    }
    return HttpResponse::new(StatusCode::UNAUTHORIZED);
}
