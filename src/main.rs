mod config;
mod db;
mod handlers;
mod models;

use actix_web::{get, middleware, web, App, HttpServer, Responder};
use dotenv::dotenv;
use handlers::{todo, todos};
use env_logger;
use log::info;
use std::io;
use tokio_postgres::{self, NoTls};

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
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(hello)
            .route("/todos{_:/?}", web::get().to(todos))
            .route("/todos/{list_id}{_:/?}", web::get().to(todo))
    })
    .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
    .run()
    .await
}
