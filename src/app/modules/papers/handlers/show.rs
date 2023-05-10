use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::papers::model::Paper;
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, paper_id: i32) -> Result<Json<Paper>, Status> {
    let paper = paper_repository::get_by_id(&db, paper_id).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::NotFound),
    }
}
