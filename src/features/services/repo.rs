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
        tracing::debug!("Service repo: Adding service with name: {}", name);
        let row = sqlx::query_as(
            "INSERT INTO service (name)
            VALUES ($1)
            RETURNING *",
        )
        .bind(name)
        .fetch_one(&*self._pool)
        .await
        .map_err(|err| {
            tracing::error!("Database error: {err}");
            err
        })?;

        tracing::debug!("Service created: {:?}", row);
        Ok(row)
    }

    pub async fn get_all_services(&self) -> Result<Vec<Service>, Box<dyn Error>> {
        tracing::debug!("Service repo: Getting vector services");
        let row = sqlx::query_as("SELECT * FROM service")
            .fetch_all(&*self._pool)
            .await
            .map_err(|err| {
                tracing::error!("Database error: {err}");
                err
            })?;

        tracing::debug!("Get services successfully");
        Ok(row)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Service, Box<dyn Error>> {
        tracing::debug!("Service repo: Getting service by id = {}", id);
        let row = sqlx::query_as::<_, Service>("SELECT * FROM service WHERE id = $1")
            .bind(id)
            .fetch_one(&*self._pool)
            .await;

        match row {
            Ok(obj) => {
                tracing::debug!("Get service successfully");
                return Ok(obj);
            }
            Err(err) => {
                tracing::error!("Database error: {err}");
                return Err(err.into());
            }
        }
    }
}
