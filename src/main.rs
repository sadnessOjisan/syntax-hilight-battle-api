#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{
    error, get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use model::{Hilights, NewPost, Post, Results};
use serde::{Deserialize, Serialize};
use std::env;

mod model;
mod schema;

#[derive(Deserialize)]
struct Save {
    winner_id: i64,
    looser_id: i64,
}

#[derive(Serialize)]
struct Hilight {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct HilightResponse {
    data: Vec<Hilight>,
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

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn);
}

fn save_result<'a>(conn: &MysqlConnection, winner_id: &'a i64, loser_id: &'a i64) {
    use schema::results;
    let results = Results {
        winner_id: winner_id,
        loser_id: loser_id,
    };

    diesel::insert_into(results::table)
        .values(&results)
        .execute(conn);
}

fn get_masters(conn: &MysqlConnection) -> Vec<Hilight> {
    let mut hilights = Vec::new();
    let res = schema::hilights::dsl::hilights
        .load::<Hilights>(conn)
        .expect("Error loading users");
    for r in res {
        hilights.push(Hilight {
            id: r.id,
            name: r.name,
        });
    }
    hilights
}

#[post("/save")]
async fn save(save: web::Json<Save>) -> impl Responder {
    let conn = establish_connection();
    save_result(&conn, &save.winner_id, &save.looser_id);
    HttpResponse::Ok()
        .content_type("application/json")
        .body("ok")
}

#[get("/battle")]
async fn battle() -> impl Responder {
    let connection = establish_connection();
    let data = get_masters(&connection);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
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
        let cors = Cors::default()
            .allowed_origin("https://www.rust-lang.org/")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(save)
            .service(battle)
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(json_error_handler),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
