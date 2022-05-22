use std::{cell::RefCell, net::SocketAddr, sync::Arc, thread};
pub mod routes;
pub mod ws;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use crossbeam_channel::{unbounded, Receiver, RecvError, SendError, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use tower_http::services::{ServeDir, ServeFile};

use crate::error::Error;

use self::routes::RouteInfo;

use super::config::{Config, Route};
#[derive(Debug, Clone)]
pub struct MsgHandler<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> MsgHandler<T> {
    pub fn new() -> Self {
        let (s, r) = unbounded::<T>();
        Self {
            sender: s,
            receiver: r,
        }
    }
    pub fn send(&self, v: T) -> Result<(), SendError<T>> {
        self.sender.send(v)?;
        Ok(())
    }
    pub fn receive(&self) -> Result<T, RecvError> {
        let val = self.receiver.recv()?;
        Ok(val)
    }
}

pub fn get_routes() -> Result<Router, Error> {
    let route_info = Config::read_config()?.fix()?.routes;
    let routes = RefCell::new(Router::new());

    for route in route_info {
        let r = ServeFile::new(route.file_name);
        let route = Router::new().route(
            &format!("/{}", route.to),
            get_service(r).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        );
        routes.replace(routes.clone().into_inner().merge(route));
    }
    Ok(routes.into_inner())
}

pub async fn server() -> Result<(), Error> {
    let routes = get_routes()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    Ok(())
}
