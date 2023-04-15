use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoDto {
    pub uid: Ulid,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewTodoDto {
    pub title: String,
    pub completed: bool,
}
