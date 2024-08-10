use crate::db::schema::posts;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
    pub publish_date: &'a chrono::NaiveDateTime,
}
