use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::papers::model::Paper;
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn get_index_admin(db: &Db, _admin: UserInClaims) -> Result<Json<Vec<Paper>>, Status> {
    let papers = paper_repository::get_all(db).await;

    match papers {
        Ok(papers) => Ok(Json(papers)),
        Err(_) => Err(Status::BadRequest)
    }
}
