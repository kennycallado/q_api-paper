use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::modules::paper_answers::model::PaperAnswer;
use crate::database::connection::Db;

use crate::app::providers::models::answer::{PubAnswer, PubNewAnswer};
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::paper_answers::services::repository as pa_repository;
use crate::app::modules::paper_resource::services::repository as pr_repository;
use crate::app::modules::papers::services::repository as paper_repository;

use crate::app::modules::papers::model::{PaperComplete, PaperPush};

pub async fn get_show_admin(
    fetch: &State<Fetch>,
    db: &Db,
    _admin: UserInClaims,
    paper_id: i32,
) -> Result<Json<PaperComplete>, Status> {
    let paper = paper_repository::get_by_id(&db, paper_id).await;
    let paper = match paper {
        Ok(paper) => paper,
        Err(e) => {
            println!("Error: get_show_admin (paper by id); {}", e);
            return Err(Status::NotFound);
        }
    };

    let resource = pr_repository::get_resource_by_id(fetch, paper.resource_id).await;
    let resource = match resource {
        Ok(resource) => resource,
        Err(e) => {
            println!("Error: get_show_admin (resource by id);");
            return Err(e);
        }
    };

    let answers = match pa_repository::get_answer_ids_by_paper_id(&db, paper.id).await {
        Ok(answer_ids) => {
            let answers = match pa_repository::get_answer_by_ids(fetch, answer_ids).await {
                Ok(answers) => answers,
                Err(e) => {
                    println!("Error: get_show_admin (answers by ids);");
                    return Err(e);
                }
            };

            Some(answers)
        }
        Err(_) => None,
    };

    let paper_complete = PaperComplete {
        id: paper.id,
        user_id: paper.user_id,
        project_id: paper.project_id,
        completed: paper.completed,
        resource,
        answers,
    };

    Ok(Json(paper_complete))
}

pub async fn get_index_user_paper(
    fetch: &State<Fetch>,
    db: &Db,
    _user: UserInClaims,
    paper_id: i32,
) -> Result<Json<Vec<PaperPush>>, Status> {
    let papers = match paper_repository::last_paper_for_all_users_where_project_id(
        &db, paper_id,
    )
    .await
    {
        Ok(papers) => papers,
        Err(_) => return Err(Status::NotFound),
    };

    let mut papers_push = Vec::new();
    for paper in papers {
        let answers = match get_answers(fetch, &db, paper.id).await {
            Ok(answers) => Some(
                answers
                    .into_iter()
                    .map(|answer| PubNewAnswer {
                        question_id: answer.question_id,
                        answer: answer.answer,
                    })
                    .collect(),
            ),
            Err(_) => None,
        };

        let paper_push = PaperPush {
            id: paper.id,
            user_id: paper.user_id,
            user_record: rocket::serde::json::Value::Null,
            project_id: paper.project_id,
            resource_id: paper.resource_id,
            completed: paper.completed,
            answers,
        };

        papers_push.push(paper_push);
    }

    Ok(Json(papers_push))
}

async fn get_answers(
    fetch: &State<Fetch>,
    db: &Db,
    paper_id: i32,
) -> Result<Vec<PubAnswer>, Status> {
    let answer_ids = match pa_repository::get_answer_ids_by_paper_id(&db, paper_id).await {
        Ok(answer_ids) => answer_ids,
        Err(_) => return Err(Status::NotFound),
    };

    pa_repository::get_answer_by_ids(fetch, answer_ids).await
}
