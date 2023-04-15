use anyhow::{anyhow, Result};
use axum::async_trait;
use ulid::Ulid;

use self::model::{Todo, TodoDetails};

use super::data_source::{TodoDataSource, TodoEntityInsert, TodoEntityUpdate};

pub struct DataSourceTodoRepository<T: TodoDataSource> {
    data_source: T,
}

#[async_trait]
pub trait TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>>;
    async fn get_by_id(&self, id: Ulid) -> Result<Todo>;
    async fn delete_by_id(&self, id: Ulid) -> Result<()>;
    async fn update(&self, id: Ulid, details: TodoDetails) -> Result<()>;
    async fn add(&self, todo: &Todo) -> Result<()>;
}

impl<T: TodoDataSource> DataSourceTodoRepository<T> {
    pub fn new(data_source: T) -> Self {
        DataSourceTodoRepository::<T> { data_source }
    }
}

#[async_trait]
impl<T: TodoDataSource + Send + Sync> TodoRepository for DataSourceTodoRepository<T> {
    async fn get_all(&self) -> Result<Vec<Todo>> {
        Ok(self
            .data_source
            .get_all()
            .await?
            .iter()
            .map(|todo| {
                Todo::try_from(todo)
                    .map_err(|_| anyhow!("Validation of todos from database failed."))
            })
            .collect::<Result<Vec<Todo>>>()?)
    }

    async fn get_by_id(&self, id: Ulid) -> Result<Todo> {
        self.data_source
            .get_by_id(id)
            .await
            .ok_or_else(|| anyhow!("Cannot find todo with id."))
            .and_then(|todo| {
                Todo::try_from(&todo)
                    .map_err(|_| anyhow!("Validation of todo from database failed."))
            })
    }

    async fn delete_by_id(&self, id: Ulid) -> Result<()> {
        self.data_source.delete_by_id(id).await?;
        Ok(())
    }

    async fn update(&self, id: Ulid, details: TodoDetails) -> Result<()> {
        self.data_source
            .update(
                id,
                TodoEntityUpdate {
                    title: details.title,
                    completed: details.completed,
                    completed_at: if details.completed {
                        Some(chrono::Utc::now().naive_utc())
                    } else {
                        None
                    },
                },
            )
            .await?;
        Ok(())
    }

    async fn add(&self, todo: &Todo) -> Result<()> {
        self.data_source
            .add(&TodoEntityInsert {
                uid: todo.uid.to_string().as_str(),
                title: todo.title.as_str(),
                completed: todo.completed,
                completed_at: todo.completed_at,
            })
            .await?;
        Ok(())
    }
}

pub mod model;
