use crate::db::{get_todo, get_todos};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub list_id: i32,
}

pub async fn todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn todo(info: web::Path<Info>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = get_todo(&client, info.list_id).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
