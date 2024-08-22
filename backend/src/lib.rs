mod routes;
mod services;

use anyhow::{Context, Result};
use routes::setup_routes;
use serde::Deserialize;
use std::{net::IpAddr, time::Duration};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::sleep,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    addr: IpAddr,
    port: u16,
    #[serde(with = "humantime_serde")]
    shutdown_timeout: Option<Duration>,
}

pub async fn run_server(config: Config) -> Result<()> {
    let Config {
        addr,
        port,
        shutdown_timeout,
    } = config;

    let app = setup_routes();

    let listener = TcpListener::bind((addr, port))
        .await
        .context("bind TcpListener")?;

    // println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_timeout))
        .await
        .context("run server")
}

async fn shutdown_signal(shutdown_timeout: Option<Duration>) {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
    if let Some(shutdown_timeout) = shutdown_timeout {
        sleep(shutdown_timeout).await;
    }
}
