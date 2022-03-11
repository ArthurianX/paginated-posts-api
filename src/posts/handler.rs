use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::posts;
use crate::posts::model::Post;
use crate::posts::model::NewPost;
use crate::posts::repository::Params;

#[get("/?<params..>")]
pub fn all_posts(connection: DbConn, params: Form<Params>) -> Result<Json<(Vec<Post>, i64)>, Status> {
    posts::repository::show_posts(&connection, params)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_post>")]
pub fn create_post(new_post: Json<NewPost>, connection: DbConn) ->  Result<status::Created<Json<usize>>, Status> {
    println!("here 0 {}",&new_post.title);
    posts::repository::create_post(new_post.into_inner(), &connection)
        .map(|post_index| post_created(post_index))
        .map_err(|error| error_status(error))

}

#[get("/<id>")]
pub fn get_post(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    posts::repository::get_post(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: i32, post: Json<Post>, connection: DbConn) -> Result<status::Created<Json<usize>>, Status> {
    posts::repository::update_post(id, post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_post(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    posts::repository::delete_post(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}


fn post_created(post_id: usize) -> status::Created<Json<usize>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/post/{id}", host = host(), port = port(), id = post_id).to_string(),
        Some(Json(post_id)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
