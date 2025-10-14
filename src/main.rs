mod schema;
mod dtos;
mod routes;

#[macro_use] extern crate rocket;
use rocket_db_pools::{Database};
use rocket_db_pools::diesel::{PgPool};

#[derive(Database)]
#[database("one_on_one")]
struct Db(PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![
            routes::get_user,

            routes::get_one_on_one,

            routes::get_meeting,
            routes::post_meeting,
            routes::put_meeting,
            routes::delete_meeting,
        ])
}
