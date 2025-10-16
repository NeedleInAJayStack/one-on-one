use chrono;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

use crate::orm;
use crate::schema;
use crate::Db;

#[get("/meetings/<id>")]
pub async fn get_meeting(
    mut db: Connection<Db>,
    id: i32,
) -> Result<Json<Meeting>, NotFound<String>> {
    schema::meetings::dsl::meetings
        .find(id)
        .first::<orm::Meeting>(&mut db)
        .await
        .map(|meeting| {
            return Json(Meeting::from_orm(meeting));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[post("/meetings", data = "<meeting>")]
pub async fn post_meeting(
    mut db: Connection<Db>,
    meeting: Json<CreateMeeting>,
) -> Result<Json<Meeting>, NotFound<String>> {
    diesel::insert_into(schema::meetings::dsl::meetings)
        .values(&meeting.0.to_orm())
        .get_result::<orm::Meeting>(&mut db)
        .await
        .map(|meeting| {
            return Json(Meeting::from_orm(meeting));
        })
        // TODO: Fix error
        .map_err(|e| NotFound(e.to_string()))
}

#[put("/meetings/<id>", data = "<meeting>")]
pub async fn put_meeting(
    mut db: Connection<Db>,
    id: i32,
    meeting: Json<UpdateMeeting>,
) -> Result<Json<Meeting>, NotFound<String>> {
    diesel::update(schema::meetings::dsl::meetings.find(id))
        .set(&meeting.0.to_orm())
        .get_result::<orm::Meeting>(&mut db)
        .await
        .map(|meeting| {
            return Json(Meeting::from_orm(meeting));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[delete("/meetings/<id>")]
pub async fn delete_meeting(
    mut db: Connection<Db>,
    id: i32,
) -> Result<Json<Meeting>, NotFound<String>> {
    diesel::delete(schema::meetings::dsl::meetings.find(id))
        .get_result::<orm::Meeting>(&mut db)
        .await
        .map(|meeting| {
            return Json(Meeting::from_orm(meeting));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
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

impl Meeting {
    fn from_orm(meeting: orm::Meeting) -> Self {
        Meeting {
            id: meeting.id,
            one_on_one_id: meeting.one_on_one_id,
            date: meeting.date,
            manager_topics: meeting.manager_topics,
            employee_topics: meeting.employee_topics,
            notes: meeting.notes,
            action_items: meeting.action_items,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMeeting {
    pub one_on_one_id: i32,
    pub date: chrono::NaiveDateTime,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

impl CreateMeeting {
    fn to_orm(&self) -> orm::CreateMeeting {
        orm::CreateMeeting {
            one_on_one_id: self.one_on_one_id,
            date: self.date,
            manager_topics: self.manager_topics.clone(),
            employee_topics: self.employee_topics.clone(),
            notes: self.notes.clone(),
            action_items: self.action_items.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateMeeting {
    pub one_on_one_id: Option<i32>,
    pub date: Option<chrono::NaiveDateTime>,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

impl UpdateMeeting {
    fn to_orm(&self) -> orm::UpdateMeeting {
        orm::UpdateMeeting {
            one_on_one_id: self.one_on_one_id,
            date: self.date,
            manager_topics: self.manager_topics.clone(),
            employee_topics: self.employee_topics.clone(),
            notes: self.notes.clone(),
            action_items: self.action_items.clone(),
        }
    }
}
