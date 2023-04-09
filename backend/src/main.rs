use axum::Router;
use db::establish_connection;
use std::{net::SocketAddr, sync::Arc};
use todos::web::get_todos_routes;
use tokio::sync::Mutex;

mod db;
mod schema;
mod todos;

#[tokio::main]
async fn main() {
    let conn = Arc::new(Mutex::new(
        establish_connection().expect("Failed to establish connection to database"),
    ));

    let todo_data_source = todos::data_source::DieselTodoDataSource::new(conn);
    let todo_repo = todos::repository::DataSourceTodoRepository::new(todo_data_source);
    let root_router = Router::new().nest("/todos", get_todos_routes(todo_repo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(root_router.into_make_service())
        .await
        .expect("Failed to start server");
}
