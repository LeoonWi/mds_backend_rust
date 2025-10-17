use std::sync::Arc;

use sqlx::PgPool;

use crate::models::dao;

pub struct Repo {
    pool: Arc<PgPool>,
}

impl Repo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Repo { pool: pool }
    }

    pub async fn create<'a>(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        password: String,
        role: dao::Role,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!("Employee repo: Adding employee");
        sqlx::query(
            "INSERT INTO employee (name, last_name, middle_name, email, password, role, active)
			VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(name)
        .bind(last_name)
        .bind(middle_name)
        .bind(email)
        .bind(password)
        .bind(role)
        .bind(true)
        .execute(&*self.pool)
        .await
        .map_err(|err| {
            tracing::error!("Database error: {err}");
            err
        })?;

        Ok(())
    }
}
