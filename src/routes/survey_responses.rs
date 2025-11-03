use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

use crate::orm;
use crate::schema;
use crate::Db;

#[get("/surveys-response/<id>")]
pub async fn get_survey_response(
    mut db: Connection<Db>,
    id: i32,
) -> Result<Json<SurveyResponse>, NotFound<String>> {
    schema::survey_responses::dsl::survey_responses
        .find(id)
        .first::<orm::SurveyResponse>(&mut db)
        .await
        .map(|survey_response| {
            return Json(SurveyResponse::from_orm(survey_response));
        })
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SurveyResponse {
    pub id: i32,
    pub survey_id: i32,
    pub date: chrono::NaiveDateTime,
    pub question: String,
}

impl SurveyResponse {
    fn from_orm(orm: orm::SurveyResponse) -> Self {
        Self {
            id: orm.id,
            survey_id: orm.survey_id,
            date: orm.date,
            question: orm.question,
        }
    }
}
