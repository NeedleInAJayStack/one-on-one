mod orm;
mod routes;
mod schema;

#[macro_use]
extern crate rocket;
use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("one_on_one")]
struct Db(PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount(
        "/",
        routes![
            routes::users::get_user,
            routes::one_on_ones::get_one_on_one,
            routes::meetings::get_meeting,
            routes::meetings::post_meeting,
            routes::meetings::put_meeting,
            routes::meetings::delete_meeting,
        ],
    )
}
