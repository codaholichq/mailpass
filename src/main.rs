use mailpass::run_server;

#[tokio::main]
async fn main() {
    run_server().await
}
