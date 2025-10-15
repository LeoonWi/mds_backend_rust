use std::sync::Arc;

use super::repo::Repo;
use crate::models::dao;
use crate::models::dto::{Error, Service};

pub struct Logic {
    repo: Arc<Repo>,
}

impl Logic {
    pub fn new(repo: Arc<Repo>) -> Self {
        Logic { repo: repo }
    }

    pub async fn create(&self, payload: Service) -> Result<Service, Error> {
        tracing::debug!("Service logic: Creating service");
        if payload.name.is_empty() {
            tracing::error!("Field name is empty");
            return Err(Error::BadRequest(
                "Field 'name' can't be empty.".to_string(),
            ));
        }
        self.repo
            .add_service(&payload.name)
            .await
            .map(|model| dao::Service::to_dto(model))
            .map_err(|_| Error::Conflict("Object already exists.".to_string()))
    }

    pub async fn get_all(&self) -> Vec<Service> {
        tracing::debug!("Service logic: Getting all services");
        match self.repo.get_all_services().await {
            Ok(v) => v
                .into_iter()
                .map(|elem| dao::Service::to_dto(elem))
                .collect(),
            Err(_) => Vec::<Service>::new(),
        }
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Service, Error> {
        tracing::debug!("Service logic: Getting service by id");
        let result = self
            .repo
            .get_by_id(id)
            .await
            .map(|model| dao::Service::to_dto(model))
            .map_err(|_| Error::NotFound(format!("Service with id: {} not found", id)));
        return result;
    }

    pub async fn put_by_id(&self, id: i64, payload: Service) -> Result<Service, Error> {
        tracing::debug!("Service logic: Updating service by id");
        if payload.name.is_empty() {
            return Err(Error::BadRequest("Field name can't be empty".to_string()));
        }

        let result = self
            .repo
            .update_by_id(id, payload.name)
            .await
            .map(|model| dao::Service::to_dto(model))
            .map_err(|_| Error::NotFound(format!("Service with id: {} not found", id)));

        return result;
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<i64, Error> {
        tracing::debug!("Service logic: Deleting service by id");
        let result = self.repo.delete_by_id(id).await;

        match result {
            Ok(rows) => {
                if rows > 0 {
                    Ok(id)
                } else {
                    tracing::error!("Database error: Service not found by id {}", id);
                    Err(Error::NotFound(format!(
                        "Service with id: {} not found",
                        id
                    )))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {err}");
                Err(Error::InternalServerError(format!(
                    "Internal database error"
                )))
            }
        }
    }
}
