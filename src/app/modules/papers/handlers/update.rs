use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::papers::model::{NewPaper, Paper};
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn put_update_admin(
    db: &Db,
    _admin: UserInClaims,
    id: i32,
    paper: NewPaper,
) -> Result<Json<Paper>, Status> {
    let paper = paper_repository::update(db, id, paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_find_admin(
    db: &Db,
    _admin: UserInClaims,
    new_paper: NewPaper,
) -> Result<Json<Paper>, Status> {
    let finded = match paper_repository::find_by_project_user_resource(
        db,
        new_paper.project_id,
        new_paper.user_id,
        new_paper.resource_id,
    )
    .await
    {
        Ok(paper) => paper,
        Err(_) => return Err(Status::InternalServerError),
    };

    let paper = paper_repository::update(db, finded.id, new_paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn patch_completed_admin(
    db: &Db,
    _admin: UserInClaims,
    id: i32,
) -> Result<Status, Status> {
    let paper = paper_repository::patch_completed(db, id).await;

    match paper {
        Ok(_paper) => {
            // may be it was completed and it is not an error ??
            // if paper == 1 {
            //     Ok(Status::Ok)
            // } else {
            //     Err(Status::NotFound)
            // }
            Ok(Status::Ok)
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
