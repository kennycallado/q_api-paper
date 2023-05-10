use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::papers::model::{Paper, NewPaper};
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn post_create_admin(db: &Db, _admin: UserInClaims, new_paper: NewPaper) -> Result<Json<Paper>, Status> {
    let paper = paper_repository::create(&db, new_paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::NotFound),
    }
}
