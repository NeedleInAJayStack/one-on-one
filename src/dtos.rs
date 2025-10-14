use chrono;
use diesel::{self, prelude::*};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::{users, one_on_ones, meetings};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = one_on_ones)]
#[serde(crate = "rocket::serde")]
pub struct OneOnOne {
    pub id: i32,
    pub manager_id: i32,
    pub employee_id: Option<i32>,
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
