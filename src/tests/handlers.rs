use crate::handlers::CreateTodoList;
use crate::models;
use crate::tests::helpers::{assert_get, assert_post};
use actix_web::test;

#[actix_rt::test]
async fn test_hello_world() {
    assert_get("/").await;
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
