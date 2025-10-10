mod config;
mod db;
mod features;
mod logger;
mod models;

use axum::Router;

use crate::config::Config;
use std::error::Error;

pub async fn server_run() -> Result<(), Box<dyn Error>> {
    let config = Config::build()?;
    match config.profile.as_str() {
        "prod" => logger::init_prod_logger(),
        _ => logger::init_dev_logger(),
    }

    tracing::info!("Starting application");

    let pool = db::connect(&config.db_url)?;
    let service = features::services::new(&pool);
    let app = Router::new().merge(service);

    tracing::info!("Server running on {}:{}", config.ip, config.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.ip, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
