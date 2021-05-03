use actix_web::{
    error, post, get,web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Save {
    winner_id: i32,
}

#[post("/save")]
async fn greet(save: web::Json<Save>) -> impl Responder {
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
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}