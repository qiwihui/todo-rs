use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io::{Error, ErrorKind};
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
        .expect("Error getting todo lists")
        .map(|row| TodoList::from_row_ref(&row).unwrap());

    match may_todo {
        Some(todo) => Ok(todo),
        None => Err(Error::new(ErrorKind::NotFound, "Not found")),
    }
}
