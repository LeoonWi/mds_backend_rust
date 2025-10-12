mod common;

use axum::http::{StatusCode, response};
use mds_backend_rust::{
    features, logger,
    models::{self, dto},
};
use serde_json::Value;
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn test_create_service(pool: PgPool) {
    println!("Testing create service");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Payload
    let name = String::from("Создание сайта");
    let payload = models::dto::Service::new(None, Some(name.clone()));

    // Request
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::Service = response.json();
    println!(
        "Result request:\n{}",
        serde_json::to_string_pretty(&json_body).expect("Failed to format JSON")
    );

    assert_eq!(
        (response.status_code(), json_body.name),
        (StatusCode::CREATED, name)
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn test_double_create_service(pool: PgPool) {
    println!("Testing double create service");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Payload
    let name = String::from("Создание сайта");
    let payload = models::dto::Service::new(None, Some(name.clone()));

    // Request №1
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::Service = response.json();
    println!(
        "Result first request:\n{}\n",
        serde_json::to_string_pretty(&json_body).expect("Failed to format JSON")
    );

    assert_eq!(
        (response.status_code(), json_body.name),
        (StatusCode::CREATED, name)
    );

    // Request №2
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::ErrorResponse = response.json();
    println!(
        "Result second request:\n{}",
        serde_json::to_string_pretty(&json_body).expect("Failed to format JSON")
    );

    assert_eq!(
        (response.status_code(), json_body.error),
        (StatusCode::CONFLICT, "Object already exists.".to_string())
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn test_create_service_with_empty_name(pool: PgPool) {
    println!("Testing create service with empty name");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Request with 'name' is ""
    let payload = models::dto::Service::new(None, None);
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::ErrorResponse = response.json();
    println!(
        "Result request:\n{}",
        serde_json::to_string_pretty(&json_body).expect("Failed to format JSON")
    );

    assert_eq!(
        (response.status_code(), json_body.error),
        (
            StatusCode::BAD_REQUEST,
            "Field 'name' can't be empty.".to_string()
        )
    );

    // Request without 'name'
    let payload = serde_json::json!({"id": 1, "eman": "Создание сайта"});
    let response = server.post("/services").json(&payload).await;
    println!("Result request:\n{}", response.status_code());

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn test_get_services(pool: PgPool) {
    println!("Testing get all services");

    logger::init_dev_logger();

    let services = common::setup_services(&pool, 5)
        .await
        .expect("Failed to created services");

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    let response = server.get("/services").await;
    let result_json: Vec<dto::Service> = response.json();
    println!(
        "Result request:\n{}",
        serde_json::to_string_pretty(&result_json).expect("Failed to format JSON")
    );

    assert_eq!(services, result_json);
}

#[sqlx::test]
async fn test_get_service_by_id(pool: PgPool) {
    println!("Testing get service by id");

    logger::init_dev_logger();

    let services = common::setup_services(&pool, 3)
        .await
        .expect("Failted to created services");

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Request №1 - Testing get service with exists id
    let id = 2 as i64;
    let response = server.get(format!("/services/{}", id).as_str()).await;
    let response_json = response.json::<dto::Service>();
    println!(
        "Result request:\n{}",
        serde_json::to_string_pretty(&response_json).expect("Failed to format JSON")
    );

    assert_eq!(
        &response_json,
        services.iter().find(|&x| x.id == Some(id)).unwrap()
    );

    // Request №2 - Testing get service with doesn't exists id
    let id = 4 as i64;
    let response = server.get(format!("/services/{}", id).as_str()).await;
    let status_code = response.status_code();
    let response_json = response.json::<dto::ErrorResponse>();
    println!(
        "Result request:\n{}",
        serde_json::to_string_pretty(&response_json).expect("Failed to format JSON")
    );

    assert_eq!(
        (status_code, response_json.error),
        (
            StatusCode::NOT_FOUND,
            format!("Service with id: {} not found", id)
        )
    );
}
