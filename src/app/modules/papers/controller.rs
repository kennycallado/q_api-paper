use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::connection::Db;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::papers::handlers::{create, index, show, update};
use crate::app::modules::papers::model::{NewPaper, Paper, PaperComplete, PaperPush};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_index_user,
        get_index_user_none,
        get_show,
        get_show_none,
        get_lasts_admin,
        get_lasts_admin_none,
        post_create,
        post_create_none,
        post_show_create,
        post_show_create_none,
        put_index_update,
        put_index_update_none,
        put_update,
        put_update_none,
        patch_completed,
        patch_completed_none,
    ]
}

#[options("/<_..>")]
pub fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: &Db, claims: AccessClaims) -> Result<Json<Vec<Paper>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/user/<id>", rank = 101)]
pub async fn get_index_user(
    db: &Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<Vec<Paper>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" |
        "user" => index::get_index_user_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_index_user; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/user/<_id>", rank = 102)]
pub async fn get_index_user_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(
    fetch: &State<Fetch>,
    db: &Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<PaperComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" |
        "user" => show::get_show_admin(fetch, db, claims.0.user, id).await,
        // TODO: manejo permisos
        _ => {
            println!(
                "Error: get_show; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/project/<id>/lasts", rank = 1)]
pub async fn get_lasts_admin(
    fetch: &State<Fetch>,
    db: &Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<Vec<PaperPush>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => show::get_index_user_paper(fetch, db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_index_user_paper; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/project/<_id>/lasts", rank = 2)]
pub async fn get_lasts_admin_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_paper>", rank = 1)]
pub async fn post_create(
    db: &Db,
    claims: AccessClaims,
    new_paper: Json<NewPaper>,
) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => create::post_create_admin(db, claims.0.user, new_paper.into_inner()).await,
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_new_paper>", rank = 2)]
pub async fn post_create_none(_new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}

#[post("/<id>", data = "<paper>", rank = 101)]
pub async fn post_show_create(
    fetch: &State<Fetch>,
    db: &Db,
    claims: AccessClaims,
    id: i32,
    paper: Json<PaperPush>,
) -> Result<rocket::serde::json::Value, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" |
        "user" => {
            create::post_show_admin(fetch, db, claims.0.user, id, paper.into_inner()).await
        }
        _ => {
            println!(
                "Error: post_show_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[post("/<_id>", data = "<_paper>", rank = 102)]
pub async fn post_show_create_none(_id: i32, _paper: Json<PaperPush>) -> Status {
    Status::Unauthorized
}

#[put("/", data = "<new_paper>", rank = 1)]
pub async fn put_index_update(
    db: &Db,
    claims: AccessClaims,
    new_paper: Json<NewPaper>,
) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => {
            update::put_update_find_admin(db, claims.0.user, new_paper.into_inner()).await
        }
        _ => {
            println!(
                "Error: put_update; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[put("/", data = "<_new_paper>", rank = 2)]
pub async fn put_index_update_none(_new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_paper>", rank = 101)]
pub async fn put_update(
    db: &Db,
    claims: AccessClaims,
    id: i32,
    new_paper: Json<NewPaper>,
) -> Result<Json<Paper>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => {
            update::put_update_admin(db, claims.0.user, id, new_paper.into_inner()).await
        }
        _ => {
            println!(
                "Error: put_update; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_paper>", rank = 102)]
pub async fn put_update_none(_id: i32, _new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}

#[patch("/<id>/completed", rank = 1)]
pub async fn patch_completed(db: &Db, claims: AccessClaims, id: i32) -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => update::patch_completed_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: patch_completed; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[patch("/<_id>/completed", rank = 2)]
pub async fn patch_completed_none(_id: i32) -> Status {
    Status::Unauthorized
}
