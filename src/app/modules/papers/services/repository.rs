use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::papers;

use crate::app::modules::papers::model::{Paper, NewPaper};

pub async fn get_all(db: &Db) -> Result<Vec<Paper>, diesel::result::Error> {
    let papers = db.run(move |conn| papers::table.load::<Paper>(conn)).await;

    papers
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Paper, diesel::result::Error> {
    let paper = db.run(move |conn| papers::table.find(id).first::<Paper>(conn)).await;

    paper
}

pub async fn get_all_by_user_id(db: &Db, user_id: i32) -> Result<Vec<Paper>, diesel::result::Error> {
    let papers = db.run(move |conn| {
        papers::table
            .filter(papers::user_id.eq(user_id))
            .load::<Paper>(conn)
    }).await;

    papers
}

pub async fn find_by_project_user_resource(db: &Db, project_id: i32, user_id: i32, resource_id: i32)
    -> Result<Paper, diesel::result::Error> {
    let paper = db.run(move |conn| {
        papers::table
            .filter(papers::project_id.eq(project_id))
            .filter(papers::user_id.eq(user_id))
            .filter(papers::resource_id.eq(resource_id))
            .first::<Paper>(conn)
    }).await;

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

pub async fn patch_completed(db: &Db, id: i32) -> Result<usize, diesel::result::Error> {
    let paper = db.run(move |conn| {
        diesel::update(
            papers::table.find(id))
            .set(papers::completed.eq(true))
            .execute(conn)
    }).await;

    paper
}

pub async fn last_paper_for_all_users_where_project_id(db: &Db, project_id: i32) -> Result<Vec<Paper>, diesel::result::Error> {
    let papers = db.run(move |conn| {
        papers::table
            .select(papers::all_columns)
            .distinct_on(papers::user_id)
            .filter(papers::project_id.eq(project_id))
            // .filter(papers::completed.eq(true))
            .order(papers::user_id.desc())
            .load::<Paper>(conn)
    }).await;

    papers
}
