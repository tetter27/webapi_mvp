use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::result::Error;

// transaction
pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    
}

// insert
pub fn insert_project (
        conn:     &PgConnection, 
        name:     &str, 
        url_name: &str)         -> Result<Project, Error> {

    let new_project = NewProject { name, url_name };
    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
}

pub fn insert_technology (
        conn:      &PgConnection, 
        name:      &str, 
        url_name:  &str, 
        image_url: &str)        -> Result<Technology, Error> {

    let new_technology = NewTechnology { name, url_name, image_url };
    diesel::insert_into(technologies::table)
        .values(&new_technology)
        .get_result(conn)
}

pub fn insert_adopt (
        conn:            &PgConnection, 
        projects_id:     i64, 
        technologies_id: i64, 
        created_at:      chrono::NaiveDateTime) -> Result<Adopt, Error> {

    let new_adopt = NewAdopt { projects_id, technologies_id, created_at };
    diesel::insert_into(adopts::table)
        .values(&new_adopt)
        .get_result(conn)
}
// ---


// get
pub fn get_technology_by_url_name (  // 1
        conn:     &PgConnection, 
        tech_url_name: &str)         -> Result<Technology, Error> {

    technologies::table
        .filter(technologies::url_name.eq(tech_url_name))
        .first(conn)
}

pub fn get_projects_by_technologies_id (  // 2 & 3
        conn:    &PgConnection, 
        tech_id: i64)           -> Result<Vec<Project>, Error> {

    projects::table
        .inner_join(adopts::table.on(projects::id.eq(adopts::projects_id)))
        .filter(adopts::technologies_id.eq(tech_id))
        .select(projects::all_columns)
        .load(conn)
}

pub fn get_technology_page_by_url_name (  // 1 -> 2 & 3
        conn:     &PgConnection, 
        tech_url_name: &str)     -> Result<(Technology, Vec<Project>), Error> {

    let tech = get_technology_by_url_name(&conn, tech_url_name)?;
    let projs = get_projects_by_technologies_id(&conn, tech.id)?;

    Ok((tech, projs))
}
// ---

// test transaction
pub fn test_transaction<F, R>(test_fn: F) -> Result<R, Error>
where
    F: FnOnce(&PgConnection) -> Result<R, Error>,
{
    let connection = create_connection();
    Ok(connection.test_transaction(|| test_fn(&connection)))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod unit_DBテスト {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn get_technology_page_by_url_name関数はTechnologyページの情報をDBから一括で取得する() {
        test_transaction(|conn| {
            let proj_name = "TestProject";
            let proj_url_name = "test_project";
            let tech_name = "TestTechnology";
            let tech_url_name = "test_technology";
            let tech_image_url = "https://example.com/";
            let dt = NaiveDate::from_ymd_opt(2016, 7, 8)
                            .unwrap().and_hms_opt(9, 10, 11).unwrap();

            // テストのためのデータ挿入
            let new_proj = insert_project(&conn, proj_name, proj_url_name)?;
            let new_tech = insert_technology(&conn, tech_name, tech_url_name, tech_image_url)?;
            let _ = insert_adopt(&conn, new_proj.id, new_tech.id, dt)?;

            // 関数のテスト実行
            let (tech, projs) = get_technology_page_by_url_name(&conn, tech_url_name)?;

            let proj = &projs[0];
            assert_eq!(tech.name, tech_name);
            assert_eq!(tech.image_url, tech_image_url);
            assert_eq!(proj.name, proj_name);
            assert_eq!(proj.url_name, proj_url_name);
    
            Ok(())
        }).unwrap(); 
    }
}