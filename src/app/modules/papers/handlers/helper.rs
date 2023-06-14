use rocket::http::Status;
use rocket::State;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::papers::model::PaperPush;

pub async fn send_to_logic(fetch: &State<Fetch>, paper: &PaperPush) -> Result<PaperPush, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let logic_url = ConfigGetter::get_entity_url("logic").unwrap_or("http://localhost:8041/api/v1/logic/".to_string())
        + "checker/push";

    let client = fetch.client.lock().await;
    let res = client
        .post(logic_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(paper)
        .send()
        .await;

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<PaperPush>().await.unwrap())
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
