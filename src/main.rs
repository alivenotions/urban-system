use sqlx::PgPool;
use std::net::TcpListener;
use urban_system::configuration::get_configuration;
use urban_system::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to DB");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to the port");
    run(listener, connection)?.await
}
