use rbatis::RBatis;
use salvo::prelude::*;
use tracing_subscriber;
use yaml::CONFIG;

mod controller;
mod mapper;
mod model;
mod router;
mod service;
mod utils;
mod yaml;

lazy_static::lazy_static! {
    static ref RB: RBatis = RBatis::new();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    utils::mysql::init_db().await;

    let router = Router::new().push(router::router());

    let local_addr = format!("{}:{}", CONFIG.server.host, CONFIG.server.port);
    let acceptor = TcpListener::new(local_addr).bind().await;

    Server::new(acceptor).serve(router).await;
}
