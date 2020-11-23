extern crate askama;
extern crate dotenv;
extern crate pretty_env_logger;
extern crate rbatis;
#[macro_use]
extern crate log;

use std::{env, io::Result};

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use rbatis::rbatis::Rbatis;

pub mod api;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load env
    dotenv().ok();

    // Set up logger
    pretty_env_logger::init();

    // Load env config vars
    let port = env::var("PORT").unwrap_or("8000".to_owned());
    let db_url = env::var("DATABASE_URL").expect("Database URL");

    // Initialize rbatis
    let rb = Rbatis::new();
    info!("Connecting to the database");
    rb.link(&db_url).await.expect("Database expect");
    info!("Database connected");

    // Start listening
    info!("Listening on port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(api::render_files)
            .service(api::render_files_root)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
