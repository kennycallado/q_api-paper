use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::papers::model::Paper;
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn get_index_admin(
    db: &Db,
    _admin: UserInClaims,
) -> Result<Json<Vec<Paper>>, Status> {
    let papers = paper_repository::get_all(db).await;

    match papers {
        Ok(papers) => Ok(Json(papers)),
        Err(_) => Err(Status::NotFound),
    }
}

pub async fn get_index_user_admin(
    db: &Db,
    _user: UserInClaims,
    user_id: i32,
) -> Result<Json<Vec<Paper>>, Status> {
    let papers = paper_repository::get_all_by_user_id(db, user_id).await;

    match papers {
        Ok(papers) => Ok(Json(papers)),
        Err(_) => Err(Status::NotFound),
    }
}
