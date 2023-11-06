use serde::{Deserialize, Serialize};

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

use crate::app::providers::models::answer::{PubAnswer, PubNewAnswer};
use crate::app::providers::models::resource::PubResource;

#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Paper {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub resource_id: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPaper {
    pub user_id: i32,
    pub project_id: i32,
    pub resource_id: i32,
    pub completed: Option<bool>,
}

impl From<Paper> for NewPaper {
    fn from(paper: Paper) -> Self {
        NewPaper {
            user_id: paper.user_id,
            project_id: paper.project_id,
            resource_id: paper.resource_id,
            completed: Some(paper.completed),
        }
    }
}

impl From<PaperPush> for NewPaper {
    fn from(value: PaperPush) -> Self {
        NewPaper {
            user_id: value.user_id,
            project_id: value.project_id,
            resource_id: value.resource_id,
            completed: Some(value.completed),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperComplete {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub completed: bool,
    pub resource: PubResource,
    pub answers: Option<Vec<PubAnswer>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperPush {
    pub id: i32,
    pub user_id: i32,
    pub user_record: rocket::serde::json::Value,
    pub project_id: i32,
    pub resource_id: i32,
    pub completed: bool,
    pub answers: Option<Vec<PubNewAnswer>>,
}
