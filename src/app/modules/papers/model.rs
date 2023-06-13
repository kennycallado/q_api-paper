use serde::{Deserialize, Serialize};

use crate::app::providers::models::resource::PubResource;
use crate::app::providers::models::answer::{PubAnswer, PubNewAnswer};

use crate::database::schema::papers;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Paper {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub resource_id: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = papers)]
#[serde(crate = "rocket::serde")]
pub struct NewPaper {
    pub user_id: i32,
    pub project_id: i32,
    pub resource_id: i32,
}

impl From<Paper> for NewPaper {
    fn from(paper: Paper) -> Self {
        NewPaper {
            user_id: paper.user_id,
            project_id: paper.project_id,
            resource_id: paper.resource_id,
        }
    }
}

impl From<PaperPush> for NewPaper {
    fn from(value: PaperPush) -> Self {
        NewPaper {
            user_id: value.user_id,
            project_id: value.project_id,
            resource_id: value.resource_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperComplete {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
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
