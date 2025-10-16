use diesel::{self, prelude::*};

use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::one_on_ones)]
pub struct OneOnOne {
    pub id: i32,
    pub manager_id: i32,
    pub employee_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::meetings)]
pub struct Meeting {
    pub id: i32,
    pub one_on_one_id: i32,
    pub date: chrono::NaiveDateTime,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::meetings)]
pub struct CreateMeeting {
    pub one_on_one_id: i32,
    pub date: chrono::NaiveDateTime,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::meetings)]
pub struct UpdateMeeting {
    pub one_on_one_id: Option<i32>,
    pub date: Option<chrono::NaiveDateTime>,
    pub manager_topics: Option<String>,
    pub employee_topics: Option<String>,
    pub notes: Option<String>,
    pub action_items: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::surveys)]
pub struct Survey {
    pub id: i32,
    pub one_on_one_id: i32,
    pub question: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::survey_responses)]
pub struct SurveyResponse {
    pub id: i32,
    pub survey_id: i32,
    pub date: chrono::NaiveDateTime,
    pub question: String,
}
