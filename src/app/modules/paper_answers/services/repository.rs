use diesel::prelude::*;

use rocket::http::Status;
use rocket::State;

use crate::database::connection::Db;
use crate::database::schema::paper_answers;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::answer::{PubAnswer, PubNewAnswer};
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::paper_answers::model::NewPaperAnswer;
 
pub async fn get_answer_ids_by_paper_id(db: &Db, paper_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
    let answer_ids = db
            .run(move |conn| paper_answers::table
                .filter(paper_answers::paper_id.eq(paper_id))
                .select(paper_answers::answer_id)
                .load::<i32>(conn)
            ).await;

    answer_ids
}

pub async fn get_answer_by_ids(fetch: &State<Fetch>, ids: Vec<i32>) -> Result<Vec<PubAnswer>, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let answer_url = ConfigGetter::get_entity_url("answer").unwrap_or("http://localhost:8012/api/v1/answer/".to_string())
        + "show/multiple";

    let client = fetch.client.lock().await;
    let res = client
        .post(answer_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&ids)
        .send()
        .await;

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

pub async fn send_answers(fetch: &State<Fetch>, answers: Vec<PubNewAnswer>) -> Result<Vec<i32>, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let answer_url = ConfigGetter::get_entity_url("answer").unwrap_or("http://localhost:8012/api/v1/answer/".to_string())
        + "multiple";
        // + "/create/multiple";

    let client = fetch.client.lock().await;
    let res = client
        .post(answer_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&answers)
        .send()
        .await;

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }
            let answers = res.json::<Vec<PubAnswer>>().await.unwrap();

            Ok(answers.into_iter().map(|answer| answer.id).collect::<Vec<i32>>())
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn add_answers(db: &Db, paper_id: i32, answers: Vec<i32>) -> Result<usize, diesel::result::Error> {
    let new_answers = answers.into_iter().map(|answer_id| NewPaperAnswer {
        paper_id,
        answer_id,
    }).collect::<Vec<NewPaperAnswer>>();

    let answers_inserted = db.run(move |conn| {
        diesel::insert_into(paper_answers::table)
            .values(new_answers)
            .execute(conn)
    }).await?;

    Ok(answers_inserted)
}
