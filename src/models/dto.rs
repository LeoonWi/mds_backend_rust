use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Service {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Service {
            id: id,
            name: name.unwrap_or_else(|| "".to_string()),
            created_at: None,
            updated_at: None,
        }
    }
}
