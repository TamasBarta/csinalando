use ::serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use axum::async_trait;
use tokio::sync::Mutex;
use ulid::Ulid;

pub struct InMemoryTodoRepository {
    todos: Mutex<Vec<Todo>>,
}

#[async_trait]
pub trait TodoRepository {
    async fn get_all(&self) -> Vec<Todo>;
    async fn get_by_id(&self, id: Ulid) -> Result<Todo>;
    async fn delete_by_id(&self, id: Ulid) -> Result<()>;
    async fn update(&self, id: Ulid, details: TodoDetails) -> Result<()>;
    async fn add(&self, todo: Todo) -> Result<()>;
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        InMemoryTodoRepository {
            todos: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn get_all(&self) -> Vec<Todo> {
        self.todos.lock().await.clone()
    }

    async fn get_by_id(&self, id: Ulid) -> Result<Todo> {
        Ok(self
            .todos
            .lock()
            .await
            .iter()
            .find(|todo| todo.id == id)
            .ok_or_else(|| anyhow!("Cannot find todo with id."))?
            .clone())
    }

    async fn delete_by_id(&self, id: Ulid) -> Result<()> {
        let mut todos = self.todos.lock().await;
        todos.retain(|todo| todo.id != id);
        Ok(())
    }

    async fn update(&self, id: Ulid, details: TodoDetails) -> Result<()> {
        let mut todos = self.todos.lock().await;
        todos
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or_else(|| anyhow!("Cannot find todo with id."))?
            .title = details.title;
        Ok(())
    }

    async fn add(&self, todo: Todo) -> Result<()> {
        self.todos.lock().await.push(todo);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoDetails {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    id: Ulid,
    title: String,
}

impl Todo {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Todo {
            id: Ulid::new(),
            title,
        }
    }
}
