use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use tokio::sync::Mutex;

use diesel::prelude::*;
use ulid::Ulid;

use crate::schema::todos;

#[async_trait]
pub trait TodoDataSource {
    async fn add(&self, todo: &TodoEntityInsert) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<TodoEntity>>;
    async fn get_by_id(&self, id: Ulid) -> Option<TodoEntity>;
    async fn delete_by_id(&self, id: Ulid) -> Result<()>;
    async fn update(&self, id: Ulid, details: TodoEntityUpdate) -> Result<()>;
}

pub struct DieselTodoDataSource {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl DieselTodoDataSource {
    pub fn new(conn: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { conn }
    }
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = todos)]
pub struct TodoEntity {
    pub id: i32,
    pub uid: String,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = todos)]
pub struct TodoEntityInsert<'a> {
    pub uid: &'a str,
    pub title: &'a str,
    pub completed: bool,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = todos)]
pub struct TodoEntityUpdate {
    pub title: String,
    pub completed: bool,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

#[async_trait]
impl TodoDataSource for DieselTodoDataSource {
    async fn add(&self, todo: &TodoEntityInsert) -> Result<()> {
        use crate::schema::todos::dsl::*;
        diesel::insert_into(todos)
            .values(todo)
            .execute(&mut *self.conn.lock().await)?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<TodoEntity>> {
        use crate::schema::todos::dsl::*;
        let conn = &mut *self.conn.lock().await;
        Ok(todos.load::<TodoEntity>(conn)?)
    }

    async fn get_by_id(&self, ulid: Ulid) -> Option<TodoEntity> {
        use crate::schema::todos::dsl::*;
        let conn = &mut *self.conn.lock().await;
        todos
            .filter(uid.eq(ulid.to_string()))
            .first::<TodoEntity>(conn)
            .ok()
    }

    async fn delete_by_id(&self, ulid: Ulid) -> Result<()> {
        use crate::schema::todos::dsl::*;
        diesel::delete(todos.filter(uid.eq(ulid.to_string())))
            .execute(&mut *self.conn.lock().await)?;
        Ok(())
    }

    async fn update(&self, ulid: Ulid, details: TodoEntityUpdate) -> Result<()> {
        use crate::schema::todos::dsl::*;
        diesel::update(todos.filter(uid.eq(ulid.to_string())))
            .set((
                title.eq(details.title),
                completed.eq(details.completed),
                completed_at.eq(details.completed_at),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut *self.conn.lock().await)?;
        Ok(())
    }
}
