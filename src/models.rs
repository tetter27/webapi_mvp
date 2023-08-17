use super::schema::*;
use serde::{Serialize, Deserialize};

// projectsテーブル用
#[derive(Queryable, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub url_name: String
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
}

// technologiesテーブル用
#[derive(Queryable, Serialize, Deserialize)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub url_name: String,
    pub image_url: String
}

#[derive(Insertable)]
#[table_name = "technologies"]
pub struct NewTechnology<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub image_url: &'a str,
}

// adoptsテーブル用
#[derive(Queryable)]
pub struct Adopt {
    pub id: i64,
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "adopts"]
pub struct NewAdopt {
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}