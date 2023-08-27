use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::{startup::run, configuration::get_configuration};
use env_logger::Env;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener, connection_pool)?.await
}