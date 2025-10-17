use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::models::dto::{Employee, ErrorResponse};

pub struct Handler {
    logic: Arc<super::Logic>,
}

impl Handler {
    pub fn new(logic: Arc<super::Logic>) -> Self {
        Handler { logic: logic }
    }

    pub async fn create(
        State(handler): State<Arc<Handler>>,
        Json(payload): Json<Employee>,
    ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
        tracing::info_span!("Employee handler: create", payload = ?payload)
            .in_scope(|| async {
                match handler.logic.create_employee(payload).await {
                    Ok(_) => {
                        tracing::debug!("Employee created successfully");
                        Ok(StatusCode::CREATED)
                    }
                    Err(err) => {
                        tracing::error!("Failed to create service: {:?}", err);
                        Err(err.into_response())
                    }
                }
            })
            .await
    }
}
