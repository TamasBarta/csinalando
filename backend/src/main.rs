use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use todos::repository::{InMemoryTodoRepository, Todo, TodoDetails, TodoRepository};
use ulid::Ulid;

mod todos;

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemoryTodoRepository::new());

    let app = Router::new()
        .route("/todos", get(find_all))
        .route("/todos/:id", get(find_by_id))
        .route("/todos/:id", delete(delete_by_id))
        .route("/todos/:id", put(update_by_id))
        .route("/todos/new", post(new))
        .with_state(repo);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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

#[derive(Deserialize)]
struct NewTodoDto {
    title: String,
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
) -> Result<(), StatusCode> {
    todo_repo
        .add(Todo::new(details.title))
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(())
}
