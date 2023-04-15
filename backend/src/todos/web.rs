use crate::todos::repository::{
    model::{Todo, TodoDetails},
    TodoRepository,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{NewTodoDto, TodoDto};
use std::sync::Arc;
use ulid::Ulid;

pub fn get_todos_routes<R: TodoRepository + Sync + Send + 'static>(repository: R) -> Router {
    Router::new()
        .route("/", get(find_all))
        .route("/", post(new))
        .route("/:id", get(find_by_id))
        .route("/:id", delete(delete_by_id))
        .route("/:id", put(update_by_id))
        .with_state(Arc::new(repository))
}

async fn find_all(
    State(todo_repo): State<Arc<impl TodoRepository>>,
) -> Result<Json<Vec<TodoDto>>, StatusCode> {
    let todos = todo_repo
        .get_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(todos
        .iter()
        .map(|todo| todo.into())
        .collect::<Vec<TodoDto>>()
        .into())
}

async fn find_by_id(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Path(id): Path<String>,
) -> Result<Json<TodoDto>, StatusCode> {
    let todo = todo_repo
        .get_by_id(Ulid::from_string(id.as_str()).map_err(|_| StatusCode::NOT_FOUND)?)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Into::<TodoDto>::into(&todo).into())
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
                completed: details.completed,
            },
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(())
}

async fn new(
    State(todo_repo): State<Arc<impl TodoRepository>>,
    Json(details): Json<NewTodoDto>,
) -> Result<Json<TodoDto>, StatusCode> {
    let new_todo = Todo::new(details.title, details.completed);
    todo_repo
        .add(&new_todo)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(TodoDto::from(&new_todo).into())
}

impl From<&Todo> for TodoDto {
    fn from(todo: &Todo) -> Self {
        TodoDto {
            uid: todo.uid,
            title: todo.title.clone(),
            completed: todo.completed,
            created_at: todo.created_at,
            completed_at: todo.completed_at,
            updated_at: todo.updated_at,
        }
    }
}
