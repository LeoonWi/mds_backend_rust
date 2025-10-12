use std::sync::Arc;

use mds_backend_rust::{features, models::dto};
use sqlx::PgPool;

pub async fn setup_services(pool: &PgPool, count: usize) -> Result<Vec<dto::Service>, dto::Error> {
    println!("Installing stock database");

    let pool = Arc::new(pool.clone());
    let repo = Arc::new(features::services::repo::Repo::new(pool));
    let logic = Arc::new(features::services::logic::Logic::new(repo));
    let mut arr = Vec::<dto::Service>::new();

    for i in 1..=count {
        let service = dto::Service::new(None, Some(format!("Service {}", i)));
        let created = logic.create(service.clone()).await?;
        arr.push(created);
    }

    Ok(arr)
}
