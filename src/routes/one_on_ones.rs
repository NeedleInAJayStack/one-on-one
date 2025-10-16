use diesel::{self};
use rocket_db_pools::{Connection};
use rocket_db_pools::diesel::{prelude::*};
use rocket::response::status::NotFound;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use crate::schema;
use crate::schema::{one_on_ones};
use crate::{Db};

#[get("/one-on-ones/<id>")]
pub async fn get_one_on_one(mut db: Connection<Db>, id: i32) -> Result<Json<OneOnOne>, NotFound<String>>  {
    schema::one_on_ones::dsl::one_on_ones.find(id)
        .first::<OneOnOne>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = one_on_ones)]
#[serde(crate = "rocket::serde")]
pub struct OneOnOne {
    pub id: i32,
    pub manager_id: i32,
    pub employee_id: Option<i32>,
}
