use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub login_token: String,
}

#[derive(Deserialize)]
pub struct NewPostForm {
    title: String,
    body: String,
}

#[derive(Deserialize)]
pub struct BlogActionForm {}
