use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get_service, Router, Server};
use tower_http::services::ServeDir;
use tracing::info;

pub async fn server() {
    let app = Router::new().nest(
        "/",
        get_service(ServeDir::new("dist")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
