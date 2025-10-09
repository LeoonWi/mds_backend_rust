use std::{error::Error, sync::Arc};

use crate::models::dao::Service;
use sqlx::PgPool;

pub struct Repo {
    _pool: Arc<PgPool>,
}

impl Repo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Repo { _pool: pool }
    }

    pub async fn add_service<'a>(&self, name: &'a str) -> Result<Service, Box<dyn Error>> {
        let row = sqlx::query_as(
            "INSERT INTO service (name)
            VALUES ($1)
            RETURNING *",
        )
        .bind(name)
        .fetch_one(&*self._pool)
        .await?;

        Ok(row)
    }
}
