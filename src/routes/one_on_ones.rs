use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

use crate::orm;
use crate::schema;
use crate::Db;

#[get("/one-on-ones")]
pub async fn get_one_on_ones(
    mut db: Connection<Db>,
) -> Result<Json<Vec<OneOnOne>>, NotFound<String>> {
    schema::one_on_ones::dsl::one_on_ones
        .load(&mut db)
        .await
        .map(|one_on_ones| {
            return Json(
                one_on_ones
                    .into_iter()
                    .map(|one_on_one| OneOnOne::from_orm(one_on_one))
                    .collect(),
            );
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/one-on-ones/<id>")]
pub async fn get_one_on_one(
    mut db: Connection<Db>,
    id: i32,
) -> Result<Json<OneOnOne>, NotFound<String>> {
    schema::one_on_ones::dsl::one_on_ones
        .find(id)
        .first::<orm::OneOnOne>(&mut db)
        .await
        .map(|one_on_one| {
            return Json(OneOnOne::from_orm(one_on_one));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OneOnOne {
    pub id: i32,
    pub manager_id: i32,
    pub employee_id: Option<i32>,
}

impl OneOnOne {
    fn from_orm(orm: orm::OneOnOne) -> Self {
        Self {
            id: orm.id,
            manager_id: orm.manager_id,
            employee_id: orm.employee_id,
        }
    }
}
