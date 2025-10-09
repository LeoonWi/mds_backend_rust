use chrono::{DateTime, Utc};

use crate::models::dto;

#[derive(Debug, sqlx::FromRow)]
pub struct Service {
    id: Option<i64>,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn to_dto(from: Service) -> dto::Service {
        dto::Service {
            id: from.id,
            name: from.name,
            created_at: Some(from.created_at),
            updated_at: from.updated_at,
        }
    }
}
