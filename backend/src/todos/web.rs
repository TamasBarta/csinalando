use crate::todos::repository::{
    dto::{Todo, TodoDetails},
    InMemoryTodoRepository, TodoRepository,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use ulid::Ulid;

pub fn get_todos_routes() -> Router {
    Router::new()
        .route("/", get(find_all))
        .route("/", post(new))
        .route("/:id", get(find_by_id))
        .route("/:id", delete(delete_by_id))
        .route("/:id", put(update_by_id))
        .with_state(Arc::new(InMemoryTodoRepository::new()))
}

#[derive(Deserialize)]
pub struct NewTodoDto {
    pub title: String,
}

async fn find_all(
    State(todo_repo): State<Arc<impl TodoRepository>>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    Ok(todo_repo.get_all().await.into())
}

async fn find_by_id(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, StatusCode> {
    Ok(todo_repo
        .get_by_id(Ulid::from_string(id.as_str()).map_err(|_| StatusCode::NOT_FOUND)?)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?
        .into())
}

async fn delete_by_id(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Path(id): Path<String>,
) -> Result<(), StatusCode> {
    todo_repo
        .delete_by_id(Ulid::from_string(id.as_str()).map_err(|_| StatusCode::NOT_FOUND)?)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(())
}

async fn update_by_id(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Path(id): Path<Ulid>,
    Json(details): Json<NewTodoDto>,
) -> Result<(), StatusCode> {
    todo_repo
        .update(
            id,
            TodoDetails {
                title: details.title.clone(),
            },
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(())
}

async fn new(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Json(details): Json<NewTodoDto>,
) -> Result<Json<Todo>, StatusCode> {
    let new_todo = Todo::new(details.title);
    todo_repo
        .add(&new_todo)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(new_todo.into())
}
