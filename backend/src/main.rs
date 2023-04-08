use axum::Router;
use std::net::SocketAddr;
use todos::web::get_todos_routes;

mod todos;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let root_router = Router::new().nest("/todos", get_todos_routes());

    axum::Server::bind(&addr)
        .serve(root_router.into_make_service())
        .await
        .unwrap();
}
