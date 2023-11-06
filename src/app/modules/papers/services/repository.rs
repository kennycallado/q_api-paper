#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::papers::model::{NewPaper, Paper};
use crate::database::connection::Db;

pub async fn get_all(db: &Db) -> Result<Vec<Paper>, sqlx::Error> {
    // let papers = db.run(move |conn| papers::table.load::<Paper>(conn)).await;
    let papers = sqlx::query_as!(Paper, "SELECT * FROM papers")
        .fetch_all(&db.0)
        .await?;

    Ok(papers)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Paper, sqlx::Error> {
    // let paper = db
    //     .run(move |conn| papers::table.find(id).first::<Paper>(conn))
    //     .await;
    let paper = sqlx::query_as!(Paper, "SELECT * FROM papers WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    Ok(paper)
}

pub async fn get_all_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<Vec<Paper>, sqlx::Error> {
    // let papers = db
    //     .run(move |conn| {
    //         papers::table
    //             .filter(papers::user_id.eq(user_id))
    //             .load::<Paper>(conn)
    //     })
    //     .await;
    let papers = sqlx::query_as!(
        Paper,
        "SELECT * FROM papers WHERE user_id = $1",
        user_id
    ).fetch_all(&db.0).await?;

    Ok(papers)
}

pub async fn find_by_project_user_resource(
    db: &Db,
    project_id: i32,
    user_id: i32,
    resource_id: i32,
) -> Result<Paper, sqlx::Error> {
    // let paper = db
    //     .run(move |conn| {
    //         papers::table
    //             .filter(papers::project_id.eq(project_id))
    //             .filter(papers::user_id.eq(user_id))
    //             .filter(papers::resource_id.eq(resource_id))
    //             .first::<Paper>(conn)
    //     })
    //     .await;
    let paper = sqlx::query_as!(
        Paper,
        "SELECT * FROM papers WHERE project_id = $1 AND user_id = $2 AND resource_id = $3",
        project_id,
        user_id,
        resource_id
    ).fetch_one(&db.0).await?;

    Ok(paper)
}

pub async fn create(db: &Db, new_paper: NewPaper) -> Result<Paper, sqlx::Error> {
    // let paper = db
    //     .run(move |conn| {
    //         diesel::insert_into(papers::table)
    //             .values(&new_paper)
    //             .get_result::<Paper>(conn)
    //     })
    //     .await;
    let paper = sqlx::query_as!(
        Paper,
        "INSERT INTO papers (project_id, user_id, resource_id, completed) VALUES ($1, $2, $3, $4) RETURNING *",
        new_paper.project_id,
        new_paper.user_id,
        new_paper.resource_id,
        new_paper.completed
    ).fetch_one(&db.0).await?;

    Ok(paper)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_paper: NewPaper,
) -> Result<Paper, sqlx::Error> {
    // let paper = db
    //     .run(move |conn| {
    //         diesel::update(papers::table.find(id))
    //             .set(&new_paper)
    //             .get_result::<Paper>(conn)
    //     })
    //     .await;
    let paper = sqlx::query_as!(
        Paper,
        "UPDATE papers SET project_id = $1, user_id = $2, resource_id = $3, completed = $4 WHERE id = $5 RETURNING *",
        new_paper.project_id,
        new_paper.user_id,
        new_paper.resource_id,
        new_paper.completed,
        id
    ).fetch_one(&db.0).await?;

    Ok(paper)
}

pub async fn patch_completed(db: &Db, id: i32) -> Result<usize, sqlx::Error> {
    // let paper = db
    //     .run(move |conn| {
    //         diesel::update(papers::table.find(id))
    //             .set(papers::completed.eq(true))
    //             .execute(conn)
    //     })
    //     .await;
    let paper = sqlx::query!(
        "UPDATE papers SET completed = true WHERE id = $1",
        id
    ).execute(&db.0).await?;

    Ok(paper.rows_affected() as usize)
}

pub async fn last_paper_for_all_users_where_project_id(
    db: &Db,
    project_id: i32,
) -> Result<Vec<Paper>, sqlx::Error> {
    // let papers = db
    //     .run(move |conn| {
    //         // .filter(papers::completed.eq(true))
    //         papers::table
    //             .select(papers::all_columns)
    //             .distinct_on(papers::user_id)
    //             .filter(papers::project_id.eq(project_id))
    //             .order((papers::user_id, papers::id.desc()))
    //             .load::<Paper>(conn)
    //     })
    //     .await;
    let papers = sqlx::query_as!(
        Paper,
        "SELECT DISTINCT ON (user_id) * FROM papers WHERE project_id = $1 ORDER BY user_id, id DESC",
        project_id
    ).fetch_all(&db.0).await?;

    Ok(papers)
}
