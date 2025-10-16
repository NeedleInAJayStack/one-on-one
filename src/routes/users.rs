use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

use crate::orm;
use crate::schema;
use crate::Db;

#[get("/users/<id>")]
pub async fn get_user(mut db: Connection<Db>, id: i32) -> Result<Json<User>, NotFound<String>> {
    schema::users::dsl::users
        .find(id)
        .first::<orm::User>(&mut db)
        .await
        .map(|user| {
            return Json(User::from_orm(user));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    fn from_orm(orm: orm::User) -> Self {
        Self {
            id: orm.id,
            name: orm.name,
            email: orm.email,
        }
    }
}
