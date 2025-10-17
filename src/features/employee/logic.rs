use std::sync::Arc;

use bcrypt::hash;

use crate::models::{dao, dto};

pub struct Logic {
    repo: Arc<super::Repo>,
}

impl Logic {
    pub fn new(repo: Arc<super::Repo>) -> Self {
        Logic { repo: repo }
    }

    pub async fn create_employee(&self, payload: dto::Employee) -> Result<(), dto::Error> {
        tracing::debug!("Employee logic: Creating employee");

        let required_fields = [
            &payload.name,
            &payload.last_name,
            &payload.email,
            &payload.password,
            &payload.role,
        ];

        if required_fields.iter().any(|field| field.is_none()) {
            return Err(dto::Error::BadRequest("Some fields are empty".to_string()));
        }

        let hash_password = hash(payload.password.unwrap(), 14)
            .map_err(|err| dto::Error::InternalServerError(format!("bcrypt error: {}", err)))?;

        let role = dao::Role::from(payload.role)?;

        self.repo
            .create(
                payload.name.unwrap(),
                payload.last_name.unwrap(),
                payload.middle_name,
                payload.email.unwrap(),
                hash_password,
                role,
            )
            .await
            .map_err(|_| dto::Error::Conflict(String::from("Employee already is exists")))?;

        Ok(())
    }
}
