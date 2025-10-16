use chrono;
use diesel::{self};
use rocket_db_pools::{Connection};
use rocket_db_pools::diesel::{prelude::*};
use rocket::response::status::NotFound;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use crate::schema;
use crate::schema::{meetings};
use crate::{Db};

#[get("/meetings/<id>")]
pub async fn get_meeting(mut db: Connection<Db>, id: i32) -> Result<Json<Meeting>, NotFound<String>>  {
    schema::meetings::dsl::meetings.find(id)
        .first::<Meeting>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[post("/meetings", data = "<meeting>")]
pub async fn post_meeting(mut db: Connection<Db>, meeting: Json<CreateMeeting>) -> Result<Json<Meeting>, NotFound<String>>  {
    diesel::insert_into(schema::meetings::dsl::meetings)
        .values(&meeting.0)
        .get_result::<Meeting>(&mut db)
        .await
        .map(Json)
        // TODO: Fix error
        .map_err(|e| NotFound(e.to_string()))
}

#[put("/meetings/<id>", data = "<meeting>")]
pub async fn put_meeting(mut db: Connection<Db>, id: i32, meeting: Json<UpdateMeeting>) -> Result<Json<Meeting>, NotFound<String>>  {
    diesel::update(schema::meetings::dsl::meetings.find(id))
        .set(&meeting.0)
        .get_result::<Meeting>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[delete("/meetings/<id>")]
pub async fn delete_meeting(mut db: Connection<Db>, id: i32) -> Result<Json<Meeting>, NotFound<String>>  {
    diesel::delete(schema::meetings::dsl::meetings.find(id))
        .get_result::<Meeting>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = meetings)]
#[serde(crate = "rocket::serde")]
pub struct Meeting {
    pub id: i32,
    pub one_on_one_id: i32,
    pub date: chrono::NaiveDateTime,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = meetings)]
#[serde(crate = "rocket::serde")]
pub struct CreateMeeting {
    pub one_on_one_id: i32,
    pub date: chrono::NaiveDateTime,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = meetings)]
#[serde(crate = "rocket::serde")]
pub struct UpdateMeeting {
    pub one_on_one_id: Option<i32>,
    pub date: Option<chrono::NaiveDateTime>,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}
