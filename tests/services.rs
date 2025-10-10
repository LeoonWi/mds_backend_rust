use axum::http::StatusCode;
use mds_backend_rust::{features, logger, models};
use serde_json::Value;
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn test_create_service(pool: PgPool) {
    eprintln!("Testing create service");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Payload
    let name = String::from("Создание сайта");
    let payload = models::dto::Service::new(None, Some(name.clone()));

    // Request
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::Service = response.json();
    eprintln!(
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
    eprintln!("Testing double create service");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Payload
    let name = String::from("Создание сайта");
    let payload = models::dto::Service::new(None, Some(name.clone()));

    // Request №1
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::Service = response.json();
    eprintln!(
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
    eprintln!(
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
    eprintln!("Testing create service with empty name");
    logger::init_dev_logger();

    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    // Request with 'name' is ""
    let payload = models::dto::Service::new(None, None);
    let response = server.post("/services").json(&payload).await;
    let json_body: models::dto::ErrorResponse = response.json();
    eprintln!(
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
    eprintln!("Result request:\n{}", response.status_code());

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}
