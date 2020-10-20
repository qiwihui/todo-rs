mod config;
mod db;
mod handlers;
mod models;

use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use handlers::todos;
use std::io;
use tokio_postgres::{self, NoTls};

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let cfg = crate::config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(NoTls).unwrap();
    println!(
        "Starting server at http://{}:{}",
        cfg.server.host, cfg.server.port
    );
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .route("/todos{_:/?}", web::get().to(todos))
    })
    .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
    .run()
    .await
}
