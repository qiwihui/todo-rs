use crate::db;
use crate::models::AppState;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTodoList {
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

pub async fn todos(state: web::Data<AppState>) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn todo(info: web::Path<GetTodoList>, state: web::Data<AppState>) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_todo(&client, info.list_id).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(
    info: web::Json<CreateTodoList>,
    state: web::Data<AppState>,
) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo(&client, info.0.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

