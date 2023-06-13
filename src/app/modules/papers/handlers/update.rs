use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::papers::model::{Paper, NewPaper};
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn put_update_admin(db: &Db, _admin: UserInClaims, id: i32, paper: NewPaper) -> Result<Json<Paper>, Status> {
    let paper = paper_repository::update(db, id, paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::InternalServerError),
    }
}
