use std::sync::Arc;

use axum::extract::State;
use axum::{Json, http::StatusCode};

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
    ) -> Result<Json<Service>, StatusCode> {
        match handler.logic.create(payload).await {
            Ok(result) => Ok(Json(result)),
            Err(_) => Err(StatusCode::CONFLICT),
        }
    }
}
