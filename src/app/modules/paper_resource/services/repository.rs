use rocket::http::Status;
use rocket::State;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::resource::PubResource;
use crate::app::providers::services::fetch::Fetch;

pub async fn get_resource_by_id(fetch: &State<Fetch>, id: i32) -> Result<PubResource, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(e) => {
            println!("Error: {}; get_resource_by_id(); getting robot_token", e);
            return Err(Status::InternalServerError);
        }
    };

    let mut resource_url = match ConfigGetter::get_entity_url("resource") {
        Some(url) => url,
        None => {
            println!("Error: ; get_resource_by_id(); getting resource_url");
            return Err(Status::InternalServerError);
        }
    };

    resource_url = resource_url + id.to_string().as_str();

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .get(resource_url)
            .header("Accept", "application/json")
            .header("Authorization", robot_token)
            .send()
            .await;
    }

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let resource = match res.json::<PubResource>().await {
                    Ok(resource) => resource,
                    Err(e) => {
                        println!("Error: {}; get_resource_by_id(); parsing json", e);
                        return Err(Status::InternalServerError);
                    }
                };

                Ok(resource)
            } else {
                println!("Error: {}; get_resource_by_id(); status code", res.status());
                Err(Status::InternalServerError)
            }
        }
        Err(e) => {
            println!("Error: {}; get_resource_by_id(); fetching resource", e);
            Err(Status::InternalServerError)
        }
    }
}
