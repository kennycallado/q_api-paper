use serde::{Deserialize, Serialize};

use crate::database::schema::paper_answers;

use crate::app::modules::papers::model::Paper;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Paper))]
#[diesel(table_name = paper_answers)]
#[serde(crate = "rocket::serde")]
pub struct PaperAnswer {
    pub id: i32,
    pub paper_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(belongs_to(Paper))]
#[diesel(table_name = paper_answers)]
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
