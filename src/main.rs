mod config;
mod db;
mod handlers;
mod models;

use actix_web::{get, middleware, web, App, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;
use log::info;
use models::AppState;
use std::io;
use tokio_postgres::NoTls;

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let cfg = crate::config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(NoTls).unwrap();
    info!(
        "Starting server at http://{}:{}",
        cfg.server.host, cfg.server.port
    );
    HttpServer::new(move || {
        App::new()
            .data(AppState { pool: pool.clone() })
            .wrap(middleware::Logger::default())
            .service(hello)
            .route("/todos{_:/?}", web::get().to(handlers::todos))
            .route("/todos{_:/?}", web::post().to(handlers::create_todo))
            .route("/todos/{list_id}{_:/?}", web::get().to(handlers::todo))
    })
    .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
    .run()
    .await
}
