use serde::{Deserialize, Serialize};

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

