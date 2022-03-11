#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::LoadPaginated;

use crate::posts::model::Post;
use crate::posts::model::NewPost;

use crate::schema::posts;

use rocket::request::Form;

#[derive(FromForm)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_by: Option<String>,
}

pub fn create_post(new_post: NewPost, conn: &SqliteConnection) -> Result<usize, Error> {
    Ok(diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post"))
}

pub fn show_posts(connection: &SqliteConnection, params: Form<Params>) -> Result<(Vec<Post>, i64), Error>  {
    let mut query = posts::table.into_boxed();
    if let Some(sort_by) = &params.sort_by {
        query = match sort_by.as_ref() {
            "id" => query.order(posts::id.asc()),
            "id.asc" => query.order(posts::id.asc()),
            "id.desc" => query.order(posts::id.desc()),
            _ => query,
        };
    }

    let (found_posts, total_pages) = query
        .load_with_pagination(&connection, params.page, params.page_size)?;

    Ok((found_posts, total_pages))
}

pub fn get_post(post_id: i32, connection: &SqliteConnection) -> QueryResult<Post> {
    posts::table.find(post_id).get_result::<Post>(connection)
}

pub fn update_post(post_id: i32, post: Post, connection: &SqliteConnection) -> Result<usize, Error> {
    Ok(diesel::update(posts::table.find(post_id))
        .set(&post)
        .execute(connection)
        .expect("Error saving new post"))
}

pub fn delete_post(post_id: i32, connection: &SqliteConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(post_id))
        .execute(connection)
}
