use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::config::database::Db;

use crate::app::modules::papers::handlers::{create, index, show, update};
use crate::app::modules::papers::model::{Paper, NewPaper};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        get_index,
        get_index_none,
    ]
}

#[options("/")]
pub fn options_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub fn options_show(_id: i32) -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Paper>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(&db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}
