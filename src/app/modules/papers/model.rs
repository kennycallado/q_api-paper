use serde::{Deserialize, Serialize};

use crate::app::providers::interfaces::resource::PubResource;
use crate::app::providers::interfaces::answer::PubAnswer;

use crate::database::schema::papers;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Paper {
    pub id: i32,
    pub user_id: i32,
    pub resource_id: i32,
    pub project_id: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = papers)]
#[serde(crate = "rocket::serde")]
pub struct NewPaper {
    pub user_id: i32,
    pub resource_id: i32,
    pub project_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperComplete {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub completed: bool,
    pub resource: PubResource,
    pub answers: Option<Vec<PubAnswer>>,
}
