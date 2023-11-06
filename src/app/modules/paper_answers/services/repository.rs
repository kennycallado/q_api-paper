use rocket::http::Status;
use rocket::State;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;
#[cfg(feature = "db_sqlx")]
use sqlx::QueryBuilder;

use crate::app::modules::paper_answers::model::NewPaperAnswer;
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::answer::{PubAnswer, PubNewAnswer};
use crate::app::providers::services::fetch::Fetch;
use crate::database::connection::Db;

pub async fn get_answer_ids_by_paper_id(
    db: &Db,
    paper_id: i32,
) -> Result<Vec<i32>, sqlx::Error> {
    // let answer_ids = db
    //     .run(move |conn| {
    //         paper_answers::table
    //             .filter(paper_answers::paper_id.eq(paper_id))
    //             .select(paper_answers::answer_id)
    //             .load::<i32>(conn)
    //     })
    //     .await;
    let answer_ids = sqlx::query!("SELECT answer_id FROM paper_answers WHERE paper_id = $1", paper_id)
        .fetch_all(&db.0)
        .await?
        .into_iter()
        .map(|answer| answer.answer_id)
        .collect::<Vec<i32>>();

    Ok(answer_ids)
}

pub async fn get_answer_by_ids(
    fetch: &State<Fetch>,
    ids: Vec<i32>,
) -> Result<Vec<PubAnswer>, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let answer_url = ConfigGetter::get_entity_url("answer")
        .unwrap_or("http://localhost:8012/api/v1/answer/".to_string())
        + "show/multiple";

    let res;
    {
        res = fetch.client.lock().await
            .post(answer_url)
            .header("Accept", "application/json")
            .header("Authorization", robot_token)
            .header("Content-Type", "application/json")
            .json(&ids)
            .send()
            .await;
    }

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<Vec<PubAnswer>>().await.unwrap())
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn send_answers(
    fetch: &State<Fetch>,
    answers: Vec<PubNewAnswer>,
) -> Result<Vec<i32>, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let answer_url = ConfigGetter::get_entity_url("answer")
        .unwrap_or("http://localhost:8012/api/v1/answer/".to_string())
        + "multiple";
    // + "/create/multiple";

    let res;
    {
        res = fetch.client.lock().await
            .post(answer_url)
            .header("Accept", "application/json")
            .header("Authorization", robot_token)
            .header("Content-Type", "application/json")
            .json(&answers)
            .send()
            .await;
    }

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }
            let answers = res.json::<Vec<PubAnswer>>().await.unwrap();

            Ok(answers
                .into_iter()
                .map(|answer| answer.id)
                .collect::<Vec<i32>>())
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn add_answers(
    db: &Db,
    paper_id: i32,
    answers: Vec<i32>,
) -> Result<usize, sqlx::Error> {
    let new_answers = answers
        .into_iter()
        .map(|answer_id| NewPaperAnswer {
            paper_id,
            answer_id,
        })
        .collect::<Vec<NewPaperAnswer>>();

    // let answers_inserted = db
    //     .run(move |conn| {
    //         diesel::insert_into(paper_answers::table)
    //             .values(new_answers)
    //             .execute(conn)
    //     })
    //     .await?;

    // " ON CONFLICT (paper_id, answer_id) DO NOTHING",

    let mut query_builder = QueryBuilder::new("INSERT INTO paper_answers (paper_id, answer_id) VALUES ");

    let query = query_builder
        .push_values(new_answers, |mut separator, new_answer| {
            separator
                .push_bind(new_answer.paper_id)
                .push_bind(new_answer.answer_id);
        })
        .push(" ON CONFLICT (paper_id, answer_id) DO NOTHING")
        .push(" RETURNING *")
        .build();
    
    let answers_inserted = query
        .execute(&db.0)
        .await?
        .rows_affected();

    Ok(answers_inserted as usize)
}
