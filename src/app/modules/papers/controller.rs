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
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
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

#[get("/<id>", rank = 101)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(&db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_paper>", rank = 1)]
pub async fn post_create(db: Db, claims: AccessClaims, new_paper: Json<NewPaper>) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(&db, claims.0.user, new_paper.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_new_paper>", rank = 2)]
pub async fn post_create_none(_new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_paper>", rank = 101)]
pub async fn put_update(db: Db, claims: AccessClaims, id: i32, new_paper: Json<NewPaper>) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(&db, claims.0.user, id, new_paper.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_paper>", rank = 102)]
pub async fn put_update_none(_id: i32, _new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}