use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io::Error;
use tokio_postgres::Row;

fn row_to_todo(row: &Row) -> TodoList {
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    TodoList { id, title }
}

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
        .map(|row| row_to_todo(row))
        .collect::<Vec<TodoList>>();

    Ok(todos)
}
