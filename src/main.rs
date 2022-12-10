use std::net::TcpListener;
use urban_system::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9091").expect("Failed to bind to the port");
    run(listener)?.await
}
