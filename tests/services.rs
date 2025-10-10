use axum::http::StatusCode;
use mds_backend_rust::{features, models};
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn test_create_service(pool: PgPool) {
    let app = features::services::new(&pool);
    let server = axum_test::TestServer::new(app).unwrap();
    let payload = models::dto::Service::new(None, Some("Создание сайта".to_string()));

    let response = server.post("/create_service").json(&payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let response = server.post("/create_service").json(&payload).await;
    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}
