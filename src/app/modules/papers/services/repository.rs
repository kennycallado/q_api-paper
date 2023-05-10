use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::papers;

use crate::app::modules::papers::model::{Paper, NewPaper};

pub async fn get_all(db: &Db) -> Result<Vec<Paper>, diesel::result::Error> {
    let papers = db.run(|conn| papers::table.load::<Paper>(conn)).await;

    papers
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Paper, diesel::result::Error> {
    let paper = db.run(move |conn| papers::table.find(id).first::<Paper>(conn)).await;

    paper
}

pub async fn create(db: &Db, new_paper: NewPaper) -> Result<Paper, diesel::result::Error> {
    let paper = db.run(move |conn| {
        diesel::insert_into(papers::table).values(&new_paper).get_result::<Paper>(conn)
    }).await;

    paper
}

pub async fn update(db: &Db, id: i32, new_paper: NewPaper) -> Result<Paper, diesel::result::Error> {
    let paper = db.run(move |conn| {
        diesel::update(papers::table.find(id)).set(&new_paper).get_result::<Paper>(conn)
    }).await;

    paper
}