mod db;
mod handlers;
mod models;

use actix_web::{get, web, App, HttpServer, Responder};
use deadpool_postgres;
use handlers::todos;
use std::io;
use tokio_postgres::{self, NoTls};

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8000");

    let mut cfg = tokio_postgres::Config::new();
    cfg.host("localhost");
    cfg.port(5432);
    cfg.user("actix");
    cfg.password("actix");
    cfg.dbname("actix");
    let mgr = deadpool_postgres::Manager::new(cfg, NoTls);
    let pool = deadpool_postgres::Pool::new(mgr, 100);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .route("/todos{_:/?}", web::get().to(todos))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
