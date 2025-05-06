#[macro_use] extern crate rocket;
use rocket::response::status::NotFound;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{QueryResult, PgPool, prelude::*};
use rocket::serde::json::Json;

mod schema;
use schema::{users};
mod user;
use user::{User};

#[derive(Database)]
#[database("one_on_one")]
struct Db(PgPool);

#[get("/users/<id>")]
async fn get_user(mut db: Connection<Db>, id: i32) -> Result<Json<User>, NotFound<String>>  {
    use schema::users::dsl::*;
    users.find(id)
        .first::<User>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_user])
}
