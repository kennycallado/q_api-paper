use rocket::http::Status;
use rocket::State;

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::helpers::fetch::Fetch;

use crate::app::providers::interfaces::answer::PubAnswer;

pub async fn get_answer_by_ids(fetch: &State<Fetch>, ids: Vec<i32>) -> Result<Vec<PubAnswer>, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let answer_url = ConfigGetter::get_entity_url("answer").unwrap_or("http://localhost:8012/api/v1/answer".to_string())
        + "/show/multiple";

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
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<Vec<PubAnswer>>().await.unwrap())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}
