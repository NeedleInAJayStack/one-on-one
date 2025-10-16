
use diesel::{self};
use rocket_db_pools::{Connection};
use rocket_db_pools::diesel::{prelude::*};
use rocket::response::status::NotFound;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use crate::schema;
use crate::{Db};

#[get("/users/<id>")]
pub async fn get_user(mut db: Connection<Db>, id: i32) -> Result<Json<User>, NotFound<String>>  {
    schema::users::dsl::users.find(id)
        .first::<User>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}
use crate::schema::{users};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
