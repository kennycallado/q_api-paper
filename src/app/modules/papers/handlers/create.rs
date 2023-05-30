use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::papers::model::{Paper, NewPaper, PaperPush};

use crate::app::modules::papers::services::repository as paper_repository;
use crate::app::modules::paper_answers::services::repository as pa_repository;

use super::helper;

pub async fn post_create_admin(db: &Db, _admin: UserInClaims, new_paper: NewPaper) -> Result<Json<Paper>, Status> {
    let paper = paper_repository::create(&db, new_paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::NotFound),
    }
}

pub async fn post_show_admin(fetch: &State<Fetch>, db: &Db, _admin: UserInClaims, _id: i32, paper_push: PaperPush) -> Result<rocket::serde::json::Value, Status> {
    // Save the answers
    if paper_push.answers.is_some() {
        match pa_repository::send_answers(fetch, paper_push.answers.clone().unwrap()).await {
            Ok(answer_ids) => {
                match pa_repository::add_answers(&db, paper_push.id, answer_ids).await {
                    Ok(_answers_inserted) => { },
                    Err(_) => return Err(Status::InternalServerError),
                }
            }
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    // Send the paper to the logics_api
    let logic_response = match helper::send_to_logic(fetch, &paper_push).await {
        Ok(res) => res, 
        Err(status) => return Err(status),
    };

    let user_record = logic_response.user_record.clone();

    // Update the paper in the database
    match paper_repository::update(&db, paper_push.id, logic_response.into()).await {
        Ok(_) => Ok(rocket::serde::json::json!({ "user_record": user_record })),
        Err(_) => Err(Status::InternalServerError),
    }
}
