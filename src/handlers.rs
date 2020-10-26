use crate::db;
use crate::errors::Error;
use crate::models::AppState;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[derive(Deserialize)]
pub struct GetTodoList {
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct GetTodoItem {
    pub list_id: i32,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
}

pub async fn todos(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn todo(
    info: web::Path<GetTodoList>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_todo(&client, info.list_id).await;
    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn create_todo(
    info: web::Json<CreateTodoList>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo(&client, info.0.title.clone()).await;
    match result {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn items(
    info: web::Path<GetTodoList>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_items(&client, info.list_id).await;
    match result {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn create_item(
    todo: web::Path<GetTodoList>,
    info: web::Json<CreateTodoItem>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::create_item(&client, todo.list_id, info.0.title.clone()).await;
    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn get_item(
    info: web::Path<GetTodoItem>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_item(&client, info.list_id, info.item_id).await;
    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(e) => Ok(e.error_response()),
    }
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: bool,
}

pub async fn check_todo(
    info: web::Path<GetTodoItem>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client: Client = state
        .pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::check_todo(&client, info.list_id, info.item_id).await;
    match result {
        Ok(yes_or_no) => Ok(HttpResponse::Ok().json(ResultResponse { result: yes_or_no })),
        Err(e) => Ok(e.error_response()),
    }
}
