use serde::{Deserialize, Serialize};

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

use crate::app::modules::papers::model::Paper;

#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperAnswer {
    pub id: i32,
    pub paper_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPaperAnswer {
    pub paper_id: i32,
    pub answer_id: i32,
}

impl From<PaperAnswer> for NewPaperAnswer {
    fn from(paper: PaperAnswer) -> Self {
        Self {
            paper_id: paper.id,
            answer_id: paper.answer_id,
        }
    }
}
