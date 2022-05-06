use std::net::SocketAddr;
pub mod ws;
use axum::{http::StatusCode, routing::get_service, Router, Server};
use tower_http::services::ServeDir;
use tracing::info;
use crossbeam_channel::{Sender, Receiver, RecvError, unbounded, SendError};

#[derive(Debug, Clone)]
pub struct MsgHandler<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>
}

impl<T> MsgHandler<T> {
    pub fn new() -> Self {
        let (s, r) = unbounded::<T>();
        Self {
            sender: s,
            receiver: r
        }
    }
    pub fn send(&self,v: T) -> Result<(), SendError<T>> {
        self.sender.send(v)?;
        Ok(())
    }
    pub fn receive(&self) -> Result<T, RecvError> {
        let val = self.receiver.recv()?;
        Ok(val)
    }
}

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