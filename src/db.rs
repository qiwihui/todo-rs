use crate::errors::Error;
use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, Error> {
    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await
        .unwrap();
    let todos = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_todo(client: &Client, list_id: i32) -> Result<TodoList, Error> {
    let statement = client
        .prepare("select * from todo_list where id = $1")
        .await
        .unwrap();

    let may_todo = client
        .query_opt(&statement, &[&list_id])
        .await
        .expect("Error getting todo list")
        .map(|row| TodoList::from_row_ref(&row).unwrap());

    match may_todo {
        Some(todo) => Ok(todo),
        None => Err(Error::NotFound("Not found".into())),
    }
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, Error> {
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .unwrap();

    client
        .query(&statement, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(Error::InternalServerError(
            "Error creating todo list".into(),
        ))
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id desc")
        .await
        .unwrap();
    let todo_items = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todo_items)
}

pub async fn get_item(client: &Client, list_id: i32, item_id: i32) -> Result<TodoItem, Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 and id = $2")
        .await
        .unwrap();

    let may_item = client
        .query_opt(&statement, &[&list_id, &item_id])
        .await
        .expect("Error getting todo list")
        .map(|row| TodoItem::from_row_ref(&row).unwrap());

    match may_item {
        Some(item) => Ok(item),
        None => Err(Error::NotFound("Not found".into())),
    }
}

pub async fn create_item(client: &Client, list_id: i32, title: String) -> Result<TodoItem, Error> {
    let statement = client
        .prepare("insert into todo_item (list_id, title) values ($1, $2) returning id, list_id, title, checked")
        .await
        .unwrap();

    client
        .query(&statement, &[&list_id, &title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(Error::InternalServerError(
            "Error creating todo list".into(),
        ))
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<bool, Error> {
    let statement = client
        .prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false")
        .await
        .unwrap();
    let result = client
        .execute(&statement, &[&list_id, &item_id])
        .await
        .expect("Error creating todo list");

    match result {
        updated if updated == 1 => Ok(true),
        _ => Ok(false),
    }
}
