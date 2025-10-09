use std::{error::Error, sync::Arc};

use super::repo::Repo;
use crate::models::{dao, dto};

pub struct Logic {
    repo: Arc<Repo>,
}

impl Logic {
    pub fn new(repo: Arc<Repo>) -> Self {
        Logic { repo: repo }
    }

    pub async fn create(&self, payload: dto::Service) -> Result<dto::Service, Box<dyn Error>> {
        let name = payload.name;
        let result = self
            .repo
            .add_service(&name)
            .await
            .map(|model| dao::Service::to_dto(model))?;

        Ok(result)
    }
}
