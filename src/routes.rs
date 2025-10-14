
use rocket_db_pools::{Connection};
use rocket_db_pools::diesel::{prelude::*};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use crate::schema;
use crate::dtos::{Meeting, CreateMeeting, UpdateMeeting, OneOnOne, User};
use crate::{Db};

#[get("/users/<id>")]
pub async fn get_user(mut db: Connection<Db>, id: i32) -> Result<Json<User>, NotFound<String>>  {
    schema::users::dsl::users.find(id)
        .first::<User>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/one-on-ones/<id>")]
pub async fn get_one_on_one(mut db: Connection<Db>, id: i32) -> Result<Json<OneOnOne>, NotFound<String>>  {
    schema::one_on_ones::dsl::one_on_ones.find(id)
        .first::<OneOnOne>(&mut db)
        .await
        .map(Json)
        .map_err(|e| NotFound(e.to_string()))
}

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
