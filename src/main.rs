mod models;

use dotenv::dotenv;
use std::env;
use models::*;
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Json, Router};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let url = match env::var("DATABASE_URL") {
        Ok(value) => value,
        Err(e) => panic!("{e}"),
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url).await?;

    let app = Router::new()
        .route("/create_service/{name}", get(create_service))
        .route("/get_service", get(get_service))
        .with_state(pool);
    let ip = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("Server listener {ip}");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn create_service(State(pool): State<PgPool>, Path(name): Path<String>) -> Result<Json<service::Service>, (StatusCode, String)> {
    let row  = sqlx::query_as(
            "INSERT INTO service (name)
            VALUES ($1)
            RETURNING *"
        )
        .bind(&name)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}"))
        })?;
    
    Ok(Json(row))
}

async fn get_service(State(pool): State<PgPool>) -> Result<Json<Vec<service::Service>>, (StatusCode, String)> {
    let row = sqlx::query_as(
        "SELECT * FROM service"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (StatusCode::NOT_FOUND, format!("{e}"))
    })?;

    Ok(Json(row))
}