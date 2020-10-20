use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}
