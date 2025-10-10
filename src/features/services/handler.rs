use std::sync::Arc;

use axum::extract::State;
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
}
