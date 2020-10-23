use crate::db;
use crate::models::AppState;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetTodoList {
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct GetTodoItem {
    pub list_id: i32,
    pub item_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
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

pub async fn items(info: web::Path<GetTodoList>, state: web::Data<AppState>) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_items(&client, info.list_id).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_item(
    todo: web::Path<GetTodoList>,
    info: web::Json<CreateTodoItem>,
    state: web::Data<AppState>,
) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::create_item(&client, todo.list_id, info.0.title.clone()).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_item(info: web::Path<GetTodoItem>, state: web::Data<AppState>) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_item(&client, info.list_id, info.item_id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: bool,
}

pub async fn check_todo(
    info: web::Path<GetTodoItem>,
    state: web::Data<AppState>,
) -> impl Responder {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::check_todo(&client, info.list_id, info.item_id).await;
    match result {
        Ok(yes_or_no) => HttpResponse::Ok().json(ResultResponse { result: yes_or_no }),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
