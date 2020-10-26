mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;

use crate::config::{init_pool, Config};
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use models::AppState;
use routes::routes;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let cfg = Config::from_env().unwrap();
    let pool = init_pool(&cfg).unwrap();
    info!(
        "Starting server at http://{}:{}",
        cfg.server.host, cfg.server.port
    );
    HttpServer::new(move || {
        App::new()
            .data(AppState { pool: pool.clone() })
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
    .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
    .run()
    .await
}
