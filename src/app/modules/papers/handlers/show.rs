use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::models::answer::PubNewAnswer;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::paper_answers::services::repository as pa_repository;
use crate::app::modules::paper_resource::services::repository as pr_repository;
use crate::app::modules::papers::services::repository as paper_repository;

use crate::app::modules::papers::model::{PaperComplete, PaperPush};

pub async fn get_show_admin(fetch: &State<Fetch>, db: &Db, _admin: UserInClaims, paper_id: i32) -> Result<Json<PaperComplete>, Status> {
    let paper = paper_repository::get_by_id(&db, paper_id).await;

    let paper = match paper {
        Ok(paper) => paper,
        Err(_) => return Err(Status::NotFound),
    };

    let resource = pr_repository::get_resource_by_id(fetch, paper.resource_id).await;

    let resource = match resource {
        Ok(resource) => resource,
        Err(_) => return Err(Status::NotFound),
    };

    let answers = match pa_repository::get_answer_ids_by_paper_id(&db, paper.id).await {
        Ok(answer_ids) => {
            let answers = match pa_repository::get_answer_by_ids(fetch, answer_ids).await {
                Ok(answers) => answers,
                Err(_) => return Err(Status::InternalServerError),
            };

            Some(answers)
        },
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

pub async fn get_index_user_paper(fetch: &State<Fetch>, db: &Db, _user: UserInClaims, paper_id: i32)
-> Result<Json<Vec<PaperPush>>, Status> {
    let papers = match paper_repository::last_paper_for_all_users_where_project_id(&db, paper_id).await {
        Ok(paper) => paper,
        Err(_) => return Err(Status::NotFound),
    };

    let mut papers_push = Vec::new();
    for paper in papers {
        let answers = match pa_repository::get_answer_ids_by_paper_id(&db, paper.id).await {
            Ok(answer_ids) => {
                let answers = match pa_repository::get_answer_by_ids(fetch, answer_ids).await {
                    Ok(answers) => {
                        let new_answers = answers.into_iter().map(|answer| {
                            PubNewAnswer {
                                question_id: answer.question_id,
                                answer: answer.answer,
                            }
                        }).collect::<Vec<PubNewAnswer>>();

                        Some(new_answers)
                    },
                    Err(_) => None,
                };

                answers
            },
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
