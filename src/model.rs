use crate::schema::posts;
use crate::schema::results;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Hilights {
    pub id: u64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "results"]
pub struct Results<'a> {
    pub winner_id: &'a i64,
    pub loser_id: &'a i64,
}