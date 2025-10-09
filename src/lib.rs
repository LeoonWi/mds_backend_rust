mod config;
mod db;
mod features;
mod models;

use axum::Router;

use crate::config::Config;
use std::error::Error;

pub async fn server_run() -> Result<(), Box<dyn Error>> {
    let config = Config::build()?;
    let pool = db::connect(&config.db_url)?;
    let service_router = features::services::new(&pool);

    let app = Router::new().merge(service_router);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &config.ip, &config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
