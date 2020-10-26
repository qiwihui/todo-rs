use crate::db::create_todo;
use crate::handlers::CreateTodoList;
use crate::models;
use crate::tests::helpers::{assert_get, assert_post, APP_STATE};
use actix_web::test;
use deadpool_postgres::Client;

#[actix_rt::test]
async fn test_hello_world() {
    assert_get("/").await;
}

#[actix_rt::test]
async fn test_get_todos() {
    // create data in db
    let todo_title = "New Todo List";
    let client: Client = APP_STATE
        .pool
        .get()
        .await
        .expect("Error connecting to the database");
    let new_todo = create_todo(&client, todo_title.into()).await;
    assert!(new_todo.is_ok(), "Failed to create new test todo");
    // get and check
    let new_todo = new_todo.unwrap();
    let response = assert_get("/todos").await;
    let todos: Vec<models::TodoList> = test::read_body_json(response).await;
    let maybe_list = todos.iter().find(|todo| todo.id == new_todo.id);
    assert!(maybe_list.is_some(), "Item not found!");
}

#[actix_rt::test]
async fn test_create_todos() {
    let todo_title = "Create todo List";

    let params = CreateTodoList {
        title: todo_title.into(),
    };
    let response = assert_post("/todos", params).await;
    // check body
    let body = test::read_body(response).await;
    let try_created: Result<models::TodoList, serde_json::error::Error> =
        serde_json::from_slice(&body);
    assert!(try_created.is_ok(), "Response couldn't not be parsed");
    // check if created
    let created_list = try_created.unwrap();
    let resp = assert_get("/todos").await;
    let todos: Vec<models::TodoList> = test::read_body_json(resp).await;
    let maybe_list = todos.iter().find(|todo| todo.id == created_list.id);
    assert!(maybe_list.is_some(), "Item not found!");
}
