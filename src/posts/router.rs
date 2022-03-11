use rocket;

use crate::db;
use crate::posts;

pub fn create_routes() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/posts",
               routes![
                    posts::handler::all_posts,
                    posts::handler::create_post,
                    posts::handler::get_post,
                    posts::handler::update_post,
                    posts::handler::delete_post
                    ],
        ).launch();
}
