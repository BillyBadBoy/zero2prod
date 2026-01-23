use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::config::{Settings, get_configuration};
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");

    run(build_listener(&config), build_db_connection(&config).await)?.await
}

fn build_listener(config: &Settings) -> TcpListener {
    let address = format!("127.0.0.1:{}", config.application_port);
    TcpListener::bind(address).expect("Failed to bind port")
}

async fn build_db_connection(config: &Settings) -> PgPool {
    let connection_string = config.database.connection_string();
    PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.")
}
