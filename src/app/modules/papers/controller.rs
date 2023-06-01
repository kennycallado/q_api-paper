use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::fetch::Fetch;

use crate::app::providers::guards::claims::AccessClaims;
use crate::config::database::Db;

use crate::app::modules::papers::handlers::{create, index, show, update};
use crate::app::modules::papers::model::{Paper, NewPaper, PaperComplete, PaperPush};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        get_last_admin,

        post_create,
        post_create_none,
        post_show_create,
        post_show_create_none,
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
pub async fn get_show(fetch: &State<Fetch>, db: Db, claims: AccessClaims, id: i32) -> Result<Json<PaperComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(fetch, &db, claims.0.user, id).await,
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

#[get("/last", rank = 1)]
pub async fn get_last_admin(fetch: &State<Fetch>, db: Db, 
    // claims: AccessClaims
) -> Result<Json<Vec<PaperPush>>, Status> {

    show::get_index_user_paper(fetch, &db, /*claims.0.user,*/ 1).await
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

#[post("/<id>", data = "<paper>", rank = 101)]
pub async fn post_show_create(fetch: &State<Fetch>, db: Db, claims: AccessClaims, id: i32, paper: Json<PaperPush>) -> Result<rocket::serde::json::Value, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_show_admin(fetch, &db, claims.0.user, id, paper.into_inner()).await,
        "user" => Ok(rocket::serde::json::json!({ "message": "Not implemented" })),
        _ => {
            println!("Error: post_show_create; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
    
}

#[post("/<_id>", data = "<_paper>", rank = 102)]
pub async fn post_show_create_none(_id: i32, _paper: Json<PaperPush>) -> Status {
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
