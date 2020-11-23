extern crate dotenv;
extern crate pretty_env_logger;

use std::{env, io::Result};

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[macro_use] extern crate log;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    let port = env::var("PORT").unwrap_or("8000".to_owned());

    info!("Starting fso2 on port {}", port);
    
    HttpServer::new(|| App::new().service(health))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
