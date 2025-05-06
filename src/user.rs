
use rocket::serde::{Serialize, Deserialize};

use crate::{users};
use diesel::{self, prelude::*};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
