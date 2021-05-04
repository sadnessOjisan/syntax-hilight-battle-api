#[macro_use]
extern crate diesel;

use actix_web::{error, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use model::{Post, NewPost, Results, Hilights};

mod model;
mod schema;

#[derive(Deserialize)]
struct Save {
    winner_id: i32,
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) {
    use schema::posts;
    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table).values(&new_post).execute(conn);
}

fn save_result<'a>(conn: &MysqlConnection, winner_id: &'a i64, loser_id: &'a i64) {
    use schema::results;
    let results = Results {
        winner_id: winner_id,
        loser_id: loser_id,
    };

    diesel::insert_into(results::table).values(&results).execute(conn);
}

fn get_masters(conn: &MysqlConnection)-> Vec<Hilights> {
   let res =  schema::hilights::dsl::hilights
         .load::<Hilights>(conn)
        .expect("Error loading users");
        res
}

#[post("/save")]
async fn greet(save: web::Json<Save>) -> impl Responder {
    let connection = establish_connection();
    create_post(&connection, "hoge"," fuga");
    HttpResponse::Ok().body(format!("Hello {}!", save.winner_id))
}

#[get("/battle")]
async fn battle() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello A!"))
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(battle).app_data(
            web::JsonConfig::default()
                // register error_handler for JSON extractors.
                .error_handler(json_error_handler),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
