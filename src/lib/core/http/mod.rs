use std::{cell::RefCell, net::SocketAddr};
pub mod routes;
pub mod ws;
use axum::{http::StatusCode, routing::get_service, Router};
use crossbeam_channel::{unbounded, Receiver, RecvError, SendError, Sender};

use tower_http::services::ServeFile;

use crate::error::Error;

use self::routes::PreRoutes;

use super::config::Config;
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
    let mut config = Config::read_config()?;
    let routes = RefCell::new(Router::new());
    PreRoutes::fix(&mut config)?;
    let route_info = config.fix()?.routes;

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
