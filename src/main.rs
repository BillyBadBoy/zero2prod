use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::config::{Settings, get_configuration};
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // We are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );

    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

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
