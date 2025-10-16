use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

use crate::orm;
use crate::schema;
use crate::Db;

#[get("/surveys/<id>")]
pub async fn get_survey(mut db: Connection<Db>, id: i32) -> Result<Json<Survey>, NotFound<String>> {
    schema::surveys::dsl::surveys
        .find(id)
        .first::<orm::Survey>(&mut db)
        .await
        .map(|survey| {
            return Json(Survey::from_orm(survey));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Survey {
    pub id: i32,
    pub one_on_one_id: i32,
    pub question: String,
}

impl Survey {
    fn from_orm(orm: orm::Survey) -> Self {
        Self {
            id: orm.id,
            one_on_one_id: orm.one_on_one_id,
            question: orm.question,
        }
    }
}
