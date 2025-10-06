use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    id: Option<i64>,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}