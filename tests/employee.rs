use axum::http::StatusCode;
use mds_backend_rust::{features, logger};
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn test_employee_create(pool: PgPool) {
    println!("Testing create employee");
    logger::init_dev_logger();

    let app = features::employee::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();

    let payload = json!({
        "name": "Василий",
        "last_name": "Ломоносов",
        "email": "shagin.v.i.21@gmail.com",
        "password": "qwerty",
        "role": "Сотрудник",
    });

    // Request 1 - OK
    let response = server.post("/employee").json(&payload).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    // Request 2 - Error
    let response = server.post("/employee").json(&payload).await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}
