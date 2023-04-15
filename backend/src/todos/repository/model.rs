use common::TodoDto;
use serde::{Deserialize, Serialize};
use ulid::{DecodeError, Ulid};

use super::super::data_source::TodoEntity;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoDetails {
    pub title: String,
    pub completed: bool,
}

#[derive(Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub uid: Ulid,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

impl Todo {
    pub fn new(title: impl Into<String>, completed: bool) -> Self {
        let title = title.into();
        Todo {
            id: None,
            uid: Ulid::new(),
            title,
            completed,
            completed_at: if completed {
                Some(chrono::Utc::now().naive_utc())
            } else {
                None
            },
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
