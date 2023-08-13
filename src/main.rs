mod models;

use actix_web::{ http, web, App, HttpServer, Responder };
use crate::models::Status;
use std::io;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().status(http::StatusCode::OK).json(Status { status: "Ok".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    print!("I have started!");
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run().await
}
