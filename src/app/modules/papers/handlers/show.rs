use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::paper_resource::services::repository as pr_repository;

use crate::app::modules::papers::model::PaperComplete;
use crate::app::modules::papers::services::repository as paper_repository;

pub async fn get_show_admin(fetch: &State<Fetch>, db: &Db, _admin: UserInClaims, paper_id: i32) -> Result<Json<PaperComplete>, Status> {
    let paper = paper_repository::get_by_id(&db, paper_id).await;

    let paper = match paper {
        Ok(paper) => paper,
        Err(_) => return Err(Status::NotFound),
    };

    let resource = pr_repository::get_resource_by_id(fetch, paper_id).await;

    let resource = match resource {
        Ok(resource) => resource,
        Err(_) => return Err(Status::NotFound),
    };

    let complete = PaperComplete {
        id: paper.id,
        user_id: paper.user_id,
        project_id: paper.project_id,
        completed: paper.completed,
        resource,
        answers: None
    };

    Ok(Json(complete))
}
