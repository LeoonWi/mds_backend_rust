use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use super::logic::Logic;
use crate::models::dto::Service;

pub struct Handler {
    logic: Arc<Logic>,
}

impl Handler {
    pub fn new(logic: Arc<Logic>) -> Self {
        Handler { logic: logic }
    }

    pub async fn create_service(
        State(handler): State<Arc<Handler>>,
        Json(payload): Json<Service>,
    ) -> (StatusCode, Json<Value>) {
        tracing::info_span!("Service handler: create_service", payload = ?payload)
            .in_scope(|| async {
                match handler.logic.create(payload).await {
                    Ok(result) => {
                        tracing::debug!("Service created successfully: {:?}", result);
                        (StatusCode::CREATED, Json(json!(result)))
                    }
                    Err(err) => {
                        tracing::error!("Failed to create service: {:?}", err);
                        let (status, Json(error_response)) = err.into_response();
                        (status, Json(json!(error_response)))
                    }
                }
            })
            .await
    }

    pub async fn get_services(State(handler): State<Arc<Handler>>) -> Json<Vec<Service>> {
        tracing::info_span!("Service handler: get_services")
            .in_scope(|| async {
                let arr = handler.logic.get_all().await;
                Json(arr)
            })
            .await
    }

    pub async fn get_service_by_id(
        State(handler): State<Arc<Handler>>,
        Path(id): Path<i64>,
    ) -> (StatusCode, Json<Value>) {
        tracing::info_span!("Service handler: get_services_by_id with ", id)
            .in_scope(|| async {
                match handler.logic.get_by_id(id).await {
                    Ok(result) => {
                        tracing::debug!("Get service by id successfully");
                        (StatusCode::OK, Json(json!(result)))
                    }
                    Err(err) => {
                        tracing::error!("Failed to get service by id");
                        let (status, Json(error_response)) = err.into_response();
                        (status, Json(json!(error_response)))
                    }
                }
            })
            .await
    }
}
