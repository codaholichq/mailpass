use tokio::net::TcpListener;

mod routes;
mod services;

use routes::setup_routes;

pub async fn run_server() {
    let app = setup_routes();

    let listener: TcpListener = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
