use rocket::http::Status;
use rocket::State;

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::helpers::fetch::Fetch;

use crate::app::providers::interfaces::resource::PubResource;

pub async fn get_resource_by_id(fetch: &State<Fetch>, id: i32) -> Result<PubResource, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Prepare url
    let resource_url = ConfigGetter::get_entity_url("resource").unwrap_or("http://localhost:8031/api/v1/resource".to_string())
        + "/"
        + id.to_string().as_str();

    // Request
    let client = fetch.client.lock().await;
    let res = client
        .get(resource_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let resource = match res.json::<PubResource>().await {
                    Ok(resource) => resource,
                    Err(_) => return Err(Status::InternalServerError),
                };

                Ok(resource)
            } else {
                Err(Status::InternalServerError)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}